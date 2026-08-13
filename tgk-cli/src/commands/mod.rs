//! Command handlers and dispatch logic for TGK CLI subcommands.

pub mod init;
pub mod scan;

use crate::cli::Commands;

/// Dispatches the parsed [`Commands`] variant to its respective module handler.
pub fn dispatch(command: Commands) {
    match command {
        Commands::Init => init::run(),
        Commands::Scan(args) => scan::run(&args),
    }
}
