use std::{
    collections::HashMap,
    env::{current_dir, set_current_dir},
    fs::{read_to_string, remove_file},
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};

use anyhow::{anyhow, Context, Result};
use candid_gen::{
    functions::{
        build_wasm32::build_wasm32, gen_candid::gen_candid,
        get_candid_path_str::get_candid_path_str, run_command::run_command,
    },
    types::{
        canisters::Canisters,
        dfx_cfg::{DfxCfg, RustCanisterCfg},
    },
    BUILD_OUTPUT_DIR,
};
use candid_parser::{Error, IDLProg};
use once_cell::sync::Lazy;

static MOCK_PROJECT_DIR: Lazy<Mutex<PathBuf>> =
    Lazy::new(|| Mutex::new(current_dir().unwrap().join("tests/mock_project")));

#[test]
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
fn test_gen_candid_success() -> Result<()> {
    let project_root = MOCK_PROJECT_DIR.lock().unwrap();
    set_current_dir(&*project_root)?;
    let build_output_path = project_root.join(BUILD_OUTPUT_DIR);
    let dfx_path = project_root.join("dfx.json");
    let dfx_json = read_to_string(&dfx_path).context("Failed to read dfx.json file")?;
    let dfx_cfg: DfxCfg = serde_json::from_str(&dfx_json).unwrap();
    let canisters: Canisters = dfx_cfg.canisters;
    if let Some((canister_name, canister)) = canisters.0.iter().next() {
        let wasm_file = build_output_path.join(format!("{}.wasm", canister_name));
        if let Err(e) = remove_file(&wasm_file) {
            eprintln!("{} already deleted: {}", &wasm_file.display(), e);
        };
        build_wasm32(canister)?;
        let candid_path = PathBuf::from_str(&get_candid_path_str(&project_root, canister)?)?;
        if let Err(e) = remove_file(&candid_path) {
            eprintln!("{} already deleted: {}", &wasm_file.display(), e);
        };
        gen_candid(&project_root, canister)?;
        assert!(candid_path.exists(), "Candid file should exist");
        let candid_file = read_to_string(&candid_path).context("Failed to read candid file")?;
        let ast: Result<IDLProg, Error> = candid_file.parse();
        assert!(ast.is_ok(), "Build output should exist");
        remove_file(&candid_path)?;
    } else {
        return Err(anyhow!("No rust canister found at {}.", dfx_path.display()));
    }
    Ok(())
}

#[test]
fn test_gen_candid_failure_nonexistent_wasm() -> Result<()> {
    let project_root = MOCK_PROJECT_DIR.lock().unwrap();
    set_current_dir(&*project_root)?;

    let build_output_path = project_root.join(BUILD_OUTPUT_DIR);
    let dfx_path = project_root.join("dfx.json");
    let dfx_json = read_to_string(&dfx_path).context("Failed to read dfx.json file")?;
    let dfx_cfg: DfxCfg = serde_json::from_str(&dfx_json).unwrap();
    let canisters: Canisters = dfx_cfg.canisters;

    if let Some((canister_name, canister)) = canisters.0.iter().next() {
        let wasm_file = build_output_path.join(format!("{}.wasm", canister_name));
        let candid_file_path_str = get_candid_path_str(&project_root, canister)?;
        let candid_file_path = PathBuf::from_str(&candid_file_path_str)?;
        if let Err(e) = remove_file(&wasm_file) {
            eprintln!("{} already deleted: {}", &wasm_file.display(), e);
        }
        if let Err(e) = remove_file(&candid_file_path) {
            eprintln!("{} already deleted: {}", &candid_file_path.display(), e);
        }
        let gen_candid_result = gen_candid(&project_root, canister);
        assert!(!candid_file_path.exists(), "Candid file should not exists");
        assert!(
            gen_candid_result.is_err(),
            "Candid generation result must be an Error"
        );
    } else {
        return Err(anyhow!("No rust canister found at {}.", dfx_path.display()));
    }
    Ok(())
}
