pub mod init;

use crate::cli::Commands;

pub fn dispatch(command: Commands) {
    match command {
        Commands::Init => init::run(),
    }
}
