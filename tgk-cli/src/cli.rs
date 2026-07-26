//! Command-line interface definition and argument parsing for TGK.

use clap::{Parser, Subcommand};

/// TGK (The Privacy Toolkit) - Personal data discovery and broker removal utility.
#[derive(Parser)]
#[command(name = "tgk", version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for TGK.
#[derive(Subcommand, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commands {
    /// Set up your local and encrypted TGK profile.
    ///
    /// Prompts for identity details (such as full name, address history,
    /// email addresses, phone numbers, and relatives) required to locate
    /// and request removal of personal data across data brokers.
    Init,
}
