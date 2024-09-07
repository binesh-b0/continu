use colored::Colorize;
use tokio::task;
use dotenv::dotenv;
use std::time::Duration;
use crate::backup::backup_system;
use crate::config::get_backup_frequency;
use crate::auth::is_logged_in;
use crate::cli::print_to_dashboard::print_to_dashboard_with_coordinates;

mod auth;
mod backup;
mod restore;
mod cli;
mod supabase;
mod encryption;
mod logging;
mod config;

static mut BACKUP_RUNNING: bool = false; // Track backup service status

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables

    // Spawn a background task for automatic backup based on frequency
    let backup_service = task::spawn(async {
        loop {
            if is_logged_in() {
                unsafe { BACKUP_RUNNING = true; }
                let frequency = get_backup_frequency().unwrap_or_else(|_| "daily".to_string());
                let interval = match frequency.as_str() {
                    "hourly" => Duration::from_secs(60 * 60),
                    "weekly" => Duration::from_secs(7 * 24 * 60 * 60),
                    _ => Duration::from_secs(24 * 60 * 60), // Default is daily
                };

                // Run backup
                backup_system().await.unwrap();
                tokio::time::sleep(interval).await;
            } else {
                unsafe { BACKUP_RUNNING = false; }
                print_to_dashboard_with_coordinates("Please log in to start the backup service.".yellow().to_string().as_str(),0,12);
                tokio::time::sleep(Duration::from_secs(60)).await; // Sleep before checking login again
            }
        }
    });

    // Run the dashboard (user interaction)
    cli::menu::show_dashboard().await?;

    // Ensure backup service runs in the background
    // backup_service.await?;

    Ok(())
}
