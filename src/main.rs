use chrono::{DateTime, Local};
use colored::Colorize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use sysinfo::{Disks, RefreshKind, System};

fn main() {
    println!("Simple CLI shell.  To see all available commands: help ");

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

                        // Memory info
                        let total_mem = sys.total_memory() / 1024 / 1024;
                        let used_mem = sys.used_memory() / 1024 / 1024;
                        println!(
                            "Memory: {} MB total, {} MB used ({}%used)",
                            total_mem.to_string().green(),
                            used_mem.to_string().yellow(),
                            (used_mem * 100 / total_mem).to_string().red()
                        );

                        // Disk info
                        println!("Disks:");
                        let disks = Disks::new_with_refreshed_list();

                        {
                            for disk in disks.list() {
                                let total_space = disk.total_space() / 1024 / 1024 / 1024;
                                let available_space = disk.available_space() / 1024 / 1024 / 1024;
                                println!(
                                    "{} ({}): {} GB free / {} GB total",
                                    disk.kind(),
                                    disk.name().to_string_lossy(),
                                    available_space.to_string().green(),
                                    total_space.to_string()
                                );
                            }
                        }
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
            "date" => {
                // Get current local time
                let now: DateTime<Local> = Local::now();

                // Format and print
                println!("{}", "Current Date and Time".cyan().bold());
                println!("{}", now.format("%Y-%m-%d %H:%M:%S").to_string().green());
                println!("Weekday: {}", now.format("%A").to_string().yellow());
                // Standard ISO format
                println!("ISO: {}", now.to_rfc3339().blue());
            }
            "help" => {
                println!(
                    "Available commands:\n\
                    {} <dir>             - Change the current directory\n\
                    {} <text>          - Print text to the terminal\n\
                    {}                  - Print the current working directory\n\
                    {} / {}             - List files in the current directory\n\
                    {} <file>           - Display the contents of a file\n\
                    {} <src> <dest>      - Copy a file from source to destination\n\
                    {} <src> <dest>      - Move or rename a file\n\
                    {} <path>            - Remove a file or directory\n\
                    {} <dir>          - Create a new directory\n\
                    {} <file>         - Create an empty file\n\
                    {} <file> <text>  - Write text to a file\n\
                    {}               - Print the current username\n\
                    {} / {}          - Clear the terminal screen\n\
                    {}                 - Show current date and time\n\
                    {}             - Show system information\n\
                    {}              - Show command history\n\
                    {}                 - Show this help message\n\
                    {}                 - Exit the shell",
                    "cd".green(),
                    "echo".green(),
                    "pwd".green(),
                    "ls".green(),
                    "dir".green(),
                    "cat".green(),
                    "cp".green(),
                    "mv".green(),
                    "rm".green(),
                    "mkdir".green(),
                    "touch".green(),
                    "write".green(),
                    "whoami".green(),
                    "clear".green(),
                    "cls".green(),
                    "date".cyan(),
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
