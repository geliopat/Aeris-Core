use aeris_config::{ConfigDocument, Error, MAX_INPUT_BYTES, UserRules};

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
fn reports_unchecked_fields_and_unreachable_rules() {
    let report = parse("mode: rule\ncustom: true\nrules: ['MATCH,DIRECT', 'AND,opaque']")
        .check()
        .expect("structural check");
    assert_eq!(report.unchecked_top_level_fields, 1);
    assert_eq!(report.unchecked_rule_matchers, 1);
    assert_eq!(report.rules_after_match, 1);
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
fn diagnostics_do_not_echo_secrets() {
    let result = parse("proxies: [{name: A, type: secret-value}]").check();
    let error = result.expect_err("invalid protocol");
    assert!(!error.to_string().contains("secret-value"));
}
