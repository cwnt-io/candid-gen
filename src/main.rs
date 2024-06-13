use std::{env::set_current_dir, fs::read_to_string};

use anyhow::{anyhow, Context, Result};
use candid_gen::args_options::Args;
use candid_gen::functions::build_wasm32::build_wasm32;
use candid_gen::functions::gen_candid::gen_candid;
use candid_gen::functions::get_project_root::get_project_root;
use candid_gen::functions::run_command::run_command;
use candid_gen::types::canisters::Canisters;
use candid_gen::types::dfx_cfg::DfxCfg;
use clap::Parser;

fn main() -> Result<()> {
    run_command("rustup --version")?;
    run_command("cargo --version")?;
    run_command("candid-extractor --version")?;
    let installed_targets = run_command("rustup target list --installed")?;
    if !installed_targets.contains("wasm32-unknown-unknown") {
        return Err(anyhow!(
            "rustup doesn't have the target wasm32-unknown-unknown installed"
        ));
    }
    let project_root = get_project_root()?;
    set_current_dir(&project_root)?;
    let dfx_path = project_root.join("dfx.json");
    let dfx_json = read_to_string(dfx_path).context("Failed to read dfx.json file")?;
    let dfx_cfg: DfxCfg = serde_json::from_str(&dfx_json).unwrap();
    let canisters: Canisters = dfx_cfg.canisters;
    let args = Args::parse();
    let canisters_to_gen_candid: Canisters = canisters.filter(&args.canisters_names);
    for (_, canister) in canisters_to_gen_candid.0.iter() {
        if let Err(e) = build_wasm32(canister) {
            eprint!("{}", e);
            continue;
        }
        gen_candid(&project_root, canister)?;
    }
    Ok(())
}
