use candid_gen::functions::run_command::run_command;

#[test]
fn test_run_command_success() {
    let result = run_command("echo integration test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().trim(), "integration test");
}

#[test]
fn test_run_command_failure() {
    let result = run_command("invalid_command");
    assert!(result.is_err());
}

#[test]
fn test_run_command_rustup() {
    let result = run_command("rustup --version");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("rustup"));
}

#[test]
fn test_run_command_cargo() {
    let result = run_command("cargo --version");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("cargo"));
}

#[test]
fn test_run_command_candid_extractor() {
    let result = run_command("candid-extractor --version");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("candid-extractor"));
}

#[test]
fn test_run_command_target_wasm32() {
    let result = run_command("rustup target list --installed");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("wasm32-unknown-unknown"));
}
