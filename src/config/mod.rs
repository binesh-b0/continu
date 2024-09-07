pub mod ubuntu;

use std::{error::Error, fs};
use crate::config::ubuntu::{get_ubuntu_config_files, is_ubuntu, load_init_settings};
use sys_info::{os_type, os_release};

pub fn get_os_details() -> Result<(String, String), Box<dyn Error>> {
    let os_release_content = fs::read_to_string("/etc/os-release")?;
    
    let mut os_name = String::new();
    let mut os_version = String::new();

    // Parse the /etc/os-release file
    for line in os_release_content.lines() {
        if line.starts_with("NAME=") {
            os_name = line.replace("NAME=", "").replace("\"", "");
        }
        if line.starts_with("VERSION_ID=") {
            os_version = line.replace("VERSION_ID=", "").replace("\"", "");
        }
    }

    if os_name.is_empty() || os_version.is_empty() {
        return Err("Unable to determine OS name or version.".into());
    }
    Ok((os_name, os_version))


}
// Function to load OS-specific configuration files, checks OS and loads appropriate config
pub fn get_config_files() -> Result<Vec<String>, Box<dyn Error>> {
    let mut config_files = Vec::new();

    // Check if OS is Ubuntu and load corresponding files
    if is_ubuntu() {
        config_files = get_ubuntu_config_files();
    } else {
        return Err("Unsupported OS. Currently only Ubuntu is supported.".into());
    }

    // Load exclusions from `.init` file
    let (excluded_files, _) = load_init_settings()?;
    config_files.retain(|file| !excluded_files.contains(file));

    Ok(config_files)
}

// For future use: dynamic config from the user
pub fn get_backup_frequency() -> Result<String, Box<dyn Error>> {
    let (_, frequency) = load_init_settings()?;
    Ok(frequency)
}
