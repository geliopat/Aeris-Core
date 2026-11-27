use aeris_config::{
    ConfigDocument, Error, MAX_INPUT_BYTES, ProfileStore, StoreError, UserRule, UserRules,
};
use std::sync::Arc;

fn parse(source: &str) -> ConfigDocument {
    ConfigDocument::parse(source.to_owned()).expect("parse fixture")
}

#[test]
fn preserves_source_and_checks_synthetic_fixture() {
    let source = include_str!("fixtures/basic.yaml");
    let document = parse(source);
    assert_eq!(document.original(), source);
    let report = document.check().expect("structural check");
    assert_eq!(report.unchecked_top_level_fields, 0);
    assert_eq!(report.rules_after_match, 0);
}

#[test]
fn rejects_invalid_yaml() {
    for source in [
        "mode: [",
        "mode: rule\nmode: global",
        "mode: rule\n---\nmode: direct",
        "mode: !include file.yaml",
    ] {
        assert!(ConfigDocument::parse(source.to_owned()).is_err());
    }
}

#[test]
fn rejects_invalid_root_and_large_input() {
    for source in ["", "{}", "[]", "123"] {
        assert!(ConfigDocument::parse(source.to_owned()).is_err());
    }
    assert!(matches!(
        ConfigDocument::parse(" ".repeat(MAX_INPUT_BYTES + 1)),
        Err(Error::TooLarge)
    ));
}

#[test]
fn rejects_bad_types_and_ports() {
    for source in [
        "mixed-port: 65536",
        "mode: invalid",
        "tun: []",
        "dns: {nameserver: 123}",
        "proxies: [{name: A, type: vless, server: proxy.example, port: 0}]",
    ] {
        assert!(parse(source).check().is_err());
    }
}

#[test]
fn accepts_hosts_scalar_list_alias_and_lan_shapes_without_rewriting_source() {
    let source = include_str!("fixtures/hosts.yaml");
    let document = parse(source);
    let report = document.check().expect("host mapping shape");
    assert_eq!(report.unchecked_top_level_fields, 0);
    assert_eq!(document.original().as_bytes(), source.as_bytes());
}

#[test]
fn accepts_omitted_null_and_empty_hosts_mappings() {
    for source in ["mode: rule", "hosts: null", "hosts: {}"] {
        let document = parse(source);
        let report = document.check().expect("optional hosts mapping");
        assert_eq!(report.unchecked_top_level_fields, 0);
        assert_eq!(document.original(), source);
    }
}

#[test]
fn rejects_invalid_hosts_shapes_without_echoing_keys_or_values() {
    for (hosts, path) in [
        ("['private.example']", "hosts"),
        ("private.example", "hosts"),
        ("{'  ': 192.0.2.1}", "hosts.key"),
        ("{private.example: false}", "hosts[]"),
        ("{private.example: null}", "hosts[]"),
        ("{private.example: ''}", "hosts[]"),
        ("{private.example: []}", "hosts[]"),
        (
            "{private.example: {alias: private-target.example}}",
            "hosts[]",
        ),
        ("{private.example: [192.0.2.1, 53]}", "hosts[][]"),
        ("{private.example: [null]}", "hosts[][]"),
        ("{private.example: ['   ']}", "hosts[][]"),
        ("{private.example: [[private-target.example]]}", "hosts[][]"),
    ] {
        let source = format!("hosts: {hosts}");
        let error = parse(&source).check().expect_err("invalid hosts shape");
        assert_eq!(error, Error::Shape(path.to_owned()));
        assert_eq!(error.to_string(), format!("invalid structure at {path}"));
    }
}

