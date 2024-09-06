use dotenv::dotenv;
use clap::Parser;

mod cli;
mod auth;
mod backup;
mod restore;

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env
    let cli = cli::Cli::parse();  // Parse the CLI commands
    cli::handle_command(&cli).await;  // Delegate command handling to the cli module
}
