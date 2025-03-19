use std::fs;
#[allow(dead_code)]
pub fn execute(args: &[&str]) {
    if args.is_empty() {
        println!("cat: please provide a file");
    } else {
        let path = args[0];
        match fs::read_to_string(path) {
            Ok(content) => println!("{}", content),
            Err(e) => println!("cat: error: {}", e),
        }
    }
}