#[test]
fn invalid_hosts_refresh_preserves_the_last_good_snapshot() {
    let source = include_str!("fixtures/hosts.yaml");
    let store = ProfileStore::new(parse(source), UserRules::default()).expect("profile store");
    let prior = store.current().expect("current snapshot");
    let invalid = "hosts: {private.example: []}";
    assert!(matches!(
        store.refresh_provider(prior.generation(), parse(invalid)),
        Err(StoreError::Invalid(Error::Shape(path))) if path == "hosts[]"
    ));
    let current = store.current().expect("last good snapshot");
    assert!(Arc::ptr_eq(&prior, &current));
    assert_eq!(current.original(), source);
}

#[test]
fn checks_fallback_filter_shapes_and_preserves_source() {
    let source = include_str!("fixtures/fallback-filter.yaml");
    let document = parse(source);
    let report = document
        .check()
        .expect("fallback filter shape and CIDR checks");
    assert_eq!(report.unchecked_top_level_fields, 0);
    assert_eq!(document.original().as_bytes(), source.as_bytes());
}

#[test]
fn accepts_omitted_null_and_empty_fallback_filters() {
    for source in [
        "dns: {}",
        "dns: {fallback-filter: null}",
        "dns: {fallback-filter: {}}",
        "dns: {fallback: ['https://backup.example/dns-query'], fallback-filter: {geoip: null, geoip-code: null, ipcidr: null, domain: null, geosite: null}}",
        "dns: {fallback-filter: {geoip-code: '', ipcidr: [], domain: [], geosite: []}}",
        "dns: {fallback-filter: {domain: [''], geosite: [''], provider-extra: {shape: null}}}",
    ] {
        let document = parse(source);
        document.check().expect("optional fallback filter shape");
        assert_eq!(document.original(), source);
    }
}

#[test]
fn rejects_invalid_fallback_filter_shapes_without_echoing_values() {
    for (filter, path) in [
        (
            "['https://private.example/probe?token=fixture']",
            "dns.fallback-filter",
        ),
        ("{geoip: 'false'}", "dns.fallback-filter.geoip"),
        ("{geoip-code: 53}", "dns.fallback-filter.geoip-code"),
        ("{ipcidr: false}", "dns.fallback-filter.ipcidr"),
        ("{ipcidr: [null]}", "dns.fallback-filter.ipcidr[]"),
        ("{domain: private.example}", "dns.fallback-filter.domain"),
        ("{domain: [53]}", "dns.fallback-filter.domain[]"),
        ("{geosite: {name: private}}", "dns.fallback-filter.geosite"),
        (
            "{geosite: [[private.example]]}",
            "dns.fallback-filter.geosite[]",
        ),
    ] {
        let source = format!("dns: {{fallback-filter: {filter}}}");
        let error = parse(&source)
            .check()
            .expect_err("invalid fallback filter shape");
        assert_eq!(error, Error::Shape(path.to_owned()));
        assert_eq!(error.to_string(), format!("invalid structure at {path}"));
    }
}

#[test]
fn validates_fallback_cidr_prefixes_only_with_configured_resolvers() {
    let dormant = "fallback-filter: {ipcidr: ['private.example/24']}";
    for source in [
        format!("dns: {{{dormant}}}"),
        format!("dns: {{fallback: [], {dormant}}}"),
    ] {
        let document = parse(&source);
        document
            .check()
            .expect("dormant filter preserves CIDR strings");
        assert_eq!(document.original(), source);
    }
    for ipcidr in ["private.example/24", "192.0.2.7/33", "2001:db8::7/129"] {
        let source = format!(
            "dns: {{fallback: ['https://backup.example/dns-query'], fallback-filter: {{ipcidr: ['{ipcidr}']}}}}"
        );
        let error = parse(&source).check().expect_err("invalid configured CIDR");
        assert_eq!(
            error,
            Error::Shape("dns.fallback-filter.ipcidr[]".to_owned())
        );
        assert_eq!(
            error.to_string(),
            "invalid structure at dns.fallback-filter.ipcidr[]"
        );
    }
}

