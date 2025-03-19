use std::fs;

pub fn execute(args: &[&str]) {
    if args.is_empty() {
        println!("mkdir: please provide a directory name");
    } else {
        let path = args[0];
        match fs::create_dir(path) {
            Ok(_) => println!("Created directory {}", path),
            Err(e) => println!("mkdir: error {}", e),
        }
    }
}
