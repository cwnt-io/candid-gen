use anyhow::{anyhow, Context, Result};
use candid_gen::functions::build_wasm32::build_wasm32;
use candid_gen::functions::run_command::run_command;
use candid_gen::types::canisters::Canisters;
use candid_gen::types::dfx_cfg::{DfxCfg, RustCanisterCfg};
use candid_gen::BUILD_OUTPUT_DIR;
use once_cell::sync::Lazy;
use serial_test::serial;
use std::collections::HashMap;
use std::env::{current_dir, set_current_dir};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::Mutex;

static MOCK_PROJECT_DIR: Lazy<Mutex<PathBuf>> =
    Lazy::new(|| Mutex::new(current_dir().unwrap().join("tests/mock_project")));

#[test]
// #[serial]
fn test_mock_project_dir() -> Result<()> {
    let mock_project_dir = MOCK_PROJECT_DIR.lock().unwrap();
    let cargo_toml = mock_project_dir.join("Cargo.toml");
    let dfx_json = mock_project_dir.join("dfx.json");
    assert!(mock_project_dir.is_dir());
    assert!(cargo_toml.is_file());
    assert!(dfx_json.is_file());
    Ok(())
}

#[test]
// #[serial]
fn test_build_wasm32_success() -> Result<()> {
    let project_root = MOCK_PROJECT_DIR.lock().unwrap();
    set_current_dir(&*project_root)?;
    let build_output_path = project_root.join(BUILD_OUTPUT_DIR);
    let dfx_path = project_root.join("dfx.json");
    let dfx_json = read_to_string(&dfx_path).context("Failed to read dfx.json file")?;
    let dfx_cfg: DfxCfg = serde_json::from_str(&dfx_json).unwrap();
    let canisters: Canisters = dfx_cfg.canisters;
    if let Some((canister_name, canister)) = canisters.0.iter().next() {
        let wasm_file = build_output_path.join(format!("{}.wasm", canister_name));
        if let Err(e) = run_command(&format!("rm {}", &wasm_file.display())) {
            eprintln!("{} already deleted: {}", &wasm_file.display(), e);
        };
        build_wasm32(canister)?;
        assert!(wasm_file.exists(), "Build output should exist");
        run_command(&format!("rm {}", &wasm_file.display()))?;
    } else {
        return Err(anyhow!("No rust canister found at {}.", dfx_path.display()));
    }
    Ok(())
}

#[test]
// #[serial]
fn test_build_wasm32_failure() -> Result<()> {
    let project_root = MOCK_PROJECT_DIR.lock().unwrap();
    set_current_dir(&*project_root)?;
    let build_output_path = project_root.join(BUILD_OUTPUT_DIR);

    let canister = RustCanisterCfg {
        package: "nonexistent_canister".to_string(),
        candid: "src/nonexistent_canister/nonexistent_canister.did".to_string(),
        other: HashMap::new(),
    };
    let canister_name = &canister.package;
    let wasm_file = build_output_path.join(format!("{}.wasm", canister_name));
    let build_result = build_wasm32(&canister);
    assert!(!wasm_file.exists(), "Build output should NOT exist");
    assert!(build_result.is_err(), "Build result must be an error");

    Ok(())
}
