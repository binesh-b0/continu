use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Session {
    access_token: String,
    email: String,
}

const SESSION_FILE: &str = ".backup_tool_session.json";

fn session_file_path() -> PathBuf {
    let mut path = home_dir().expect("Unable to determine home directory");
    path.push(SESSION_FILE);
    path
}

/// Save the session to a file
fn save_session(session: &Session) -> Result<(), Box<dyn Error>> {
    let session_path = session_file_path();
    let file = File::create(session_path)?;
    serde_json::to_writer_pretty(file, session)?;
    Ok(())
}

/// Load the session from a file
fn load_session() -> Result<Session, Box<dyn Error>> {
    let session_path = session_file_path();
    let session: Session = serde_json::from_reader(File::open(session_path)?)?;
    Ok(session)
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
        let access_token = data["access_token"].as_str().ok_or("Failed to get access token")?;
        let session = Session {
            access_token: access_token.to_string(),
            email: email.to_string(),
        };

        save_session(&session)?;
        println!("Successfully logged in as {}", email);
    } else {
        println!("Login failed: {}", response.text().await?);
    }

    Ok(())
}

/// Log out by clearing the session
pub fn logout() -> Result<(), Box<dyn Error>> {
    clear_session()?;
    println!("Successfully logged out.");
    Ok(())
}

/// Show the session status
pub fn session_status() -> Result<(), Box<dyn Error>> {
    if let Ok(session) = load_session() {
        println!("Logged in as {}", session.email);
    } else {
        println!("Not logged in.");
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
        println!("Password reset link sent to {}", email);
    } else {
        println!("Failed to send password reset link: {}", response.text().await?);
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
        println!("Successfully signed up. Please log in.");
    } else {
        println!("Sign-up failed: {}", response.text().await?);
    }

    Ok(())
}
