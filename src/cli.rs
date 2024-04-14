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
        remove_by: String,
    },

    /// updates dotfiles folder
    Upd,

    /// restores all the dotfiles to their destination
    Rest,

    /// lists all the paths from the dotmanager
    List,
}
