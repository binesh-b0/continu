use crate::encryption;
use crate::supabase;
use crate::logging::{log_progress, write_log};
use crate::config::get_config_files; // Updated config loading
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::error::Error;
use std::process::Command;

pub struct BackupState {
    pub total_files: usize,
    pub completed_files: usize,
    pub total_size: u64,
    pub processed_size: u64,
}

impl BackupState {
    pub fn new(total_files: usize, total_size: u64) -> Self {
        BackupState {
            total_files,
            completed_files: 0,
            total_size,
            processed_size: 0,
        }
    }

    pub fn update_progress(&mut self, file_size: u64) {
        self.completed_files += 1;
        self.processed_size += file_size;
        log_progress(self);
    }

    pub fn progress_percentage(&self) -> f64 {
        (self.processed_size as f64 / self.total_size as f64) * 100.0
    }
}

// Main backup function for user configurations and installed packages
pub async fn backup_system() -> Result<(), Box<dyn Error>> {
    write_log("Starting system backup...");

    // Backup system configuration files
    let config_files = get_config_files()?; // Get the list of configuration files from config.rs
    let total_size: u64 = config_files.iter().map(|file| get_file_size(file)).sum();
    let mut state = BackupState::new(config_files.len(), total_size);

    for file in config_files {
        if Path::new(&file).exists() {
            let file_size = get_file_size(&file);
            backup_file(&file, &mut state).await?;
            state.update_progress(file_size);
        } else {
            write_log(&format!("File not found: {}", file));
        }
    }

    // Backup list of installed packages
    backup_installed_packages().await?;

    write_log("Backup completed successfully.");
    Ok(())
}

// Backup a specific configuration file
async fn backup_file(file_path: &str, _state: &mut BackupState) -> Result<(), Box<dyn Error>> {
    let file_data = fs::read(file_path)?;
    let encrypted_data = encryption::encrypt_data(&file_data)?;

    supabase::upload_file(file_path, &encrypted_data).await?;

    Ok(())
}

// Get file size for progress tracking
fn get_file_size(file_path: &str) -> u64 {
    fs::metadata(file_path).map(|meta| meta.len()).unwrap_or(0)
}

// Backup installed package list for Ubuntu (dpkg-based systems)
async fn backup_installed_packages() -> Result<(), Box<dyn Error>> {
    write_log("Backing up installed packages...");

    let output = Command::new("dpkg")
        .arg("--get-selections")
        .output()
        .expect("Failed to execute dpkg");

    let packages_file_path = "/tmp/installed_packages.txt";
    let mut file = BufWriter::new(File::create(packages_file_path)?);
    file.write_all(&output.stdout)?;

    // Encrypt and upload the package list
    let file_data = fs::read(packages_file_path)?;
    let encrypted_data = encryption::encrypt_data(&file_data)?;
    supabase::upload_file(packages_file_path, &encrypted_data).await?;

    Ok(())
}
