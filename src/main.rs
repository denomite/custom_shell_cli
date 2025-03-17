use colored::Colorize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use sysinfo::{RefreshKind, System};

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
            "sysninfo" => {
                let mut sys = System::new_all(); // Initialize sysinfo

                let command = "sysinfo"; // This would come from input logic

                match command {
                    "sysinfo" => {
                        // Refresh system information
                        sys.refresh_specifics(RefreshKind::everything());

                        // Print system information
                        println!("{}", "System Information: ".cyan().bold());

                        // Os Info
                        println!("OS: {}", System::name().unwrap_or("Unknown".to_string()));
                        println!(
                            "Host: {}",
                            System::host_name().unwrap_or("Unknown".to_string())
                        );
                        // CPU Info
                        let cpu_count = sys.cpus().len();
                        let cpu_usage = sys.global_cpu_usage();
                        println!(
                            "CPUs: {} cores ({}% usage)",
                            cpu_count.to_string().green(),
                            format!("{:.1}", cpu_usage).yellow()
                        );
                    }
                    _ => {
                        println!("{}: '{}'", "Unknown Command".red(), command);
                    }
                }
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
                    "
                    Available commands:

                    cd <dir>             - Change the current directory
                    echo <text>          - Print text to the terminal
                    pwd                  - Print the current working directory
                    ls / dir             - List files in the current directory
                    cat <file>           - Display the contents of a file
                    cp <src> <dest>      - Copy a file from source to destination
                    mv <src> <dest>      - Move or rename a file
                    rm <path>            - Remove a file or directory
                    mkdir <dir>          - Create a new directory
                    touch <file>         - Create an empty file
                    write <file> <text>  - Write text to a file
                    whoami               - Print the current username
                    clear / cls          - Clear the terminal screen
                    {} - Show system information
                    {} - Show command history
                    {} - Show this help message
                    {}  - Exit the shell",
                    "sysninfo".cyan(),
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
