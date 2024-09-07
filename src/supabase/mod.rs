use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use dotenv::var;
use chrono::Utc;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Config {
    userid: String,
    os_name: String,
    os_version: String,
}


/// Uploads file to Supabase storage
pub async fn upload_file(file_path: &str, encrypted_data: &[u8]) -> Result<(), Box<dyn Error>> {
    let supabase_url = var("SUPABASE_URL")?;
    let supabase_bucket = var("SUPABASE_BUCKET")?;
    let supabase_key = var("SUPABASE_KEY")?;
    let client = Client::new();
    let file_name = file_path.split('/').last().unwrap();
    let url = format!("{}/storage/v1/object/{}/{}", supabase_url, supabase_bucket, file_name);

    let response = client
        .post(&url)
        .bearer_auth(supabase_key)
        .body(encrypted_data.to_vec())
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            println!("Successfully uploaded: {}", file_name);
            Ok(())
        }
        Ok(resp) => {
            eprintln!("Failed to upload: {}. Status: {}. Body: {}", file_name, resp.status(), resp.text().await.unwrap_or_default());
            Err("Failed to upload file.".into())
        }
        Err(e) => {
            eprintln!("Error during upload: {}. File: {}", e, file_name);
            Err(e.into())
        }
    }
}

/// Stores metadata in the Supabase database
pub async fn store_metadata_in_db(file_name: &str, file_size: u64) -> Result<(), Box<dyn Error>> {
    let supabase_url = var("SUPABASE_URL")?;
    let supabase_key = var("SUPABASE_KEY")?;
    let client = Client::new();
    let url = format!("{}/rest/v1/backups", supabase_url);

    let metadata = json!({
        "file_name": file_name,
        "file_size": file_size,
        "backup_date": Utc::now().to_rfc3339(),
    });

    let response = client.post(&url)
        .bearer_auth(supabase_key)
        .json(&metadata)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully stored metadata.");
        Ok(())
    } else {
        println!("Failed to store metadata.");
        Err("Failed to store metadata.".into())
    }
}

/// Creates a new user entry in the database
pub async fn create_user_entry(user_id: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let supabase_url = var("SUPABASE_URL")?;
    let supabase_key = var("SUPABASE_KEY")?;
    let client = Client::new();
    let url = format!("{}/rest/v1/users", supabase_url);

    let user_data = json!({
        "id": user_id,
        "email": email,
        "created_at": Utc::now().to_rfc3339(),
    });

    let response = client.post(&url)
        .bearer_auth(supabase_key)
        .json(&user_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully created user entry.");
        Ok(())
    } else {
        println!("Failed to create user entry.");
        Err("Failed to create user entry.".into())
    }
}

/// Creates a new entry in the configs table
pub async fn create_config_entry(user_id: &str, os_details: &str, system_details: &str) -> Result<(), Box<dyn Error>> {
    let supabase_url = var("SUPABASE_URL")?;
    let supabase_key = var("SUPABASE_KEY")?;
    let client = Client::new();
    let url = format!("{}/rest/v1/configs", supabase_url);

    let config_data = json!({
        "userid": user_id,
        "os_details": os_details,
        "system_details": system_details,
    });

    let response = client.post(&url)
        .bearer_auth(supabase_key)
        .json(&config_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully created config entry.");
        Ok(())
    } else {
        println!("Failed to create config entry.");
        Err("Failed to create config entry.".into())
    }
}

pub async fn check_os_details(user_id: &str, current_os_name: &str, current_os_version: &str) -> Result<(), Box<dyn Error>> {
    let supabase_url = var("SUPABASE_URL")?;
    let supabase_key = var("SUPABASE_KEY")?;
    let client = Client::new();
    let url = format!("{}/rest/v1/configs?user_id=eq.{}", supabase_url, user_id);

    // First, check if there is an existing config for the user
    let response = client
        .get(&url)
        .header("apikey", &supabase_key)  // Use apikey for authorization
        .send()
        .await?;

    if response.status().is_success() {
        let configs: Vec<Config> = response.json().await?;

        // If a config is found, check if OS details match
        if !configs.is_empty() {
            let config = &configs[0];  // Take the first config
            if config.os_name == current_os_name && config.os_version == current_os_version {
                Ok(())  // The OS details match, return Ok
            } else {
                Err("OS not supported.".into())  // OS details don't match
            }
        } else {
            // No config found, insert the new OS details into the database
            add_os_details(user_id, current_os_name, current_os_version).await?;
            Ok(())
        }
    } else {
        // If the response status is not successful, log and return an error
        let error_message = response.text().await?;
        Err(format!("Failed to check OS details: {}", error_message).into())
    }
}

/// Adds the current OS details to the configs table
async fn add_os_details(user_id: &str, os_name: &str, os_version: &str) -> Result<(), Box<dyn Error>> {
    let supabase_url = var("SUPABASE_URL")?;
    let supabase_key = var("SUPABASE_KEY")?;
    let client = Client::new();

    let config_data = json!({
        "user_id": user_id,
        "os_name": os_name,
        "os_version": os_version
    });

    let url = format!("{}/rest/v1/configs", supabase_url);
    let response = client
        .post(&url)
        .header("apikey", &supabase_key)
        .json(&config_data)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(format!("Failed to add OS details: {}", error_message).into())
    }
}