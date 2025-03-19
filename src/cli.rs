use clap::{Parser, Subcommand};

#[derive(Parser)]
// Custom help
#[command(about = "Simple CLI shell", version = "1.0", disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(disable_help_flag = true)]
enum Commands {
    Echo {
        text: Vec<String>,
    },
    Cd {
        dir: String,
    },
    Pwd,
    #[command(name = "ls")]
    Ls,
    #[command(name = "dir")]
    Dir,
    Cat {
        file: String,
    },
    Cp {
        src: String,
        dest: String,
    },
    Mv {
        src: String,
        dest: String,
    },
    Rm {
        path: String,
    },
    Mkdir {
        dir: String,
    },
    Touch {
        file: String,
    },
    Write {
        file: String,
        text: String,
    },
    Whoami,
    #[command(name = "cleaner")]
    Clear,
    #[command(name = "cls")]
    Cls,
    #[command(name = "sysinfo")]
    Sysinfo,
    History,
    Date,
    Help,
    Exit,
}
