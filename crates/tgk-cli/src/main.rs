use clap::{Args, Parser, Subcommand};
use std::process::ExitCode;
use tgk_core::{Account, Profile};

#[derive(Parser)]
#[command(name = "tgk", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all startup steps (profile now; more steps land later).
    Init(InitArgs),

    /// Manage the user profile.
    Profile(ProfileCmd),
}

#[derive(Args)]
struct ProfileCmd {
    #[command(subcommand)]
    command: ProfileSub,
}

#[derive(Subcommand)]
enum ProfileSub {
    /// Create an in-memory profile from the supplied flags.
    Create(CreateArgs),
}

#[derive(Args)]
struct CreateArgs {
    /// Display name for the profile.
    #[arg(long)]
    name: String,

    /// Email address associated with the identity. May be repeated.
    #[arg(long = "email")]
    emails: Vec<String>,

    /// Username associated with the identity. May be repeated.
    #[arg(long = "username")]
    usernames: Vec<String>,

    /// Registered account as SERVICE:LOGIN. May be repeated.
    #[arg(long = "account")]
    accounts: Vec<String>,

    /// Emit the profile as pretty-printed JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct InitArgs {
    /// Identity inputs used by the profile step.
    #[command(flatten)]
    profile: CreateArgs,
}

impl CreateArgs {
    fn profile(&self) -> Result<Profile, String> {
        let mut accounts = Vec::with_capacity(self.accounts.len());
        for entry in &self.accounts {
            let (service, login) = entry.split_once(':').ok_or_else(|| {
                format!("malformed --account value '{entry}': expected SERVICE:LOGIN")
            })?;
            if service.is_empty() || login.is_empty() {
                return Err(format!(
                    "malformed --account value '{entry}': service and login must be non-empty"
                ));
            }
            accounts.push(Account::new(service.to_owned(), login.to_owned()));
        }

        Ok(Profile {
            display_name: self.name.clone(),
            emails: self.emails.clone(),
            usernames: self.usernames.clone(),
            accounts,
        })
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(init) => run_init(init),
        Commands::Profile(cmd) => match cmd.command {
            ProfileSub::Create(args) => create(args),
        },
    }
}

fn run_init(init: InitArgs) -> ExitCode {
    let status = create(init.profile);
    // additional startup steps are appended here as they land
    status
}

fn create(args: CreateArgs) -> ExitCode {
    let profile = match args.profile() {
        Ok(profile) => profile,
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::FAILURE;
        }
    };

    if args.json {
        let json = match serde_json::to_string_pretty(&profile) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("error: failed to serialize profile: {err}");
                return ExitCode::FAILURE;
            }
        };
        println!("{json}");
        return ExitCode::SUCCESS;
    }

    print_summary(&profile);
    ExitCode::SUCCESS
}

fn print_summary(profile: &Profile) {
    println!("Profile created (in-memory):");
    println!("  {:<10} {}", "Name:", profile.display_name);
    println!("  {:<10} {}", "Emails:", join_or_dash(&profile.emails));
    println!(
        "  {:<10} {}",
        "Usernames:",
        join_or_dash(&profile.usernames)
    );
    println!("  {:<10} {}", "Accounts:", profile.accounts.len());
}

fn join_or_dash(values: &[String]) -> String {
    if values.is_empty() {
        "-".to_owned()
    } else {
        values.join(", ")
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
    fn unknown_command_is_rejected() {
        assert!(Cli::try_parse_from(["tgk", "nope"]).is_err());
        assert!(Cli::try_parse_from(["tgk", "execute"]).is_err());
    }

    fn parsed_create_args(args: &[&str]) -> CreateArgs {
        let cli = Cli::try_parse_from(args).expect("command line should parse");
        match cli.command {
            Commands::Profile(cmd) => match cmd.command {
                ProfileSub::Create(create_args) => create_args,
            },
            Commands::Init(_) => panic!("expected `profile create`"),
        }
    }

    fn parsed_init_args(args: &[&str]) -> InitArgs {
        let cli = Cli::try_parse_from(args).expect("command line should parse");
        match cli.command {
            Commands::Init(init) => init,
            Commands::Profile(_) => panic!("expected `init`"),
        }
    }

    #[test]
    fn profile_create_parses() {
        let args = parsed_create_args(&[
            "tgk",
            "profile",
            "create",
            "--name",
            "X",
            "--email",
            "a@b.c",
            "--account",
            "github:ethan",
        ]);
        assert_eq!(args.name, "X");
        assert_eq!(args.emails, vec!["a@b.c".to_owned()]);
        let profile = args.profile().expect("profile should build");
        assert_eq!(profile.accounts.len(), 1);
        assert_eq!(profile.accounts[0].service, "github");
    }

    #[test]
    fn account_with_colon_in_login_parses() {
        let args = parsed_create_args(&[
            "tgk",
            "profile",
            "create",
            "--name",
            "X",
            "--account",
            "example.com:user:sub",
        ]);
        let profile = args.profile().expect("profile should build");
        assert_eq!(profile.accounts[0].service, "example.com");
        assert_eq!(profile.accounts[0].login, "user:sub");
    }

    #[test]
    fn malformed_account_is_rejected() {
        let args = parsed_create_args(&[
            "tgk",
            "profile",
            "create",
            "--name",
            "X",
            "--account",
            "nocolon",
        ]);
        assert!(args.profile().is_err());
    }

    #[test]
    fn init_runs_profile_step() {
        let init = parsed_init_args(&["tgk", "init", "--name", "X", "--email", "a@b.c"]);
        assert_eq!(init.profile.name, "X");
        assert_eq!(init.profile.emails, vec!["a@b.c".to_owned()]);
        let profile = init.profile.profile().expect("profile should build");
        assert_eq!(profile.display_name, "X");
    }

    #[test]
    fn profile_json_roundtrip() {
        let profile = Profile {
            display_name: "Ethan".to_owned(),
            emails: vec!["a@b.c".to_owned()],
            usernames: vec!["ethan".to_owned()],
            accounts: vec![Account::new("github", "ethan")],
        };
        let json = serde_json::to_string(&profile).expect("serialize");
        let back: Profile = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(profile, back);
    }
}
