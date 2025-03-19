use colored::Colorize;

pub fn execute(_args: &[&str]) {
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