#[test]
fn rejects_invalid_fallback_refresh_and_retains_the_last_good_snapshot() {
    let source = include_str!("fixtures/fallback-filter.yaml");
    let store = ProfileStore::new(parse(source), UserRules::default()).expect("profile store");
    let prior = store.current().expect("current snapshot");
    let invalid = source.replace("192.0.2.17/24", "192.0.2.17/33");
    assert!(matches!(
        store.refresh_provider(prior.generation(), parse(&invalid)),
        Err(StoreError::Invalid(Error::Shape(path))) if path == "dns.fallback-filter.ipcidr[]"
    ));
    let current = store.current().expect("last good snapshot");
    assert!(Arc::ptr_eq(&prior, &current));
    assert_eq!(current.original(), source);
}

#[test]
fn accepts_nameserver_policy_scalars_and_lists_without_rewriting_source() {
    let source = include_str!("fixtures/nameserver-policy.yaml");
    let document = parse(source);
    let report = document.check().expect("policy shape");
    assert_eq!(report.unchecked_top_level_fields, 0);
    assert_eq!(document.original().as_bytes(), source.as_bytes());
}

#[test]
fn accepts_empty_optional_nameserver_policy_shapes() {
    for source in [
        "dns: {}",
        "dns: {nameserver-policy: null}",
        "dns: {nameserver-policy: {}}",
        "dns: {nameserver-policy: {disabled.example: []}}",
    ] {
        let document = parse(source);
        document.check().expect("empty policy shape");
        assert_eq!(document.original(), source);
    }
}

#[test]
fn rejects_invalid_nameserver_policy_shapes_without_echoing_selectors_or_values() {
    for (policy, path) in [
        (
            "['https://private.example/probe?token=fixture']",
            "dns.nameserver-policy",
        ),
        ("false", "dns.nameserver-policy"),
        (
            "{\"   \": tls://resolver.example}",
            "dns.nameserver-policy.key",
        ),
        ("{private.example: null}", "dns.nameserver-policy[]"),
        ("{private.example: 53}", "dns.nameserver-policy[]"),
        ("{private.example: '  '}", "dns.nameserver-policy[]"),
        (
            "{private.example: {url: 'tls://private.example'}}",
            "dns.nameserver-policy[]",
        ),
        (
            "{private.example: [tls://resolver.example, false]}",
            "dns.nameserver-policy[][]",
        ),
        ("{private.example: [null]}", "dns.nameserver-policy[][]"),
        ("{private.example: ['']}", "dns.nameserver-policy[][]"),
        (
            "{private.example: [[tls://resolver.example]]}",
            "dns.nameserver-policy[][]",
        ),
    ] {
        let source = format!("dns: {{nameserver-policy: {policy}}}");
        let error = parse(&source).check().expect_err("invalid policy shape");
        assert_eq!(error, Error::Shape(path.to_owned()));
        assert_eq!(error.to_string(), format!("invalid structure at {path}"));
    }
}

#[test]
fn invalid_nameserver_policy_refresh_preserves_the_last_good_snapshot() {
    let source = include_str!("fixtures/nameserver-policy.yaml");
    let store = ProfileStore::new(parse(source), UserRules::default()).expect("profile store");
    let prior = store.current().expect("current snapshot");
    let invalid = "dns: {nameserver-policy: {private.example: [null]}}";
    assert!(matches!(
        store.refresh_provider(prior.generation(), parse(invalid)),
        Err(StoreError::Invalid(Error::Shape(field))) if field == "dns.nameserver-policy[][]"
    ));
    let current = store.current().expect("last good snapshot");
    assert!(Arc::ptr_eq(&prior, &current));
    assert_eq!(current.original(), source);
}

#[test]
fn checks_synthetic_group_health_profile_and_preserves_source() {
    let source = include_str!("fixtures/group-health.yaml");
    let document = parse(source);
    let report = document.check().expect("health field shape and references");
    assert_eq!(report.unchecked_top_level_fields, 0);
    assert_eq!(report.unchecked_rule_matchers, 0);
    assert_eq!(document.original().as_bytes(), source.as_bytes());
    assert_eq!(
        document
            .preview_rules(&UserRules::default())
            .collect::<Vec<_>>(),
        ["DOMAIN,service.example,Selected", "MATCH,Measured"]
    );
}

