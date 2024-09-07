use chrono::Local;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;

/// Helper function to get the current date and create the log file path.
fn log_file_path() -> String {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let log_dir = Path::new("logs");

    // Create the log directory if it doesn't exist
    if !log_dir.exists() {
        create_dir_all(log_dir).unwrap();
    }

    format!("logs/{}.log", date)
}

/// Helper function to write a log message with a timestamp.
pub fn write_log(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("[{}] {}", timestamp, message);

    // Log to the console
    println!("{}", log_message);

    // Log to a file
    let log_file = log_file_path();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .unwrap();

    writeln!(file, "{}", log_message).unwrap();
}

/// Logs the backup progress
pub fn log_progress(state: &crate::backup::BackupState) {
    let progress = format!(
        "Progress: {:.2}% - {}/{} files completed. {}/{} bytes processed.",
        state.progress_percentage(),
        state.completed_files,
        state.total_files,
        state.processed_size,
        state.total_size,
    );

    write_log(&progress);
}
