use clap::Parser;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct Args {
    /// Specify one or more canister names to generate Candid files, or use '.'
    /// to target the current canister in the working directory or its
    /// subdirectories. Each canister name should be provided as a separate argument.
    pub canisters_names: Vec<String>,
}
