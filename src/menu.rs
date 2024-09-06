use inquire::{Select, Password, Text};
use crate::auth;
use crate::backup;
use crate::restore;
use std::error::Error;

pub async fn show_dashboard() -> Result<(), Box<dyn Error>> {
    loop {
        // Present the main dashboard options
        let options = vec![
            "Login",
            "Sign Up",
            "Reset Password",
            "Check Status",
            "Logout",
            "Backup",
            "Restore",
            "Quit",
        ];

        let choice = Select::new("Choose an option:", options.clone()).prompt()?;

        match choice {
            "Login" => {
                login().await?;
            }
            "Sign Up" => {
                signup().await?;
            }
            "Reset Password" => {
                reset_password().await?;
            }
            "Check Status" => {
                auth::session_status()?;
            }
            "Logout" => {
                auth::logout()?;
            }
            "Backup" => {
                if auth::is_logged_in() {
                    backup::backup_files().await?;
                } else {
                    println!("Please log in first.");
                }
            }
            "Restore" => {
                if auth::is_logged_in() {
                    restore::restore_files().await?;
                } else {
                    println!("Please log in first.");
                }
            }
            "Quit" => {
                println!("Quitting...");
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn login() -> Result<(), Box<dyn Error>> {
    let email = Text::new("Enter your email:").prompt()?;
    let password = Password::new("Enter your password:").prompt()?;

    auth::login(&email, &password).await?;
    Ok(())
}

async fn signup() -> Result<(), Box<dyn Error>> {
    let email = Text::new("Enter your email:").prompt()?;
    let password = Password::new("Enter your password:").prompt()?;

    auth::sign_up(&email, &password).await?;
    Ok(())
}

async fn reset_password() -> Result<(), Box<dyn Error>> {
    let email = Text::new("Enter your email:").prompt()?;
    auth::password_reset(&email).await?;
    Ok(())
}
