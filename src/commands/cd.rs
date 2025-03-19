use std::env;

pub fn execute(args: &[&str]) {
    if args.is_empty() {
        println!("cd: provide a directory");
    } else {
        let path = args[0];
        match env::set_current_dir(path) {
            Ok(()) => println!("Changed directory to {}", path),
            Err(e) => println!("cd: error: {}", e),
        }
    }
}
