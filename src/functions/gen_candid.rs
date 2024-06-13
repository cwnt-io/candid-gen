use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use cmd_lib::run_cmd;

use crate::{
    functions::get_candid_path_str::get_candid_path_str, types::dfx_cfg::RustCanisterCfg,
    BUILD_OUTPUT_DIR,
};

pub fn gen_candid(project_root: &Path, canister: &RustCanisterCfg) -> Result<()> {
    let candid_path_str = get_candid_path_str(project_root, canister)?;
    let canister_name = &canister.package;
    let canister_path_str = format!("{}/{}.wasm", BUILD_OUTPUT_DIR, canister_name);

    if !PathBuf::from_str(&canister_path_str)?.is_file() {
        return Err(anyhow!(
            "Canister wasm file {} does not exists.",
            canister_path_str
        ));
    }

    if let Err(e) = run_cmd!(candid-extractor "$canister_path_str" > "$candid_path_str") {
        eprintln!(
            "Failed to extract candid for the canister '{}': {}",
            canister_name, e
        );
    }
    println!(
        "candid-gen: Canister '{}' candid file was successfully generated.",
        canister_name
    );

    Ok(())
}
