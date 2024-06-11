use std::env::current_dir;

use anyhow::Result;
use args_options::Args;
use clap::Parser;

mod args_options;

fn main() -> Result<()> {
    let _args = Args::parse();
    println!("Hello dir, {:?}", current_dir()?);
    Ok(())
}
