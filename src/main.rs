use crate::cli::{Cli, Commands};
use clap::Parser;
use std::io::{self, Write};

mod cli;
mod commands;

fn main() {
    println!("All available commands: (cargo/clap): help, (custom/help): assist ");

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
        let mut args: Vec<&str> = vec!["myshell"];
        args.extend(input.split_whitespace());

        match Cli::try_parse_from(&args) {
            Ok(cli) => match cli.command {
                Some(Commands::Echo { text }) => commands::echo::execute(&text),
                Some(Commands::Cd { dir }) => commands::cd::execute(&[&dir]),
                Some(Commands::Pwd) => commands::pwd::execute(&[]),
                Some(Commands::Ls) => commands::ls::execute(&[]),
                Some(Commands::Dir) => commands::ls::execute(&[]),
                Some(Commands::Cat { file }) => commands::cat::execute(&[&file]),
                Some(Commands::Cp { src, dest }) => commands::cp::execute(&[&src, &dest]),
                Some(Commands::Mv { src, dest }) => commands::mv::execute(&[&src, &dest]),
                Some(Commands::Rm { path }) => commands::rm::execute(&[&path]),
                Some(Commands::Mkdir { dir }) => commands::mkdir::execute(&[&dir]),
                Some(Commands::Touch { file }) => commands::touch::execute(&[&file]),
                Some(Commands::Write { file, text }) => commands::write::execute(&[&file, &text]),
                Some(Commands::Whoami) => commands::whoami::execute(&[]),
                Some(Commands::Clear) => commands::clear::execute(&[]),
                Some(Commands::Cls) => commands::clear::execute(&[]),
                Some(Commands::Sysinfo) => commands::sysninfo::execute(&[]),
                Some(Commands::History) => commands::history::execute(&[], &history),
                Some(Commands::Date) => commands::date::execute(&[]),
                Some(Commands::Assist) => commands::assist::execute(&[]),
                Some(Commands::Exit) => commands::exit::execute(&[]),
                None => println!("Unknown command: '{}'", args[1]),
            },
            Err(e) => {
                e.print().unwrap();
                continue;
            }
        }
    }
}