#[test]
fn rejects_invalid_group_health_shapes_without_echoing_values() {
    for (field, value) in [
        ("url", "['https://secret.example/probe?token=fixture']"),
        ("interval", "'300'"),
        ("interval", "-1"),
        ("interval", "9223372036854775808"),
        ("timeout", "true"),
        ("timeout", "0.5"),
        ("timeout", "-1"),
        ("lazy", "'false'"),
        ("tolerance", "-1"),
        ("tolerance", "65536"),
        ("tolerance", "'50'"),
    ] {
        let source = format!(
            "proxy-groups: [{{name: Measured, type: url-test, proxies: [DIRECT], {field}: {value}}}]"
        );
        let error = parse(&source).check().expect_err("invalid health field");
        assert_eq!(error, Error::Shape(format!("proxy-groups[].{field}")));
        assert_eq!(
            error.to_string(),
            format!("invalid structure at proxy-groups[].{field}")
        );
    }
}

#[test]
fn health_fields_keep_zero_and_empty_values_and_scope_tolerance() {
    let source = "proxy-groups:\n  - {name: One, type: url-test, proxies: [DIRECT], url: '', interval: 0, timeout: 0, lazy: false, tolerance: 0}\n  - {name: Two, type: url-test, proxies: [One], tolerance: 65535}\n  - {name: Three, type: load-balance, proxies: [Two], tolerance: [opaque], strategy: round-robin}";
    parse(source).check().expect("supported boundary shapes");
}

#[test]
fn invalid_group_health_refresh_preserves_the_previous_snapshot() {
    let source = include_str!("fixtures/group-health.yaml");
    let store = ProfileStore::new(parse(source), UserRules::default()).expect("profile store");
    let prior = store.current().expect("current snapshot");
    let invalid = source.replace("timeout: 4200", "timeout: invalid");
    assert!(matches!(
        store.refresh_provider(prior.generation(), parse(&invalid)),
        Err(StoreError::Invalid(Error::Shape(field))) if field == "proxy-groups[].timeout"
    ));
    assert!(Arc::ptr_eq(
        &prior,
        &store.current().expect("preserved snapshot")
    ));
}

#[test]
fn rejects_dangling_and_cyclic_dependencies() {
    let missing = "proxy-groups: [{name: A, type: select, proxies: [B]}]";
    assert!(matches!(parse(missing).check(), Err(Error::Reference(_))));
    let cyclic = "proxy-groups: [{name: A, type: select, proxies: [B]}, {name: B, type: select, proxies: [A]}]";
    assert_eq!(parse(cyclic).check(), Err(Error::Cycle));
    let self_cycle = "proxy-groups: [{name: A, type: select, proxies: [A]}]";
    assert_eq!(parse(self_cycle).check(), Err(Error::Cycle));
}

#[test]
fn duplicate_names_and_rule_references_fail() {
    let duplicate = "proxy-groups: [{name: A, type: select}, {name: A, type: select}]";
    assert_eq!(parse(duplicate).check(), Err(Error::DuplicateName));
    for source in [
        "rules: ['MATCH,Missing']",
        "rules: ['RULE-SET,Missing,DIRECT']",
        "proxy-groups: [{name: A, type: select, use: [Missing]}]",
    ] {
        assert!(matches!(parse(source).check(), Err(Error::Reference(_))));
    }
}

