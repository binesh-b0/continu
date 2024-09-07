use std::fs;
use std::error::Error;

// List of default Ubuntu configuration files to back up
pub fn get_ubuntu_config_files() -> Vec<String> {
    vec![
        "/home/user/.bashrc".to_string(),
        "/home/user/.vimrc".to_string(),
        "/home/user/.gitconfig".to_string(),
        "/etc/apt/sources.list".to_string(),
        "/etc/environment".to_string(),
    ]
}

// Function to check if the current OS is Ubuntu
pub fn is_ubuntu() -> bool {
    match fs::read_to_string("/etc/os-release") {
        Ok(contents) => contents.contains("Ubuntu"),
        Err(_) => false,
    }
}

// Function to load OS-specific settings from the `.init` file
pub fn load_init_settings() -> Result<(Vec<String>, String), Box<dyn Error>> {
    let init_file_path = dirs::home_dir().unwrap().join(".init");
    let mut excluded_files = Vec::new();
    let mut frequency = "daily".to_string(); // Default frequency

    if init_file_path.exists() {
        let content = fs::read_to_string(init_file_path)?;
        for line in content.lines() {
            if line.starts_with("exclude:") {
                let file = line.replace("exclude:", "").trim().to_string();
                excluded_files.push(file);
            } else if line.starts_with("frequency:") {
                frequency = line.replace("frequency:", "").trim().to_string();
            }
        }
    }

    Ok((excluded_files, frequency))
}
