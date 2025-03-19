use std::fs;
pub fn execute(_args: &[&str]) {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{} ", entry.file_name().to_string_lossy());
                }
            }
            println!();
        }
        Err(e) => println!("ls: error: {}", e),
    }
}
