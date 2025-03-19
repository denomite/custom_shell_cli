use std::fs;

pub fn execute(args: &[&str]) {
    if args.len() < 2 {
        println!("mv: please provide source and destination");
    } else {
        let src = args[0];
        let dest = args[1];
        match fs::rename(src, dest) {
            Ok(_) => println!("Moved {} to {}", src, dest),
            Err(e) => println!("mv: error: {}", e),
        }
    }
}
