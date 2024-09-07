use crate::cli::print_to_dashboard::{print_to_dashboard, print_to_dashboard_with_coordinates};
use crate::encryption::{self, decrypt_data, encrypt_data};
use crate::supabase::{check_os_details, create_config_entry, create_user_entry};
use chrono::Utc; // For timestamp
use colored::Colorize;
use dirs::home_dir;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Session {
    access_token: String,
    email: String,
    user_id: String,    // Store the user ID for later reference
    login_time: String, // Timestamp of login
}

const SESSION_FILE: &str = "continue.bin"; // Using binary format for storage

fn session_file_path() -> PathBuf {
    let mut path = home_dir().expect("Unable to determine home directory");
    path.push(SESSION_FILE);
    path
}

/// Save the session to a file, encrypted for security
fn save_session(session: &Session) -> Result<(), Box<dyn Error>> {
    let session_path = session_file_path();
    let session_data = bincode::serialize(&session)?; // Serialize session to binary format
    let encrypted_data = encrypt_data(&session_data)?; // Encrypt the session data
    let mut file = File::create(session_path)?;
    file.write_all(&encrypted_data)?; // Write encrypted data to binary file
    Ok(())
}

/// Load the session from a file, decrypting it
fn load_session() -> Result<Session, Box<dyn Error>> {
    let session_path = session_file_path();
    if session_path.exists() {
        let encrypted_data = fs::read(session_path)?; // Read encrypted data
        let decrypted_data = decrypt_data(&encrypted_data)?; // Decrypt the session data
        let session: Session = bincode::deserialize(&decrypted_data)?; // Deserialize from binary format
        Ok(session)
    } else {
        Err("Session file does not exist.".into())
    }
}

/// Clear the session by deleting the session file
fn clear_session() -> Result<(), Box<dyn Error>> {
    let session_path = session_file_path();
    if session_path.exists() {
        fs::remove_file(session_path)?;
    }
    Ok(())
}

/// Check if the session exists and return status
pub fn is_logged_in() -> bool {
    session_file_path().exists()
}

/// Perform login and save the session
pub async fn login(email: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let supabase_url = std::env::var("SUPABASE_URL")?;
    let supabase_key = std::env::var("SUPABASE_KEY")?;
    let login_url = format!("{}/auth/v1/token?grant_type=password", supabase_url);

    let response = client
        .post(&login_url)
        .header("apikey", supabase_key)
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let data: serde_json::Value = response.json().await?;
        let access_token = data["access_token"]
            .as_str()
            .ok_or("Failed to get access token")?;
        let user_id = data["user"]["id"].as_str().ok_or("Failed to get user ID")?;
        let login_time = Utc::now().to_rfc3339();

        let session = Session {
            access_token: access_token.to_string(),
            email: email.to_string(),
            user_id: user_id.to_string(),
            login_time,
        };
        println!("{}", session.access_token.to_string());
        // Save the session securely
        save_session(&session)?;

        let (current_os_name, current_os_version) = crate::config::get_os_details()?;
        check_os_details(user_id, &current_os_name, &current_os_version).await?;

        print_to_dashboard(&format!("Successfully logged in as {}", email).green());
    } else {
        print_to_dashboard(&format!("Login failed: {}", response.text().await?).red());
    }

    Ok(())
}

/// Log out by clearing the session
pub fn logout() -> Result<(), Box<dyn Error>> {
    clear_session()?;
    print_to_dashboard("Successfully logged out.".green().to_string().as_str());
    Ok(())
}

/// Show the session status
pub fn session_status() -> Result<(), Box<dyn Error>> {
    if let Ok(session) = load_session() {
        print_to_dashboard_with_coordinates(
            &format!("Logged in as {}", session.email).green(),
            50,
            9,
        );
        print_to_dashboard_with_coordinates(
            &format!("User ID: {}", session.user_id).green(),
            50,
            10,
        );
        print_to_dashboard_with_coordinates(
            &format!("Login time: {}", session.login_time).green(),
            50,
            11,
        );
    
    } else {
        print_to_dashboard(&"Not logged in.".red());
    }
    Ok(())
}

/// Request a password reset link
pub async fn password_reset(email: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let supabase_url = std::env::var("SUPABASE_URL")?;
    let supabase_key = std::env::var("SUPABASE_KEY")?;
    let reset_url = format!("{}/auth/v1/recover", supabase_url);

    let response = client
        .post(&reset_url)
        .header("apikey", supabase_key)
        .json(&serde_json::json!({ "email": email }))
        .send()
        .await?;

    if response.status().is_success() {
        print_to_dashboard(&format!("Password reset link sent to {}", email).green());
    } else {
        print_to_dashboard(
            &format!(
                "Failed to send password reset link: {}",
                response.text().await?
            )
            .red(),
        );
    }

    Ok(())
}

/// Sign up a new user using Supabase Auth
pub async fn sign_up(email: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let supabase_url = std::env::var("SUPABASE_URL")?;
    let supabase_key = std::env::var("SUPABASE_KEY")?;
    let sign_up_url = format!("{}/auth/v1/signup", supabase_url);

    let response = client
        .post(&sign_up_url)
        .header("apikey", supabase_key)
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    if response.status().is_success() {
        // Extract user information from response
        let data: serde_json::Value = response.json().await?;
        let user_id = data["user"]["id"].as_str().ok_or("Failed to get user id")?;

        print_to_dashboard(
            "Successfully signed up. Please log in."
                .green()
                .to_string()
                .as_str(),
        );
    } else {
        let error_message = response.text().await?;
        print_to_dashboard(&format!("Sign-up failed: {}", error_message).red());
    }

    Ok(())
}
