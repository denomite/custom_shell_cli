use std::env;
#[allow(dead_code)]
pub fn execute(_args: &[&str]) {
    match env::var("USER").or_else(|_| env::var("USERNAME")) {
        Ok(user) => println!("{}", user),
        Err(e) => println!("Whoami: error: {}", e),
    }
}
