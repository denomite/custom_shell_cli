use colored::Colorize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() {
    println!(" #################### Custom Shell #################### ");
    println!(" --- To see all available commands, type: help  ---");

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
            "ls" | "dir" => match fs::read_dir(".") {
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
            "cp" => {
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
            "mv" => {
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
            "rm" => {
                if args.is_empty() {
                    println!("rm: please provide a file or directory");
                } else {
                    let path = Path::new(args[0]);
                    if path.is_dir() {
                        match fs::remove_dir_all(path) {
                            Ok(_) => println!("Removed directory {}", args[0]),
                            Err(e) => println!("rm: error: {}", e),
                        }
                    } else {
                        match fs::remove_file(path) {
                            Ok(_) => println!("Removed file {}", args[0]),
                            Err(e) => println!("rm: error: {}", e),
                        }
                    }
                }
            }
            "mkdir" => {
                if args.is_empty() {
                    println!("mkdir: please provide a directory name");
                } else {
                    let path = args[0];
                    match fs::create_dir(path) {
                        Ok(_) => println!("Created directory {}", path),
                        Err(e) => println!("mkdir: error {}", e),
                    }
                }
            }
            "touch" => {
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
            "write" => {
                if args.len() < 2 {
                    println!(
                        "write: please provide a file name and text(example: write file.txt Hello"
                    );
                } else {
                    let path = args[0];
                    let content = args[1..].join(" ");
                    match fs::write(path, content) {
                        Ok(_) => println!("Wrote to file {}", path),
                        Err(e) => println!("write: error: {}", e),
                    }
                }
            }
            "whoami" => match env::var("USER").or_else(|_| env::var("USERNAME")) {
                Ok(user) => println!("{}", user),
                Err(e) => println!("Whoami: error: {}", e),
            },
            "clear" | "cls" => {
                println!("\x1B[2J\x1B[H");
                io::stdout().flush().unwrap();
            }
            "history" => {
                if history.is_empty() {
                    println!("No command history yet.");
                } else {
                    for (i, cmd) in history.iter().enumerate() {
                        println!("{} : {}", i + 1, cmd);
                    }
                }
            }
            "help" => {
                println!(
                    "Available commands:\n\
                    cd <dir>             - Change the current directory\n\
                    echo <text>          - Print text to the terminal\n\
                    pwd                  - Print the current working directory\n\
                    ls / dir             - List files in the current directory\n\
                    cat <file>           - Display the contents of a file\n\
                    cp <src> <dest>      - Copy a file from source to destination\n\
                    mv <src> <dest>      - Move or rename a file\n\
                    rm <path>            - Remove a file or directory\n\
                    mkdir <dir>          - Create a new directory\n\
                    touch <file>         - Create an empty file\n\
                    write <file> <text>  - Write text to a file\n\
                    whoami               - Print the current username\n\
                    clear / cls          - Clear the terminal screen\n\
                    {}                   - Show command history\n\
                    {}                   - Show this help message\n\
                    {}                   - Exit the shell",
                    "history".yellow().underline(),
                    "help".yellow().underline(),
                    "exit".red().bold()
                );
            }
            _ => {
                println!("Unknown command: '{}' ", command);
            }
        }
    }
}
