use std::env;

pub fn execute(_args: &[&str]) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("pwd, error: {}", e),
    }
}
