mod cli;
mod dotmanager;

use clap::Parser;
use cli::{Cli, Command};
use dotmanager::DotManager;

fn main() {
    let home_dir = home::home_dir().expect("Couldn't find home dir!");
    let dot_manager = DotManager::new(home_dir);
    match dot_manager.init() {
        Ok(created) => {
            if created == true {
                println!("Create .dotmanager inside your dotfiles directory!");
            }
        }
        Err(err) => {
            println!("Couldn't create .dotmanager in your dotfiles directory due to: {err}")
        }
    };
    let cli = Cli::parse();
    let command = cli.command;
    match command {
        Command::Add { path } => {
            dot_manager.add(path);
        }

        Command::Rmv { remove_by } => match dot_manager.rmv(&remove_by) {
            Ok(removed) => {
                if removed == true {
                    println!("{remove_by} removed successfully!");
                } else {
                    println!("No match for {remove_by} in your dotfiles!");
                }
            }
            Err(err) => println!("Couldn't remove due to: {err}"),
        },

        Command::Upd => {
            dot_manager.update();
        }

        Command::Rest => {
            dot_manager.restore();
        }

        Command::List => match dot_manager.read() {
            Ok(tracked_files) => {
                println!("This are your tracked dotfiles: ");
                for (idx, line) in tracked_files.iter().enumerate() {
                    println!("[{}] {}", idx, line);
                }
            }
            Err(err) => println!("Couldn't list dotfiles due to: {}", err),
        },
    }
}
