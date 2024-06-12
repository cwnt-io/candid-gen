use std::{collections::HashMap, env::set_current_dir, fs::read_to_string};

use anyhow::{anyhow, Context, Result};
use candid_gen::args_options::Args;
use candid_gen::dfx_cfg::{DfxCfg, RustCanisterCfg};
use candid_gen::functions::get_candid_path_str::get_candid_path_str;
use candid_gen::functions::get_project_root::get_project_root;
use candid_gen::functions::run_command::run_command;
use clap::Parser;
use cmd_lib::run_cmd;

type Canisters = HashMap<String, RustCanisterCfg>;

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
    let canisters_to_gen_candid: Canisters = match args.canisters_names {
        Some(canisters_names) => canisters_names
            .iter()
            .fold(HashMap::new(), |mut map, name| {
                if let Some(canister) = canisters.get(name) {
                    map.insert(name.clone(), canister.clone());
                } else {
                    eprintln!(
                        "candid-gen error: Not able to generate the candid file for the canister: {}.\n\
                        Verify if it is a 'rust' canister type, or if the name is correct.\n",
                        name
                    );
                }
                map
            }),
        None => canisters,
    };
    for (canister_name, canister) in canisters_to_gen_candid.iter() {
        if let Err(e) = run_cmd!(cargo build --release --target wasm32-unknown-unknown --package "$canister_name")
        {
            eprintln!("Failed to build the canister '{}': {}", canister_name, e);
            continue;
        }
        println!(
            "candid-gen: Canister '{}' built successfully.",
            canister_name
        );
        let candid = get_candid_path_str(&project_root, canister)?;
        if let Err(e) = run_cmd!(
            candid-extractor
            "target/wasm32-unknown-unknown/release/$canister_name.wasm" >
            "$candid")
        {
            eprintln!(
                "Failed to extract candid for the canister '{}': {}",
                canister_name, e
            );
        }
        println!(
            "candid-gen: Canister '{}' candid file was successfully generated.",
            canister_name
        );
    }
    Ok(())
}
