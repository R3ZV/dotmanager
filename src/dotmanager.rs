use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct DotManager {
    dotfiles_path: PathBuf,
}

enum RemoveBy {
    PathName(String),
    Id(usize),
}

impl DotManager {
    pub fn new(home_dir: PathBuf) -> Self {
        let dotfiles_path = home_dir.join("dotfiles/.dotmanager");
        DotManager { dotfiles_path }
    }

    /// creates .dotmanager inside ~/dotfiles
    /// if it already exists it won't create it
    pub fn init(&self) -> Result<bool, Box<dyn Error>> {
        if !self.dotfiles_path.exists() {
            fs::File::create(&self.dotfiles_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Adds the path to the files tracked
    /// by the dotmanager
    pub fn add(&self, path: String) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&self.dotfiles_path)
            .unwrap();

        file.write_all(format!("{}\n", path).as_bytes()).unwrap();
    }
    /// Removes the path from the
    /// files tracked by the dotmanager
    pub fn rmv(&self, remove_by: &String) -> Result<bool, Box<dyn Error>> {
        let remove_by = match remove_by.parse::<usize>() {
            Ok(num) => RemoveBy::Id(num),
            Err(_) => RemoveBy::PathName(remove_by.to_string()),
        };
        let mut files = self.read()?;
        let initial_files_count = files.len();
        match remove_by {
            RemoveBy::PathName(name) => files.retain(|s| *s != name),
            RemoveBy::Id(index) => {
                files = files
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, v)| v.clone())
                    .collect()
            }
        }

        if files.len() == initial_files_count {
            Ok(false)
        } else {
            files.iter_mut().for_each(|file| file.push('\n'));

            let content = files.join("");
            std::fs::write(&self.dotfiles_path, content)?;
            Ok(true)
        }
    }

    /// Goes through all the tracked paths
    /// and if there are changes to the file / directory
    /// the file / directory will be copied to the
    /// dotfiles directory
    pub fn update(&self) {
        let files = self.read().unwrap();
        for file in files {
            if fs::metadata(&file).unwrap().is_dir() {
                self.copy_dir(PathBuf::from(file));
            } else {
                let new_path = file.replace(".config", "dotfiles");

                match fs::copy(&file, &new_path) {
                    Ok(_) => println!("Updated {}", &file),
                    Err(err) => println!("Couldn't copy {} due to {}", &file, err),
                }
            }
        }
    }

    fn copy_dir(&self, dir_path: PathBuf) {
        let tree = fs::read_dir(dir_path).unwrap();
        for entry in tree {
            let entry_path = String::from(entry.unwrap().path().to_string_lossy());
            if fs::metadata(&entry_path).unwrap().is_dir() {
                self.copy_dir(PathBuf::from(entry_path));
            } else {
                let new_path = entry_path.replace(".config", "dotfiles");
                match fs::copy(&entry_path, &new_path) {
                    Ok(_) => println!("Updated {:?}", &entry_path),
                    Err(err) => println!("Couldn't copy {:?} due to {}", &entry_path, err),
                }
            }
        }
    }

    /// Goes through all the tracked paths
    /// and restors each file / directory to the
    /// path where they need to be
    pub fn restore(&self) {
        todo!();
    }

    /// reads the contents of the ~/.dotmanager
    /// it the file doesn't exist it will create it
    /// if it can't read it, will return None
    pub fn read(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let file = fs::read_to_string(&self.dotfiles_path)?;
        Ok(file.lines().map(|line| line.to_string()).collect())
    }
}
