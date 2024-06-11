use clap::Parser;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct Args {
    /// Specify one or more canister names to generate Candid files.
    /// Each canister name should be provided as a separate argument.
    /// If no canister name is passed, this will generate
    /// the Candid files for every rust canister of the project.
    pub canisters_names: Option<Vec<String>>,
}
