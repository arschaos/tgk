use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tgk", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print Hello World
    Execute,
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Execute => println!("Hello World"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn execute_is_parsed() {
        let cli = Cli::try_parse_from(["tgk", "execute"]).unwrap();
        assert!(matches!(cli.command, Commands::Execute));
    }

    #[test]
    fn unknown_command_is_rejected() {
        assert!(Cli::try_parse_from(["tgk", "nope"]).is_err());
    }
}