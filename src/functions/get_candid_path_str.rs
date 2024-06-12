use std::path::Path;

use anyhow::{anyhow, Result};

use crate::dfx_cfg::RustCanisterCfg;

pub fn get_candid_path_str(project_root: &Path, canister: &RustCanisterCfg) -> Result<String> {
    let candid_path = project_root.join(&canister.candid);
    if !candid_path
        .parent()
        .is_some_and(|p| p.is_dir() && p.to_str().unwrap().contains(&canister.package))
    {
        return Err(anyhow!(
            "fn gen_candid_path_str: Could not find the candid dir."
        ));
    }
    let candid_str = candid_path.to_str().unwrap().to_string();
    Ok(candid_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::collections::HashMap;
    use std::env::set_current_dir;
    use std::fs::{create_dir_all, File};
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn test_get_candid_path_str_success() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        set_current_dir(&temp_dir)
            .expect("Failed to set temp_dir as the current dir and project_root.");
        let canister = RustCanisterCfg::new("test");
        let candid_dir = temp_dir.path().join("src/test/");
        create_dir_all(&candid_dir).expect("Failed to create directories");
        File::create(candid_dir.join("test.did")).expect("Failed to create file");

        let result = get_candid_path_str(temp_dir.path(), &canister);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            temp_dir.path().join("src/test/test.did").to_str().unwrap()
        );
    }

    #[test]
    #[serial]
    fn test_get_candid_path_str_failure_nonexistent_dir() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        set_current_dir(&temp_dir)
            .expect("Failed to set temp_dir as the current dir and project_root.");
        let canister = RustCanisterCfg::new("nonexistent");

        let result = get_candid_path_str(temp_dir.path(), &canister);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "fn gen_candid_path_str: Could not find the candid dir."
        );
    }

    #[test]
    #[serial]
    fn test_get_candid_path_str_failure_invalid_path() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        set_current_dir(&temp_dir)
            .expect("Failed to set temp_dir as the current dir and project_root.");
        let canister = RustCanisterCfg {
            package: "test".to_string(),
            candid: "".to_string(),
            other: HashMap::new(),
        };

        let result = get_candid_path_str(temp_dir.path(), &canister);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "fn gen_candid_path_str: Could not find the candid dir."
        );
    }

    #[test]
    #[serial]
    fn test_get_candid_path_str_with_sub_directory() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        set_current_dir(&temp_dir)
            .expect("Failed to set temp_dir as the current dir and project_root.");
        let canister = RustCanisterCfg {
            package: "test".to_string(),
            candid: "subdir/test/test.did".to_string(),
            other: HashMap::new(),
        };
        let candid_dir = temp_dir.path().join("subdir/test/");
        create_dir_all(&candid_dir).expect("Failed to create directories");
        File::create(candid_dir.join("test.did")).expect("Failed to create file");

        let result = get_candid_path_str(temp_dir.path(), &canister);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            temp_dir
                .path()
                .join("subdir/test/test.did")
                .to_str()
                .unwrap()
        );
    }
}