#[test]
fn accepts_explicit_and_implicit_global_groups() {
    let implicit = concat!(
        "proxies:\n",
        "  - {name: A, type: socks5, server: proxy.example, port: 1080, dialer-proxy: GLOBAL}\n",
        "rules: ['MATCH,GLOBAL']",
    );
    let explicit = concat!(
        "proxy-groups:\n",
        "  - {name: A, type: select, proxies: [GLOBAL]}\n",
        "  - {name: GLOBAL, type: select, proxies: [DIRECT]}\n",
        "rules: ['MATCH,GLOBAL']",
    );
    for source in [implicit, explicit] {
        let document = parse(source);
        document.check().expect("valid GLOBAL group reference");
        assert_eq!(document.original(), source);
    }
}

#[test]
fn rejects_implicit_global_as_a_group_member() {
    let source = "proxy-groups: [{name: A, type: select, proxies: [GLOBAL]}]";
    assert_eq!(
        parse(source).check(),
        Err(Error::Reference("proxy-groups[].proxies[]".to_owned()))
    );
}

#[test]
fn global_group_does_not_allow_duplicate_or_reserved_names() {
    for source in [
        "proxies: [{name: GLOBAL, type: socks5, server: proxy.example, port: 1080}]",
        concat!(
            "proxy-groups:\n",
            "  - {name: GLOBAL, type: select, proxies: [DIRECT]}\n",
            "  - {name: GLOBAL, type: select, proxies: [REJECT]}\n",
        ),
        "proxy-groups: [{name: DIRECT, type: select, proxies: [REJECT]}]",
    ] {
        assert_eq!(parse(source).check(), Err(Error::DuplicateName), "{source}");
    }
}

#[test]
fn rejects_cycles_through_an_explicit_global_group() {
    let self_cycle = "proxy-groups: [{name: GLOBAL, type: select, proxies: [GLOBAL]}]";
    let mutual_cycle = concat!(
        "proxy-groups:\n",
        "  - {name: A, type: select, proxies: [GLOBAL]}\n",
        "  - {name: GLOBAL, type: select, proxies: [A]}\n",
    );
    for source in [self_cycle, mutual_cycle] {
        assert_eq!(parse(source).check(), Err(Error::Cycle), "{source}");
    }
}

#[test]
fn reports_unchecked_fields_and_unreachable_rules() {
    let report = parse("mode: rule\ncustom: true\nrules: ['MATCH,DIRECT', 'AND,opaque']")
        .check()
        .expect("structural check");
    assert_eq!(report.unchecked_top_level_fields, 1);
    assert_eq!(report.unchecked_rule_matchers, 1);
    assert_eq!(report.rules_after_match, 1);
}

#[test]
fn accepts_mihomo_rule_options_without_rewriting_rules() {
    let source = concat!(
        "rule-providers: {example: {}}\nrules:\n",
        "  - IP-CIDR, 192.0.2.0/24, DIRECT, no-resolve\n",
        "  - ip-cidr6,2001:db8::/32,DIRECT,src,no-resolve\n",
        "  - GEOIP,US,DIRECT,src\n",
        "  - IP-ASN,64496,DIRECT,no-resolve,src\n",
        "  - RULE-SET,example,DIRECT,no-resolve\n",
        "  - match,DIRECT",
    );
    let document = parse(source);
    let user = UserRules::parse("['IP-CIDR,198.51.100.0/24,REJECT,src']").expect("user rules");
    let report = document
        .check_with_user_rules(Some(&user))
        .expect("structural rule check");
    assert_eq!(report.unchecked_rule_matchers, 0);
    assert_eq!(document.original(), source);
    assert_eq!(
        document.preview_rules(&user).nth(2),
        Some("ip-cidr6,2001:db8::/32,DIRECT,src,no-resolve")
    );
}

