use std::io;
use std::io::Write;

pub fn execute(_args: &[&str]) {
    println!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
