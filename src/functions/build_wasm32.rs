use anyhow::{anyhow, Result};
use cmd_lib::run_cmd;

use crate::types::dfx_cfg::RustCanisterCfg;

pub fn build_wasm32(canister: &RustCanisterCfg) -> Result<()> {
    let canister_name = &canister.package;
    if let Err(e) =
        run_cmd!(cargo build --release --target wasm32-unknown-unknown --package "$canister_name")
    {
        return Err(anyhow!(
            "Failed to build the canister '{}': {}",
            canister_name,
            e
        ));
    }
    println!(
        "candid-gen: Canister '{}' built successfully.",
        canister_name
    );
    Ok(())
}
