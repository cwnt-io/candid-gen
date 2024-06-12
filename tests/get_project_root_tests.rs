use anyhow::Result;
use candid_gen::functions::get_project_root::get_project_root;
use serial_test::serial;
use std::{
    env::{current_dir, set_current_dir, set_var},
    fs::{create_dir, File},
    path::{Path, PathBuf},
};
use tempfile::{tempdir, TempDir};

fn create_temp_project_with_files() -> TempDir {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    File::create(temp_dir.path().join("dfx.json")).expect("Failed to create dfx.json");
    File::create(temp_dir.path().join("Cargo.toml")).expect("Failed to create Cargo.toml");
    temp_dir
}

fn change_dir<P: AsRef<Path>>(new_dir: P) -> Result<PathBuf> {
    let original_dir = current_dir()?;
    set_current_dir(&new_dir)?;
    Ok(original_dir)
}

#[test]
#[serial]
fn test_get_project_root_success() {
    let temp_dir = create_temp_project_with_files();

    // Change the current directory to the temp dir
    let original_dir = change_dir(&temp_dir).expect("Failed to get current directory");

    // Set the HOME environment variable to a parent directory
    let home_dir = temp_dir.path().parent().unwrap();
    set_var("HOME", home_dir);

    let result = get_project_root();

    // Revert the current directory
    set_current_dir(original_dir).expect("Failed to revert current directory");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), temp_dir.path());
}

#[test]
#[serial]
fn test_get_project_root_failure_no_dfx() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    File::create(temp_dir.path().join("Cargo.toml")).expect("Failed to create Cargo.toml");

    let original_dir = change_dir(&temp_dir).expect("Failed to get current directory");

    let home_dir = temp_dir.path().parent().unwrap();
    set_var("HOME", home_dir);

    let result = get_project_root();

    set_current_dir(original_dir).expect("Failed to revert current directory");

    assert!(result.is_err());
}

#[test]
#[serial]
fn test_get_project_root_failure_no_cargo() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    File::create(temp_dir.path().join("dfx.json")).expect("Failed to create dfx.json");

    let original_dir = change_dir(&temp_dir).expect("Failed to get current directory");

    let home_dir = temp_dir.path().parent().unwrap();
    set_var("HOME", home_dir);

    let result = get_project_root();

    set_current_dir(original_dir).expect("Failed to revert current directory");

    assert!(result.is_err());
}

#[test]
#[serial]
fn test_get_project_root_failure_no_files() {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    let original_dir = change_dir(&temp_dir).expect("Failed to get current directory");

    let home_dir = temp_dir.path().parent().unwrap();
    set_var("HOME", home_dir);

    let result = get_project_root();

    set_current_dir(original_dir).expect("Failed to revert current directory");

    assert!(result.is_err());
}

#[test]
#[serial]
fn test_get_project_root_from_sub_directory() {
    let temp_dir = create_temp_project_with_files();
    let sub_dir = temp_dir.path().join("subdir");
    create_dir(sub_dir).expect("Failed to create subdir");

    // Change the current directory to the subdir
    let original_dir = change_dir(&temp_dir).expect("Failed to get current directory");

    // Set the HOME environment variable to a parent directory
    let home_dir = temp_dir.path().parent().unwrap();
    set_var("HOME", home_dir);

    let result = get_project_root();

    // Revert the current directory
    set_current_dir(original_dir).expect("Failed to revert current directory");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), temp_dir.path());
}
