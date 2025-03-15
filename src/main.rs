/*
    Command-line shell:
    1.Reads user input
    2.Supports commands:
        - cd: Changes the current directory.
        - echo: Joins arguments with spaces and print them.
        - pwd
        - exit
        - ls
        - cat
        - cp
        - mv
        - rm
        - mkdir
        - whoami
        - clear
        -  help: To see all available commands.
    3.Runs in a loop until the user exits.
*/
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() {
    println!(" --------- Custom Shell --------- ");
    println!(" --- To see all available commands, type: help  ---");

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

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => {
                println!("Ending loop, and exiting!");
                process::exit(0);
            }
            "echo" => {
                let output = args.join(" ");
                println!("{}", output);
            }
            "cd" => {
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
            "pwd" => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("pwd, error: {}", e),
            },
            "ls" => match fs::read_dir(".") {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("{} ", entry.file_name().to_string_lossy());
                        }
                    }
                    println!();
                }
                Err(e) => println!("ls: error: {}", e),
            },
            "cat" => {
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
            "help" => {
                println!("Available commands: ");
                println!(" cd <dir>             - Change the current directory");
                println!(" echo <text>          - Print text to the terminal");
                println!(" pwd                  - Print the current working directory ");
                println!(" exit                 - Exit the shell");
                println!(" ls                   - List files in the current directory");
                println!(" cat <file>           - Display the contents of a file");
                println!(" cp <src> <dest>      - Copy a file from source to destination");
                println!(" mv <src> <dest>      - Move or rename a file");
                println!(" rm <path>            - Remove a file or directory ");
                println!(" mkdir <dir>          - Create a new directory");
                println!(" whoami               - Print the current username");
                println!(" clear                - Clear the terminal screen");
                println!(" help                 - Show this help message");
            }
            _ => {
                println!("Unknown command: '{}' ", command);
            }
        }
    }
}
