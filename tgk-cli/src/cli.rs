//! Command-line interface definition and argument parsing for TGK.

use clap::{Args, Parser, Subcommand};

/// TGK (The Privacy Toolkit) - Personal data discovery and broker removal utility.
#[derive(Parser)]
#[command(name = "tgk", version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for TGK.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    /// Set up your local and encrypted TGK profile.
    ///
    /// Prompts for identity details (such as full name, address history,
    /// email addresses, phone numbers, and relatives) required to locate
    /// and request removal of personal data across data brokers.
    Init,

    /// Scans for public PII data based on TGK profile.
    ///
    /// Will scan based on specified severity amount, a specific data broker,
    /// amount, and verbosity.
    Scan(ScanArgs),

    /// Checks the status of current or past audit scans and opt-out removal requests.
    Status,

    /// Triggers automated opt-out requests to purge personal data from data brokers.
    ///
    /// Dispatches removal requests filtered by specified severity level, data broker,
    /// and verbosity.
    Purge(PurgeArgs),
}

/// Arguments for the `scan` subcommand.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ScanArgs {
    /// Minimum severity level of data exposure to report (e.g. low, medium, high, critical).
    #[arg(short, long)]
    pub severity: Option<String>,

    /// Specific data broker to target for scanning.
    #[arg(short, long)]
    pub broker: Option<String>,

    /// Maximum amount or number of data sources/records to scan.
    #[arg(short, long)]
    pub amount: Option<usize>,

    /// Verbosity level for detailed scan output.
    #[arg(short, long)]
    pub verbosity: Option<u8>,
}

/// Arguments for the `purge` subcommand.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct PurgeArgs {
    /// Minimum severity level of data exposure to target for purge (e.g. low, medium, high, critical).
    #[arg(short, long)]
    pub severity: Option<String>,

    /// Specific data broker to target for data removal.
    #[arg(short, long)]
    pub broker: Option<String>,

    /// Verbosity level for detailed purge output.
    #[arg(short, long)]
    pub verbosity: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_cmd_defaults() {
        let cli =
            Cli::try_parse_from(["tgk", "scan"]).expect("Failed to parse default scan command");
        assert_eq!(
            cli.command,
            Commands::Scan(ScanArgs {
                severity: None,
                broker: None,
                amount: None,
                verbosity: None,
            })
        );
    }

    #[test]
    fn test_scan_cmd_with_args() {
        let cli = Cli::try_parse_from([
            "tgk",
            "scan",
            "--severity",
            "high",
            "--broker",
            "whitepages",
            "--amount",
            "5",
            "--verbosity",
            "2",
        ])
        .expect("Failed to parse scan command with flags");

        assert_eq!(
            cli.command,
            Commands::Scan(ScanArgs {
                severity: Some("high".to_string()),
                broker: Some("whitepages".to_string()),
                amount: Some(5),
                verbosity: Some(2),
            })
        );
    }

    #[test]
    fn test_status_cmd() {
        let cli = Cli::try_parse_from(["tgk", "status"]).expect("Failed to parse status command");
        assert_eq!(cli.command, Commands::Status);
    }

    #[test]
    fn test_purge_cmd_defaults() {
        let cli =
            Cli::try_parse_from(["tgk", "purge"]).expect("Failed to parse default purge command");
        assert_eq!(
            cli.command,
            Commands::Purge(PurgeArgs {
                severity: None,
                broker: None,
                verbosity: None,
            })
        );
    }

    #[test]
    fn test_purge_cmd_with_args() {
        let cli = Cli::try_parse_from([
            "tgk",
            "purge",
            "--severity",
            "critical",
            "--broker",
            "spokeo",
            "--verbosity",
            "1",
        ])
        .expect("Failed to parse purge command with flags");

        assert_eq!(
            cli.command,
            Commands::Purge(PurgeArgs {
                severity: Some("critical".to_string()),
                broker: Some("spokeo".to_string()),
                verbosity: Some(1),
            })
        );
    }
}
