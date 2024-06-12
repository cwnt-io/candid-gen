use anyhow::{anyhow, Result};
use std::process::Command;

pub fn run_command(command: &str) -> Result<String> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string()))
    }
}
