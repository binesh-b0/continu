use dotenv::dotenv;
use std::error::Error;

mod auth;
mod backup;
mod restore;
mod menu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env
    
    // Show the dashboard (interactive menu)
    menu::show_dashboard().await?;
    
    Ok(())
}
