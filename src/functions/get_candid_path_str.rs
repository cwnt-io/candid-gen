use std::{fs::create_dir_all, path::Path};

use anyhow::{anyhow, Result};

use crate::types::dfx_cfg::RustCanisterCfg;

pub fn get_candid_path_str(project_root: &Path, canister: &RustCanisterCfg) -> Result<String> {
    let candid_file_path = project_root.join(&canister.candid_file_path_str);
    let candid_file_dir = candid_file_path.parent().unwrap();
    create_dir_all(candid_file_dir)?;
    if !candid_file_dir
        .to_str()
        .unwrap()
        .contains(&canister.package)
    {
        return Err(anyhow!(
            "fn gen_candid_path_str: Could not find the candid dir."
        ));
    }
    let candid_file_path_str = candid_file_path.to_str().unwrap().to_string();
    Ok(candid_file_path_str)
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
            candid_file_path_str: "".to_string(),
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
            candid_file_path_str: "subdir/test/test.did".to_string(),
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
