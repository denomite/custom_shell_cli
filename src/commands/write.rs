use std::fs;

pub fn execute(args: &[&str]) {
    if args.len() < 2 {
        println!("write: please provide a file name and text(example: write file.txt Hello");
    } else {
        let path = args[0];
        let content = args[1..].join(" ");
        match fs::write(path, content) {
            Ok(_) => println!("Wrote to file {}", path),
            Err(e) => println!("write: error: {}", e),
        }
    }
}
