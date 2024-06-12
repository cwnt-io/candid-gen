use core::fmt;
use std::collections::HashMap;

use anyhow::Result;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RustCanisterCfg {
    package: String,
    pub candid: String,
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct DfxCfg {
    pub canisters: HashMap<String, RustCanisterCfg>,
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
                let mut canisters = HashMap::new();

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
                                        canisters.insert(canister_name.clone(), canister);
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
