use clap::{Parser, Subcommand};
use crate::auth;
use nix::unistd::Uid;

use crate::backup;
use crate::restore;

/// Define the CLI structure with clap
#[derive(Parser)]
#[command(name = "Backup Tool")]
#[command(about = "A tool to backup and restore user configurations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

fn is_root() -> bool {
    Uid::effective().is_root()
}

/// Define the available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Login to Supabase
    Login {
        email: String,
        password: String,
    },
    Signup {
        email: String,
        password: String,
    },
    /// Logout from the current session
    Logout {},
    /// Check login status
    Status {},
    /// Request password reset
    Reset {
        email: String,
    },
    /// Backup all configuration files and package lists
    Backup {},
    /// Restore configuration files and packages
    Restore {},
}

/// Handle the parsed CLI command
pub async fn handle_command(cli: &Cli) {
    match &cli.command {
        Commands::Signup { email, password } => {
            auth::sign_up(email, password).await.unwrap();
        }
        Commands::Login { email, password } => {
            auth::login(email, password).await.unwrap();
        }
        Commands::Logout {} => {
            auth::logout().unwrap();
        }
        Commands::Status {} => {
            auth::session_status().unwrap();
        }
        Commands::Reset { email } => {
            auth::password_reset(email).await.unwrap();
        }
        Commands::Backup {} => {
            if !is_root() {
                println!("Please run this command as root or with sudo.");
                return;
            }
            if auth::is_logged_in() {
                backup::backup_files().await.unwrap();
            } else {
                println!("Please log in first.");
            }
        }
        Commands::Restore {} => {
            if !is_root() {
                println!("Please run this command as root or with sudo.");
                return;
            }
            if auth::is_logged_in() {
                restore::restore_files().await.unwrap();
            } else {
                println!("Please log in first.");
            }
        }
    }
}
