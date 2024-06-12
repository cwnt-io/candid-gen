use anyhow::{anyhow, Context, Result};
use std::{
    env::{current_dir, var},
    path::{Path, PathBuf},
};

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
