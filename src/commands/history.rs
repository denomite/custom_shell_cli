pub fn execute(_args: &[&str], history: &[String]) {
    if history.is_empty() {
        println!("No command history yet.");
    } else {
        for (i, cmd) in history.iter().enumerate() {
            println!("{} : {}", i + 1, cmd);
        }
    }
}
