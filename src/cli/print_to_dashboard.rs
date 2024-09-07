use std::io::{stdout, Write};

/// Default function with optional x and y coordinates.
/// When x and y are not supplied, they default to (0, 1).
pub fn print_to_dashboard(message: &str) {
    // Default position is x = 0, y = 1 (second line).
    print_to_dashboard_with_coordinates(message, 50, 10);
}

/// Clear and print messages to a specific part of the dashboard.
/// Allows specifying custom x and y coordinates.
pub fn print_to_dashboard_with_coordinates(message: &str, x: u16, y: u16) {
    // Move the cursor to the specified x, y position
    let move_cursor = format!("\x1B[{};{}H", y + 1, x + 1);  // ANSI escape to move the cursor to (x, y)
    print!("{}{}", move_cursor, message);

    // Flush stdout to ensure it's printed immediately
    stdout().flush().unwrap();
}
