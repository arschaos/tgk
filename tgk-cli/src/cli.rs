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
    Init,

    /// Scans for public PII data based on TGK profile.
    Scan(ScanArgs),

    /// Checks the status of current or past audit scans and opt-out removal requests.
    Status,

    /// Triggers automated opt-out requests to purge personal data from data brokers.
    Purge(PurgeArgs),
}

/// Arguments for the `scan` subcommand.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ScanArgs {
    /// Minimum severity level of data exposure to report (e.g. low, medium, high, critical).
    #[arg(short, long)]
    pub severity: Option<String>,
}

/// Arguments for the `purge` subcommand.
#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct PurgeArgs {
    /// Minimum severity level of data exposure to target for purge (e.g. low, medium, high, critical).
    #[arg(short, long)]
    pub severity: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_cmd() {
        let cli = Cli::try_parse_from(["tgk", "init"]).expect("Failed to parse init command");
        assert_eq!(cli.command, Commands::Init);
    }

    #[test]
    fn test_scan_cmd_defaults() {
        let cli =
            Cli::try_parse_from(["tgk", "scan"]).expect("Failed to parse default scan command");
        assert_eq!(cli.command, Commands::Scan(ScanArgs { severity: None }));
    }

    #[test]
    fn test_scan_cmd_with_args() {
        let cli = Cli::try_parse_from(["tgk", "scan", "--severity", "high"])
            .expect("Failed to parse scan command with flags");

        assert_eq!(
            cli.command,
            Commands::Scan(ScanArgs {
                severity: Some("high".to_string()),
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
        assert_eq!(cli.command, Commands::Purge(PurgeArgs { severity: None }));
    }

    #[test]
    fn test_purge_cmd_with_args() {
        let cli = Cli::try_parse_from(["tgk", "purge", "--severity", "critical"])
            .expect("Failed to parse purge command with flags");

        assert_eq!(
            cli.command,
            Commands::Purge(PurgeArgs {
                severity: Some("critical".to_string()),
            })
        );
    }

    #[test]
    fn test_invalid_cmd() {
        let result = Cli::try_parse_from(["tgk", "nonexistent"]);
        assert!(result.is_err());
    }
}
