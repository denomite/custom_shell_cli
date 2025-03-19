use chrono::{DateTime, Local};
use colored::Colorize;

pub fn execute(_args: &[&str]) {
    // Get current local time
    let now: DateTime<Local> = Local::now();

    // Format and print
    println!("{}", "Current Date and Time".cyan().bold());
    println!("{}", now.format("%Y-%m-%d %H:%M:%S").to_string().green());
    println!("Weekday: {}", now.format("%A").to_string().yellow());
    // Standard ISO format
    println!("ISO: {}", now.to_rfc3339().blue());
}
