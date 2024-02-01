use std::fs;
use std::fs::read_to_string;
use std::io::Write;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Available commands
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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

    /// lists all the paths from the dotmanager
    List,
}

/// creates .dotmanager inside ~/dotfiles
/// if it already exists it won't create it
fn create_dotmanager_file() -> bool {
    let path = home::home_dir().unwrap().join("dotfiles/.dotmanager");

    if !path.exists() {
        match fs::File::create(&path) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
    return false;
}

/// Adds the path to the files tracked
/// by the dotmanager
pub fn add_managed(path: String) {
    println!("Adding '{}' to the dotmanager", path);
    create_dotmanager_file();

    let manager_path = home::home_dir().unwrap().join("dotfiles/.dotmanager");
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&manager_path)
        .unwrap();

    file.write_all(format!("{}\n", path).as_bytes()).unwrap();
}

/// Removes the path from the
/// files tracked by the dotmanager
pub fn rmv_managed(path: String) {
    // TODO: remove also by index
    let manager_path = home::home_dir().unwrap().join("dotfiles/.dotmanager");

    let mut files = list_dotfiles().unwrap();
    files.retain(|s| *s != path);

    let content = files.join("\n");
    let content_bytes = content.as_bytes();

    std::fs::write(manager_path, content_bytes).unwrap();
}

/// Goes through all the tracked paths
/// and if there are changes to the file / directory
/// the file / directory will be copied to the
/// dotfiles directory
pub fn update_dotfiles() {
    todo!();
}

/// Goes through all the tracked paths
/// and restors each file / directory to the
/// path where they need to be
pub fn restore_dotfiles() {
    todo!();
}

/// reads the contents of the ~/.dotmanager
/// it the file doesn't exist it will create it
/// if it can't read it, will return None
pub fn list_dotfiles() -> Option<Vec<String>> {
    // TODO: list files in a tabel with index to allow for
    // removing by index
    let path = home::home_dir().unwrap().join("dotfiles/.dotmanager");
    create_dotmanager_file();
    let file = read_to_string(&path);
    if file.is_err() {
        return None;
    }
    let mut lines = Vec::new();
    for line in file.unwrap().lines() {
        lines.push(line.to_string());
    }
    Some(lines)
}
