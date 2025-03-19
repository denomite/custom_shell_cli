use std::fs;

pub fn execute(args: &[&str]) {
    if args.len() < 2 {
        println!("cp: please provide source and destination");
    } else {
        let src = args[0];
        let dest = args[1];
        match fs::copy(src, dest) {
            Ok(_) => println!("Copied {} to {}", src, dest),
            Err(e) => println!("cp: error: {}", e),
        }
    }
}
