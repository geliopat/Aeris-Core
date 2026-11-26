use aeris_config::{ConfigDocument, ProfileStore, StoreError, UserRule, UserRules};
use std::sync::{Arc, Barrier};
use std::thread;

fn document(text: &str) -> ConfigDocument {
    ConfigDocument::parse(text.to_owned()).expect("document")
}

fn user_rule(text: &str, enabled: bool) -> UserRule {
    UserRule::new(text.to_owned(), enabled).expect("user rule")
}

#[test]
fn rejects_bad_candidate_and_preserves_old_snapshot() {
    let store = ProfileStore::new(document("mode: rule"), UserRules::default()).expect("store");
    let old = store.current().expect("current");
    let invalid = store.replace(1, document("mode: invalid"), UserRules::default());
    assert!(matches!(invalid, Err(StoreError::Invalid(_))));
    assert!(Arc::ptr_eq(&old, &store.current().expect("current")));

    let updated = store
        .replace(1, document("mode: global"), UserRules::default())
        .expect("updated");
    assert_eq!(updated.generation(), 2);
    assert_eq!(old.original(), "mode: rule");
    assert_eq!(updated.original(), "mode: global");
    assert!(matches!(
        store.replace(1, document("mode: direct"), UserRules::default()),
        Err(StoreError::Conflict)
    ));

    let restored = store.rollback(2).expect("rollback");
    assert_eq!(restored.generation(), 3);
    assert_eq!(restored.original(), old.original());
    assert!(matches!(store.rollback(3), Err(StoreError::NoPrevious)));
}

#[test]
fn user_rules_are_checked_against_candidate_references() {
    let store = ProfileStore::new(
        document("rules: ['MATCH,DIRECT']"),
        UserRules::parse("['DOMAIN,example.org,REJECT']").expect("user rules"),
    )
    .expect("store");
    let current = store.current().expect("current");
    assert_eq!(
        current.rules().collect::<Vec<_>>(),
        ["DOMAIN,example.org,REJECT", "MATCH,DIRECT"]
    );
    let bad = UserRules::parse("['DOMAIN,example.org,Unknown']").expect("user rules");
    assert!(matches!(
        store.replace(1, document("mode: rule"), bad),
        Err(StoreError::Invalid(_))
    ));
    assert!(Arc::ptr_eq(&current, &store.current().expect("current")));
}

#[test]
fn only_one_concurrent_writer_can_publish_same_generation() {
    let store =
        Arc::new(ProfileStore::new(document("mode: rule"), UserRules::default()).expect("store"));
    let barrier = Arc::new(Barrier::new(3));
    let writers: Vec<_> = ["mode: direct", "mode: global"]
        .into_iter()
        .map(|mode| {
            let store = Arc::clone(&store);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                store.replace(1, document(mode), UserRules::default())
            })
        })
        .collect();
    barrier.wait();
    let outcomes: Vec<_> = writers
        .into_iter()
        .map(|worker| worker.join().expect("writer"))
        .collect();
    assert_eq!(outcomes.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        outcomes
            .iter()
            .filter(|result| matches!(result, Err(StoreError::Conflict)))
            .count(),
        1
    );
    assert_eq!(store.current().expect("current").generation(), 2);
}

#[test]
fn edit_and_reorder_user_rules_without_replacing_provider_document() {
    let source =
        "# Keep provider text\r\nrules: ['DOMAIN,provider.example,DIRECT', 'MATCH,DIRECT']\r\n";
    let initial = UserRules::from_entries(vec![
        user_rule("DOMAIN,first.example,DIRECT", true),
        user_rule("DOMAIN,second.example,REJECT", true),
        user_rule("DOMAIN,removed.example,DIRECT", true),
    ])
    .expect("user rules");
    let store = ProfileStore::new(document(source), initial).expect("store");
    let old = store.current().expect("current");
    let mut entries = old.user_rules().entries().to_vec();
    entries.pop();
    entries[0] = user_rule("DOMAIN,first.example,DIRECT", false);
    entries[1] = user_rule("DOMAIN,edited.example,REJECT", true);
    entries.insert(0, user_rule("DOMAIN,added.example,DIRECT", true));
    entries.swap(0, 2);
    let edited = store
        .update_user_rules(
            old.generation(),
            UserRules::from_entries(entries).expect("user rules"),
        )
        .expect("edits published");

    assert_eq!(edited.generation(), 2);
    assert_eq!(edited.original(), source);
    assert_eq!(edited.user_rules().entries().len(), 3);
    assert!(!edited.user_rules().entries()[1].enabled());
    assert_eq!(
        edited.rules().collect::<Vec<_>>(),
        [
            "DOMAIN,edited.example,REJECT",
            "DOMAIN,added.example,DIRECT",
            "DOMAIN,provider.example,DIRECT",
            "MATCH,DIRECT",
        ]
    );
    assert_eq!(
        old.user_rules().entries()[1].text(),
        "DOMAIN,second.example,REJECT"
    );

    let invalid = UserRules::from_entries(vec![user_rule("DOMAIN,bad.example,Missing", true)])
        .expect("user rules");
    assert!(matches!(
        store.update_user_rules(2, invalid),
        Err(StoreError::Invalid(_))
    ));
    assert!(Arc::ptr_eq(
        &edited,
        &store.current().expect("still current")
    ));
    let restored = store.rollback(2).expect("restore previous user rules");
    assert_eq!(restored.original(), source);
    assert_eq!(
        restored.rules().collect::<Vec<_>>(),
        old.rules().collect::<Vec<_>>()
    );
}

