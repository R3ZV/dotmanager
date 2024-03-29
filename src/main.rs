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

        Command::Rmv { remove_by } => {
            rmv_managed(remove_by);
        }

        Command::Upd => {
            update_dotfiles();
        }

        Command::Rest => {
            restore_dotfiles();
        }

        Command::List => {
            if let Some(tracked_files) = list_dotfiles() {
                for (idx, line) in tracked_files.iter().enumerate() {
                    println!("[{}] {}", idx, line);
                }
            }
        }
    }
}
