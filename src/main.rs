use anyhow::Result;
use args_options::Args;
use clap::Parser;
use functions::{check_dependencies, get_project_root};

mod args_options;
mod functions;

fn main() -> Result<()> {
    let _args = Args::parse();
    let _project_root = get_project_root()?;
    check_dependencies()?;
    Ok(())
}
