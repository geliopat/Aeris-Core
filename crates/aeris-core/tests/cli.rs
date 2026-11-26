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
