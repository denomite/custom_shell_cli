use std::fs;

pub fn execute(args: &[&str]) {
    if args.is_empty() {
        println!("touch: please provide a file name");
    } else {
        let path = args[0];
        match fs::File::create(path) {
            Ok(_) => println!("Crated file {}", path),
            Err(e) => println!("touch: error: {}", e),
        }
    }
}
