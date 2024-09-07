use inquire::Select;
use colored::*; // Add colored for terminal colors and text styling
use crate::auth;
use crate::backup;
use crate::restore;
use crate::cli::print_to_dashboard::print_to_dashboard_with_coordinates;
use std::error::Error;
use std::io::{stdout, Write};

/// Clears the terminal screen and moves the cursor to the top-left corner.
pub fn clear_screen() {
    // ANSI escape code to clear the entire screen and move the cursor to (1,1)
    print!("\x1B[2J\x1B[H");

    // Flush stdout to apply the changes immediately
    stdout().flush().unwrap();
}

pub async fn show_dashboard() -> Result<(), Box<dyn Error>> {
    clear_screen();
    // ASCII art for the app name "Continu"
    let app_name = r#"
                 _   _             
  ___ ___  _ __ | |_(_)_ __  _   _ 
 / __/ _ \| '_ \| __| | '_ \| | | |
| (_| (_) | | | | |_| | | | | |_| |
 \___\___/|_| |_|\__|_|_| |_|\__,_| 
        "#.blue().bold();

        print_to_dashboard_with_coordinates(&app_name, 0, 0);  // Print title at the top

    loop {
        let mut actions = vec![];
        if auth::is_logged_in() {
            actions = vec![
                "Backup", "Restore", "Logout", "Check Status", "Quit"
            ];
        } else {
            actions = vec![
                "Login", "Sign Up", "Reset Password", "Check Status", "Quit"
            ];
        }

 
        // Extra dashboard info printed to the right side (without interfering with the main menu)
        print_to_dashboard_with_coordinates("Server Status: Running".green().to_string().as_str(), 50, 5);
        print_to_dashboard_with_coordinates("Last Backup: 2 hours ago".yellow().to_string().as_str(), 50, 6);
        print_to_dashboard_with_coordinates("OS: Ubuntu 20.04\n".blue().to_string().as_str(), 50, 7);

        // Prompt the user for action
        let choice = Select::new("Choose an option:", actions.clone()).prompt()?;

        match choice {
            "Login" => {
                clear_screen();
                login().await?;
            }
            "Sign Up" => {
                clear_screen();
                signup().await?;
            }
            "Reset Password" => {
                clear_screen();
                reset_password().await?;
            }
            "Logout" => {
                clear_screen();
                auth::logout()?;
                print_to_dashboard_with_coordinates("Successfully logged out.".green().to_string().as_str(), 0, 12);
            }
            "Backup" => {
                clear_screen();
                if auth::is_logged_in() {
                    backup::backup_system().await?;
                    print_to_dashboard_with_coordinates("Backup completed successfully.".green().to_string().as_str(), 0, 12);
                } else {
                    print_to_dashboard_with_coordinates("Please log in first.".red().to_string().as_str(), 0, 12);
                }
            }
            "Restore" => {
                clear_screen();
                if auth::is_logged_in() {
                    restore::restore_files().await?;
                    print_to_dashboard_with_coordinates("Restore completed successfully.".green().to_string().as_str(), 0, 12);
                } else {
                    print_to_dashboard_with_coordinates("Please log in first.".red().to_string().as_str(), 0, 12);
                }
            }
            "Check Status" => {
                clear_screen();
                auth::session_status()?;
            }
            "Quit" => {
                clear_screen();
                print_to_dashboard_with_coordinates("Quitting...\n".red().to_string().as_str(), 0, 12);
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
    let password = inquire::Text::new("Enter your password:").prompt()?;  // No confirmation needed

    auth::login(&email, &password).await?;
    Ok(())
}

// Signup function (with confirm password)
async fn signup() -> Result<(), Box<dyn Error>> {
    let email = inquire::Text::new("Enter your email:").prompt()?;
    let password = inquire::Password::new("Enter your password:").prompt()?;
    let confirm_password = inquire::Password::new("Confirm your password:").prompt()?;

    if password != confirm_password {
        print_to_dashboard_with_coordinates("Passwords do not match. Please try again.".red().to_string().as_str(), 0, 12);
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
