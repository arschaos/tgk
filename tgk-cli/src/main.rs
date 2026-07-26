//! Entry point for the TGK command-line application (`tgk-cli`).

mod cli;
mod commands;
mod prompt;

use clap::Parser;
use cli::Cli;

/// Parses command-line arguments and dispatches execution to the corresponding command handler.
fn main() {
    let cli = Cli::parse();
    commands::dispatch(cli.command);
}
