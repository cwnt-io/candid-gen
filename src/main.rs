use anyhow::Result;
use args_options::Args;
use clap::Parser;
use functions::get_project_root;

mod args_options;
mod functions;

fn main() -> Result<()> {
    let _args = Args::parse();
    let project_root = get_project_root()?;
    println!("Hello dir, {:?}", project_root);
    Ok(())
}
