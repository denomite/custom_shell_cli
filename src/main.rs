use std::io::{self, Write};
mod cli;
mod commands;

fn main() {
    println!("Simple CLI shell.  All available commands: help ");

    // Store command history
    let mut history: Vec<String> = Vec::new();

    loop {
        print!("myshell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        //Add to history before processing
        history.push(input.to_string());

        // Split input into arguments for clap
        let args: Vec<&str> = input.split_whitespace().collect();

        // Prase the input with clap
        match Cli::try_parse_from(&args) {
            "echo" => commands::echo::execute(&parts[1..]),
            "cd" => commands::cd::execute(&parts[1..]),
            "pwd" => commands::pwd::execute(&parts[1..]),
            "ls" | "dir" => commands::ls::execute(&parts[1..]),
            "cat" => commands::ls::execute(&parts[1..]),
            "cp" => commands::cp::execute(&parts[1..]),
            "mv" => commands::mv::execute(&parts[1..]),
            "rm" => commands::rm::execute(&parts[1..]),
            "mkdir" => commands::mkdir::execute(&parts[1..]),
            "touch" => commands::touch::execute(&parts[1..]),
            "write" => commands::write::execute(&parts[1..]),
            "whoami" => commands::write::execute(&parts[1..]),
            "clear" | "cls" => commands::clear::execute(&parts[1..]),
            "sysninfo" => commands::sysninfo::execute(&parts[1..]),
            "history" => commands::history::execute(&parts[1..], &history),
            "date" => commands::date::execute(&parts[1..]),
            "help" => commands::help::execute(&parts[1..]),
            "exit" => break,
            _ => {
                println!("Unknown command: '{}' ", command);
            }
        }
    }
}
