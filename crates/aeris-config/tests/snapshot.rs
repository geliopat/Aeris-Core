use aeris_config::{ConfigDocument, ProfileStore, StoreError, UserRules};
use std::sync::{Arc, Barrier};
use std::thread;

fn document(text: &str) -> ConfigDocument {
    ConfigDocument::parse(text.to_owned()).expect("document")
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
    let store = Arc::new(
        ProfileStore::new(document("mode: rule"), UserRules::default()).expect("store"),
    );
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
