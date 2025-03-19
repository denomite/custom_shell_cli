use std::fs;
use std::path::Path;

pub fn execute(args: &[&str]) {
    if args.is_empty() {
        println!("rm: please provide a file or directory");
    } else {
        let path = Path::new(args[0]);
        if path.is_dir() {
            match fs::remove_dir_all(path) {
                Ok(_) => println!("Removed directory {}", args[0]),
                Err(e) => println!("rm: error: {}", e),
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => println!("Removed file {}", args[0]),
                Err(e) => println!("rm: error: {}", e),
            }
        }
    }
}
