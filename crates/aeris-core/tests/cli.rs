use std::process::Command;

#[test]
fn version_is_reported() {
    let output = Command::new(env!("CARGO_BIN_EXE_aeris-core"))
        .arg("version")
        .output()
        .expect("run CLI");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        concat!("Aeris Core ", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn unsupported_command_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_aeris-core"))
        .args(["vpn", "start"])
        .output()
        .expect("run CLI");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn configuration_check_and_test_alias_work() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../aeris-config/tests/fixtures/basic.yaml");
    for subcommand in ["check", "test"] {
        let output = Command::new(env!("CARGO_BIN_EXE_aeris-core"))
            .args(["config", subcommand])
            .arg(&fixture)
            .output()
            .expect("run CLI");
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("NOT verified"));
    }
}

#[test]
fn unreadable_configuration_returns_error_without_path() {
    let secret_path = "nonexistent-aeris-fixture-private-token-123.yaml";
    let output = Command::new(env!("CARGO_BIN_EXE_aeris-core"))
        .args(["config", "test", secret_path])
        .output()
        .expect("run CLI");
    assert_eq!(output.status.code(), Some(1));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(secret_path));
}
