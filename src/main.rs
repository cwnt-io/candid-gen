use std::env::set_current_dir;

use anyhow::Result;
use args_options::Args;
use clap::Parser;
use cmd_lib::run_cmd;
use functions::{check_dependencies, get_project_root, Canisters, DfxCfg};

mod args_options;
mod functions;

fn main() -> Result<()> {
    check_dependencies()?;
    let args = Args::parse();
    let project_root = get_project_root()?;
    set_current_dir(&project_root)?;
    let dfg_path = project_root.join("dfx.json");
    let dfx_cfg = DfxCfg::try_from(dfg_path)?;
    let canisters = Canisters::try_from(dfx_cfg)?;
    let canisters_to_gen_candid: Canisters = match args.canisters_names {
        Some(canisters_names) => canisters.filter(canisters_names),
        None => canisters,
    };
    for (canister_name, canister) in canisters_to_gen_candid.iter() {
        run_cmd!(cargo build --release --target wasm32-unknown-unknown --package "$canister_name")?;
        let candid = &canister.candid_path;
        run_cmd!(candid-extractor "target/wasm32-unknown-unknown/release/$canister_name.wasm" > "$candid")?;
    }
    Ok(())
}
