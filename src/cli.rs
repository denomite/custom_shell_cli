use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Simple CLI shell", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
    #[command(name = "assist")]
    Assist,
    Exit,
}
