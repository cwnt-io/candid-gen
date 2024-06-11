use std::{
    collections::{hash_map::Iter, HashMap},
    env::{current_dir, var},
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Ok, Result};
use cmd_lib::run_fun;
use serde::{Deserialize, Serialize};

pub fn check_dependencies() -> Result<()> {
    if let Err(e) = run_fun!(rustup --version 2> /dev/null) {
        return Err(anyhow!("rustup command is not available: {}", e));
    }
    if let Err(e) = run_fun!(cargo - -version) {
        return Err(anyhow!("cargo command is not available: {}", e));
    }
    if let Err(e) = run_fun!(candid - extractor - -version) {
        return Err(anyhow!("candid-extractor command is not available: {}", e));
    }

    let installed_targets = run_fun!(rustup target list --installed)?;
    if !installed_targets.contains("wasm32-unknown-unknown") {
        return Err(anyhow!(
            "rustup doesn't have the target wasm32-unknown-unknown installed"
        ));
    }
    Ok(())
}

pub fn get_project_root() -> Result<PathBuf> {
    let curr_dir = current_dir().context("Failed to get current directory")?;
    let home_dir = var("HOME").context("Failed to get HOME environment variable")?;
    let mut potential_dir = curr_dir.as_path();
    let home_path = Path::new(&home_dir);

    while potential_dir != home_path {
        let dfx_path = potential_dir.join("dfx.json");
        let cargo_path = potential_dir.join("Cargo.toml");

        if dfx_path.exists() && cargo_path.exists() {
            return Ok(potential_dir.to_path_buf());
        }

        potential_dir = match potential_dir.parent() {
            Some(parent) => parent,
            None => break,
        }
    }

    Err(anyhow!("Failed to find the IC project root containing both dfx.json and Cargo.toml from this directory"))
}

#[derive(Serialize, Deserialize)]
struct CanisterCfg {
    #[serde(rename = "candid")]
    candid_path_str: String,
    #[serde(rename = "package")]
    canister_name: String,
    #[serde(rename = "type")]
    canister_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DfxCfg {
    canisters: HashMap<String, CanisterCfg>,
}

impl TryFrom<PathBuf> for DfxCfg {
    type Error = anyhow::Error;

    fn try_from(dfg_cfg_path: PathBuf) -> Result<Self> {
        let file_content = read_to_string(dfg_cfg_path).context("Failed to read dfx.json file")?;
        let config: DfxCfg =
            serde_json::from_str(&file_content).context("Failed to parse JSON content")?;
        Ok(config)
    }
}

pub struct Canisters(HashMap<String, Canister>);

impl TryFrom<DfxCfg> for Canisters {
    type Error = anyhow::Error;

    fn try_from(dfx_cfg: DfxCfg) -> Result<Self> {
        let mut canisters = HashMap::new();
        for (canister_name, canister_cfg) in dfx_cfg.canisters {
            if canister_cfg.canister_type != "rust" {
                continue;
            }
            let canister = Canister::try_from(canister_cfg)?;
            canisters.insert(canister_name, canister);
        }
        Ok(Canisters(canisters))
    }
}

impl Canisters {
    pub fn filter(&self, keys: Vec<String>) -> Self {
        let mut map: HashMap<String, Canister> = HashMap::new();
        for k in keys {
            if self.0.contains_key(&k) {
                map.insert(k.clone(), self.0.get(&k).unwrap().clone());
            }
        }
        Canisters(map)
    }
    pub fn iter(&self) -> Iter<String, Canister> {
        self.0.iter()
    }
}

#[derive(Clone)]
pub struct Canister {
    pub name: String,
    pub candid_path: String,
}

impl TryFrom<CanisterCfg> for Canister {
    type Error = anyhow::Error;

    fn try_from(canister_cfg: CanisterCfg) -> Result<Self> {
        let canister_name = canister_cfg.canister_name.trim();
        let mut candid_filename = canister_name.to_string();
        candid_filename.push_str(".did");

        let mut canister_path = current_dir()?;
        canister_path.push("src");
        canister_path.push(canister_name);
        if !canister_path.is_dir() {
            return Err(anyhow!(
                "Could not find the canister dir from the canister name."
            ));
        }
        let candid_path = canister_path
            .join(candid_filename)
            .to_str()
            .unwrap()
            .to_string();
        Ok(Canister {
            name: canister_name.to_owned(),
            candid_path,
        })
    }
}