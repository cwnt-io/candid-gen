use core::fmt;
use std::collections::HashMap;

use anyhow::Result;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use super::canisters::Canisters;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RustCanisterCfg {
    pub package: String,
    #[serde(rename = "candid")]
    pub candid_file_path_str: String,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl RustCanisterCfg {
    pub fn new(canister_name: &str) -> Self {
        RustCanisterCfg {
            package: canister_name.to_owned(),
            candid_file_path_str: format!("src/{}/{}.did", canister_name, canister_name),
            other: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct DfxCfg {
    pub canisters: Canisters,
}

impl<'de> Deserialize<'de> for DfxCfg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DfxCfgVisitor;

        impl<'de> Visitor<'de> for DfxCfgVisitor {
            type Value = DfxCfg;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DfxCfg")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DfxCfg, V::Error>
            where
                V: MapAccess<'de>,
            {
                // let mut canisters = HashMap::new();
                let mut canisters = Canisters::new();

                while let Some(key) = map.next_key::<String>()? {
                    if key == "canisters" {
                        let value: HashMap<String, serde_json::Value> = map.next_value()?;
                        for (canister_name, canister_value) in value {
                            if let Ok(canister) =
                                serde_json::from_value::<RustCanisterCfg>(canister_value.clone())
                            {
                                if let Some(canister_type) =
                                    canister_value.get("type").and_then(|v| v.as_str())
                                {
                                    if canister_type == "rust" {
                                        canisters.0.insert(canister_name.clone(), canister);
                                    }
                                }
                            }
                        }
                    } else {
                        let _: serde_json::Value = map.next_value()?;
                    }
                }

                Ok(DfxCfg { canisters })
            }
        }

        const FIELDS: &[&str] = &["canisters"];
        deserializer.deserialize_struct("DfxCfg", FIELDS, DfxCfgVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_rust_canister_cfg_new() {
        let canister = RustCanisterCfg::new("test_canister");
        assert_eq!(canister.package, "test_canister");
        assert_eq!(
            canister.candid_file_path_str,
            "src/test_canister/test_canister.did"
        );
        assert!(canister.other.is_empty());
    }

    #[test]
    fn test_dfx_cfg_deserialize() {
        let data = json!({
            "canisters": {
                "test_canister": {
                    "package": "test_canister",
                    "candid": "src/test_canister/test_canister.did",
                    "type": "rust"
                },
                "non_rust_canister": {
                    "package": "non_rust_canister",
                    "candid": "src/non_rust_canister/non_rust_canister.did",
                    "type": "non_rust"
                }
            }
        });

        let dfx_cfg: DfxCfg = serde_json::from_value(data).expect("Failed to deserialize");

        println!("{:#?}", dfx_cfg);

        assert!(dfx_cfg.canisters.0.contains_key("test_canister"));
        assert!(!dfx_cfg.canisters.0.contains_key("non_rust_canister"));

        let canister = dfx_cfg.canisters.0.get("test_canister").unwrap();
        assert_eq!(canister.package, "test_canister");
        assert_eq!(
            canister.candid_file_path_str,
            "src/test_canister/test_canister.did"
        );
        assert!(canister.other.get("type").is_some());
    }
}