#[test]
fn rejects_unrecognized_or_misplaced_options_for_checked_rules() {
    for rule in [
        "DOMAIN,example.org,DIRECT,unexpected,garbage",
        "domain,example.org,DIRECT,no-resolve",
        "PROCESS-NAME,process.exe,DIRECT,src",
        "GEOSITE,example,DIRECT,no-resolve",
        "IP-CIDR,192.0.2.0/24,DIRECT,",
        "IP-ASN,64496,DIRECT,no-resolve,unknown",
        "IP-CIDR6,2001:db8::/32,DIRECT,NO-RESOLVE",
        "IP-CIDR,192.0.2.0/24,DIRECT,\tno-resolve",
        "IP-CIDR,192.0.2.0/24,DIRECT,\u{00a0}src",
        "RULE-SET,example,DIRECT,unknown",
    ] {
        let source = format!("rule-providers: {{example: {{}}}}\nrules: ['{rule}']");
        let expected = Error::Shape("rules[].options".to_owned());
        assert_eq!(parse(&source).check(), Err(expected.clone()), "{rule}");
        let user = UserRules::parse(&format!("['{rule}']")).expect("user rules");
        let document = parse("rule-providers: {example: {}}\nrules: ['MATCH,DIRECT']");
        assert_eq!(document.check_with_user_rules(Some(&user)), Err(expected));
    }
}

#[test]
fn cidr_aliases_accept_both_address_families_and_preserve_host_bits() {
    let source = concat!(
        "rules:\n",
        "  - IP-CIDR6,192.0.2.7/24,DIRECT\n",
        "  - IP-CIDR,2001:db8::7/32,DIRECT\n",
        "  - IP-CIDR,0.0.0.0/0,DIRECT\n",
        "  - IP-CIDR6,::ffff:192.0.2.7/128,DIRECT",
    );
    let document = parse(source);
    let report = document.check().expect("structural CIDR check");
    assert_eq!(report.unchecked_rule_matchers, 0);
    assert_eq!(document.original(), source);
}

#[test]
fn rejects_invalid_ip_prefixes_without_echoing_payload() {
    for rule in [
        "IP-CIDR,secret-value/24,DIRECT",
        "IP-CIDR,192.0.2.1,DIRECT",
        "IP-CIDR,192.0.2.1/33,DIRECT",
        "IP-CIDR6,2001:db8::1/129,DIRECT",
        "IP-CIDR6,fe80::1%eth0/64,DIRECT",
        "IP-CIDR,192.0.2.1/024,DIRECT",
        "IP-CIDR,192.0.2.1/+24,DIRECT",
        "IP-CIDR6,::/-1,DIRECT",
        "IP-CIDR,\t192.0.2.1/24,DIRECT",
        "IP-CIDR6,\u{00a0}2001:db8::/32,DIRECT",
    ] {
        let source = format!("rules: ['{rule}']");
        let expected = Error::Shape("rules[].IP-CIDR".to_owned());
        assert_eq!(parse(&source).check(), Err(expected.clone()), "{rule}");
        let user = UserRules::parse(&format!("['{rule}']")).expect("user rules");
        let document = parse("rules: ['MATCH,DIRECT']");
        assert_eq!(document.check_with_user_rules(Some(&user)), Err(expected));
    }
}

#[test]
fn user_rules_precede_provider_without_mutation() {
    let source = "rules: ['DOMAIN,example.org,DIRECT', 'MATCH,DIRECT']";
    let document = parse(source);
    let user = UserRules::parse("['DOMAIN,example.org,REJECT']").expect("user rules");
    let rules: Vec<_> = document.preview_rules(&user).collect();
    assert_eq!(
        rules,
        [
            "DOMAIN,example.org,REJECT",
            "DOMAIN,example.org,DIRECT",
            "MATCH,DIRECT",
        ]
    );
    assert_eq!(document.original(), source);
}

