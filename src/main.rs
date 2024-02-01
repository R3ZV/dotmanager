mod cli;

use clap::Parser;
use cli::{
    add_managed, list_dotfiles, restore_dotfiles, rmv_managed, update_dotfiles, Cli, Command,
};

fn main() {
    let cli = Cli::parse();
    let command = cli.command;
    match command {
        Command::Add { path } => {
            add_managed(path);
        }

        Command::Rmv { path } => {
            rmv_managed(path);
        }

        Command::Upd => {
            update_dotfiles();
        }

        Command::Rest => {
            restore_dotfiles();
        }

        Command::List => {
            if let Some(tracked_files) = list_dotfiles() {
                for line in tracked_files {
                    println!("{}", line);
                }
            }
        }
    }
}
