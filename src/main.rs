use std::fs;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Available commands
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// adds to dotfiles tracker
    Add {
        /// path of the file / directory
        path: String,
    },

    /// removes from dotfiles tracker
    Rmv {
        /// path of the file / directory
        path: String,
    },

    /// updates dotfiles folder
    Upd,

    /// restores all the dotfiles to their destination
    Rest,
}

/// reads the contents of the ~/.dotmanager
/// it the file doesn't exist it will create it
/// if it can't read it, will return None
fn read_managed() -> Option<Vec<String>> {
    let path = PathBuf::from("~/dotfiles/.dotmanager");
    // TODO: check if it exist
    // if not create it
    let file = fs::read(path);
    if file.is_err() {
        return None;
    }
    let mut lines = Vec::new();
    for line in file.unwrap() {
        lines.push(line.to_string());
    }
    Some(lines)
}

/// Adds the path to the files tracked
/// by the dotmanager
fn add_managed(path: String) {
    todo!();
}

/// Removes the path from the
/// files tracked by the dotmanager
fn rmv_managed(path: String) {
    todo!();
}

/// Goes through all the tracked paths
/// and if there are changes to the file / directory
/// the file / directory will be copied to the
/// dotfiles directory
fn update_dotfiles() {
    todo!();
}

/// Goes through all the tracked paths
/// and restors each file / directory to the
/// path where they need to be
fn restore_dotfiles() {
    todo!();
}

fn main() {
    // all the files / directories that dotmanager will keep track of
    // will be located at ~/dotfiles/.dotmanager

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
    }
}