#[test]
fn disabled_user_rules_do_not_affect_validation_or_order() {
    let document = parse("rules: ['DOMAIN,provider.example,DIRECT', 'MATCH,DIRECT']");
    let user = UserRules::from_entries(vec![
        UserRule::new("MATCH,DIRECT".to_owned(), false).expect("entry"),
        UserRule::new("DOMAIN,user.example,REJECT".to_owned(), true).expect("entry"),
        UserRule::new("DOMAIN,disabled.example,Unknown".to_owned(), false).expect("entry"),
        UserRule::new(
            "AND,((DOMAIN,unchecked.example),(NETWORK,TCP)),DIRECT".to_owned(),
            false,
        )
        .expect("entry"),
    ])
    .expect("user rules");
    let report = document
        .check_with_user_rules(Some(&user))
        .expect("structural check");
    assert_eq!(report.user_rules_after_match, 0);
    assert_eq!(report.rules_after_match, 0);
    assert_eq!(report.unchecked_rule_matchers, 0);
    assert_eq!(
        document.preview_rules(&user).collect::<Vec<_>>(),
        [
            "DOMAIN,user.example,REJECT",
            "DOMAIN,provider.example,DIRECT",
            "MATCH,DIRECT",
        ]
    );
    assert_eq!(user.entries().len(), 4);
    assert!(!user.entries()[0].enabled());
    assert_eq!(user.entries()[0].text(), "MATCH,DIRECT");
}

#[test]
fn parses_legacy_user_rules_as_enabled_without_accepting_a_new_yaml_format() {
    let user =
        UserRules::parse("['DOMAIN,user.example,DIRECT', 'MATCH,DIRECT']").expect("legacy rules");
    assert!(user.entries().iter().all(UserRule::enabled));
    assert!(UserRules::parse("[{text: 'MATCH,DIRECT', enabled: false}]").is_err());
    assert!(UserRules::parse("['missing-separator']").is_err());
    assert!(UserRule::new("missing-separator".to_owned(), false).is_err());
}

#[test]
fn typed_user_rules_enforce_entry_and_aggregate_byte_limits() {
    fn text(bytes: usize) -> String {
        let mut text = "s".repeat(bytes - 1);
        text.push(',');
        text
    }

    assert!(matches!(
        UserRule::new(text(MAX_INPUT_BYTES + 1), false),
        Err(Error::TooLarge)
    ));
    let entry = UserRule::new(text(MAX_INPUT_BYTES), false).expect("entry at byte limit");
    let exact = UserRules::from_entries(vec![entry.clone(), entry.clone()]);
    assert!(exact.is_ok());
    drop(exact);
    let extra = UserRule::new(",".to_owned(), false).expect("disabled entry");
    assert!(matches!(
        UserRules::from_entries(vec![entry.clone(), entry, extra]),
        Err(Error::TooLarge)
    ));
}

#[test]
fn typed_user_rules_enforce_entry_count_without_excluding_disabled_rules() {
    let max_entries = serde_saphyr::Budget::default().max_events;
    let disabled = UserRule::new("MATCH,DIRECT".to_owned(), false).expect("disabled entry");
    assert!(UserRules::from_entries(vec![disabled.clone(); max_entries]).is_ok());
    assert!(matches!(
        UserRules::from_entries(vec![disabled; max_entries + 1]),
        Err(Error::TooLarge)
    ));
}

#[test]
fn legacy_user_rules_keep_the_existing_decoded_scalar_budget_for_aliases() {
    let suffix = "s".repeat(MAX_INPUT_BYTES / 2);
    let source = format!("[&rule 'DOMAIN,{suffix},DIRECT', *rule, *rule]");
    assert!(source.len() < MAX_INPUT_BYTES);
    let user = UserRules::parse(&source).expect("aliases fit the decoded scalar budget");
    assert_eq!(user.entries().len(), 3);
    assert!(user.entries().iter().all(UserRule::enabled));
    assert!(
        user.entries()
            .iter()
            .map(|entry| entry.text().len())
            .sum::<usize>()
            > MAX_INPUT_BYTES
    );
}

#[test]
fn diagnostics_do_not_echo_secrets() {
    let result = parse("proxies: [{name: A, type: secret-value}]").check();
    let error = result.expect_err("invalid protocol");
    assert!(!error.to_string().contains("secret-value"));
}
