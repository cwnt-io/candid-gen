mod args_options;
mod dfx_cfg;
mod functions;

use std::{collections::HashMap, env::set_current_dir, fs::read_to_string};

use anyhow::{Context, Result};
use args_options::Args;
use clap::Parser;
use cmd_lib::run_cmd;
use dfx_cfg::{DfxCfg, RustCanisterCfg};
use functions::{check_dependencies, get_candid_path_str, get_project_root};

type Canisters = HashMap<String, RustCanisterCfg>;

fn main() -> Result<()> {
    check_dependencies()?;
    let args = Args::parse();
    let project_root = get_project_root()?;
    set_current_dir(&project_root)?;
    let dfx_path = project_root.join("dfx.json");
    let dfx_json = read_to_string(dfx_path).context("Failed to read dfx.json file")?;
    let dfx_cfg: DfxCfg = serde_json::from_str(&dfx_json).unwrap();
    let canisters: Canisters = dfx_cfg.canisters;
    let canisters_to_gen_candid: Canisters = match args.canisters_names {
        Some(canisters_names) => canisters_names
            .iter()
            .fold(HashMap::new(), |mut map, name| {
                if let Some(canister) = canisters.get(name) {
                    map.insert(name.clone(), canister.clone());
                }
                map
            }),
        None => canisters,
    };
    for (canister_name, canister) in canisters_to_gen_candid.iter() {
        run_cmd!(cargo build --release --target wasm32-unknown-unknown --package "$canister_name")?;
        let candid = get_candid_path_str(&project_root, canister)?;
        run_cmd!(candid-extractor "target/wasm32-unknown-unknown/release/$canister_name.wasm" > "$candid")?;
    }
    Ok(())
}
