use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tgk", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set up your local and encrypted TGK profile.
    Init,
}