#[test]
fn provider_refresh_retains_rules_and_rejects_broken_active_references() {
    let user_text = "DOMAIN,user.example,Preferred";
    let initial_provider = "proxy-groups: [{name: Preferred, type: select, proxies: [DIRECT]}]\nrules: ['MATCH,DIRECT']";
    let updated_provider = "proxy-groups: [{name: Preferred, type: select, proxies: [DIRECT]}]\nrules: ['MATCH,REJECT']";
    let provider_without_target = "rules: ['MATCH,REJECT']";
    let initial = UserRules::from_entries(vec![user_rule(user_text, true)]).expect("user rules");
    let store = ProfileStore::new(document(initial_provider), initial).expect("store");
    let refreshed = store
        .refresh_provider(1, document(updated_provider))
        .expect("provider refreshed");
    assert_eq!(refreshed.generation(), 2);
    assert_eq!(refreshed.original(), updated_provider);
    assert_eq!(
        refreshed.rules().collect::<Vec<_>>(),
        [user_text, "MATCH,REJECT"]
    );

    assert!(matches!(
        store.refresh_provider(1, document(provider_without_target)),
        Err(StoreError::Conflict)
    ));
    assert!(matches!(
        store.refresh_provider(2, document(provider_without_target)),
        Err(StoreError::Invalid(_))
    ));
    assert!(Arc::ptr_eq(
        &refreshed,
        &store.current().expect("last good")
    ));
    let restored = store.rollback(2).expect("old provider remains restorable");
    assert_eq!(restored.original(), initial_provider);
    assert_eq!(
        restored.rules().collect::<Vec<_>>(),
        [user_text, "MATCH,DIRECT"]
    );

    let disabled = UserRules::from_entries(vec![user_rule(user_text, false)]).expect("user rules");
    store.update_user_rules(3, disabled).expect("rule disabled");
    let recovered = store
        .refresh_provider(4, document(provider_without_target))
        .expect("provider refreshed after disabling broken reference");
    assert_eq!(recovered.original(), provider_without_target);
    assert_eq!(recovered.rules().collect::<Vec<_>>(), ["MATCH,REJECT"]);
    assert_eq!(recovered.user_rules().entries()[0].text(), user_text);
    assert!(!recovered.user_rules().entries()[0].enabled());

    let reopened = UserRules::from_entries(vec![user_rule(user_text, true)]).expect("user rules");
    assert!(matches!(
        store.update_user_rules(5, reopened),
        Err(StoreError::Invalid(_))
    ));
    assert!(Arc::ptr_eq(
        &recovered,
        &store.current().expect("last good")
    ));
}

#[test]
fn concurrent_user_edit_and_provider_refresh_cannot_lose_a_change_silently() {
    let store = ProfileStore::new(document("mode: direct"), UserRules::default()).expect("store");
    let barrier = Barrier::new(3);
    let (edit, refresh) = thread::scope(|scope| {
        let edit = scope.spawn(|| {
            let entries =
                UserRules::from_entries(vec![user_rule("MATCH,REJECT", true)]).expect("user rules");
            barrier.wait();
            store.update_user_rules(1, entries)
        });
        let refresh = scope.spawn(|| {
            let updated = document("mode: global");
            barrier.wait();
            store.refresh_provider(1, updated)
        });
        barrier.wait();
        (
            edit.join().expect("editor"),
            refresh.join().expect("refresher"),
        )
    });

    let winner = match (edit, refresh) {
        (Ok(edited), Err(StoreError::Conflict)) => {
            assert_eq!(edited.original(), "mode: direct");
            assert_eq!(edited.rules().collect::<Vec<_>>(), ["MATCH,REJECT"]);
            edited
        }
        (Err(StoreError::Conflict), Ok(refreshed)) => {
            assert_eq!(refreshed.original(), "mode: global");
            assert!(refreshed.user_rules().entries().is_empty());
            refreshed
        }
        _ => panic!("exactly one operation should publish, the other should report conflict"),
    };
    assert_eq!(winner.generation(), 2);
    assert!(Arc::ptr_eq(&winner, &store.current().expect("current")));
}
