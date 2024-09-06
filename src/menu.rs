use inquire::{Select};
use colored::*; // Add colored for terminal colors and text styling
use crate::auth;
use crate::backup;
use crate::restore;
use std::error::Error;

pub async fn show_dashboard() -> Result<(), Box<dyn Error>> {
    // Clear the terminal screen to make it look cleaner
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    // ASCII art for the app name "Continu"
    let app_name = r#"
                 _   _             
  ___ ___  _ __ | |_(_)_ __  _   _ 
 / __/ _ \| '_ \| __| | '_ \| | | |
| (_| (_) | | | | |_| | | | | |_| |
 \___\___/|_| |_|\__|_|_| |_|\__,_| 
        "#.blue().bold();
        
    println!("{}", app_name);

    // Present the categories and sub-options directly
    let options = vec![
        "Auth: Login Logout Signup Reset Password Delete Account".cyan().to_string(),
        "Backup: Sync List".green().to_string(),
        "Restore: Restore".yellow().to_string(),
        "Help: Status".magenta().to_string(),
        "Exit: Quit Kill Service".red().to_string(),
    ];

    loop {
        // Prompt the user for action, only showing key options
        let actions = vec![
            "Login", "Sign Up", "Reset Password", "Logout", "Backup", "Restore", "Check Status", "Quit",
        ];

        let choice = Select::new("Choose an option:", actions.clone()).prompt()?;

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
            "Logout" => {
                auth::logout()?;
            }
            "Backup" => {
                if auth::is_logged_in() {
                    backup::backup_files().await?;
                } else {
                    println!("{}", "Please log in first.".red());
                }
            }
            "Restore" => {
                if auth::is_logged_in() {
                    restore::restore_files().await?;
                } else {
                    println!("{}", "Please log in first.".red());
                }
            }
            "Check Status" => {
                auth::session_status()?;
            }
            "Quit" => {
                println!("{}", "Quitting...".red());
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

// Login function (NO confirm password)
async fn login() -> Result<(), Box<dyn Error>> {
    let email = inquire::Text::new("Enter your email:").prompt()?;
    let password = inquire::Password::new("Enter your password:").prompt()?;  // No confirmation needed

    auth::login(&email, &password).await?;
    Ok(())
}

// Signup function (with confirm password)
async fn signup() -> Result<(), Box<dyn Error>> {
    let email = inquire::Text::new("Enter your email:").prompt()?;
    let password = inquire::Password::new("Enter your password:").prompt()?;
    let confirm_password = inquire::Password::new("Confirm your password:").prompt()?;

    if password != confirm_password {
        println!("{}", "Passwords do not match. Please try again.".red());
    } else {
        auth::sign_up(&email, &password).await?;
    }
    Ok(())
}

// Reset password function
async fn reset_password() -> Result<(), Box<dyn Error>> {
    let email = inquire::Text::new("Enter your email:").prompt()?;
    auth::password_reset(&email).await?;
    Ok(())
}
