use std::collections::HashMap;

use super::dfx_cfg::RustCanisterCfg;

#[derive(Debug, Default, Clone)]
pub struct Canisters(pub HashMap<String, RustCanisterCfg>);

impl Canisters {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn filter(&self, names: &Option<Vec<String>>) -> Self {
        match names {
            Some(canisters_names) => canisters_names
                .iter()
                .fold(Canisters::new(), |mut map, name| {
                    if let Some(canister) = self.0.get(name) {
                        map.0.insert(name.clone(), canister.clone());
                    } else {
                        eprintln!(
                            "candid-gen error: Not able to generate the candid file for the canister: {}.\n\
                                Verify if it is a 'rust' canister type, or if the name is correct.\n",
                            name
                        );
                    }
                    map
                }),
            None => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_canisters_new() {
        let canisters = Canisters::new();
        assert!(canisters.0.is_empty());
    }

    #[test]
    fn test_canisters_filter_with_names() {
        let mut canisters = Canisters::new();
        canisters.0.insert(
            "test1".to_string(),
            RustCanisterCfg {
                package: "test1".to_string(),
                candid_file_path_str: "src/test1/test1.did".to_string(),
                other: HashMap::new(),
            },
        );
        canisters.0.insert(
            "test2".to_string(),
            RustCanisterCfg {
                package: "test2".to_string(),
                candid_file_path_str: "src/test2/test2.did".to_string(),
                other: HashMap::new(),
            },
        );

        let names = Some(vec!["test1".to_string()]);

        let filtered_canisters = canisters.filter(&names);

        assert_eq!(filtered_canisters.0.len(), 1);
        assert!(filtered_canisters.0.contains_key("test1"));
        assert!(!filtered_canisters.0.contains_key("test2"));
    }

    #[test]
    fn test_canisters_filter_without_names() {
        let mut canisters = Canisters::new();
        canisters.0.insert(
            "test1".to_string(),
            RustCanisterCfg {
                package: "test1".to_string(),
                candid_file_path_str: "src/test1/test1.did".to_string(),
                other: HashMap::new(),
            },
        );
        canisters.0.insert(
            "test2".to_string(),
            RustCanisterCfg {
                package: "test2".to_string(),
                candid_file_path_str: "src/test2/test2.did".to_string(),
                other: HashMap::new(),
            },
        );

        let names: Option<Vec<String>> = None;
        let filtered_canisters = canisters.filter(&names);

        assert_eq!(filtered_canisters.0.len(), 2);
        assert_eq!(filtered_canisters.0, canisters.0);
    }

    #[test]
    fn test_canisters_filter_with_nonexistent_names() {
        let mut canisters = Canisters::new();
        canisters.0.insert(
            "test1".to_string(),
            RustCanisterCfg {
                package: "test1".to_string(),
                candid_file_path_str: "src/test1/test1.did".to_string(),
                other: HashMap::new(),
            },
        );
        canisters.0.insert(
            "test2".to_string(),
            RustCanisterCfg {
                package: "test2".to_string(),
                candid_file_path_str: "src/test2/test2.did".to_string(),
                other: HashMap::new(),
            },
        );

        let names = Some(vec!["test3".to_string()]);

        let filtered_canisters = canisters.filter(&names);

        assert!(filtered_canisters.0.is_empty());
    }
}
