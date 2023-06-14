use colored::Colorize;
use home;
use std::{
    fs,
    path::{self, Path, PathBuf},
};

/// Represents and alias
#[derive(Debug)]
pub struct Alias {
    name: String,
    path: String,
    is_commented: bool,
}

/// The project manager
pub struct PM {
    home_dir: path::PathBuf,
    alias_file: String,
    aliases: Vec<Alias>,
}

impl PM {
    /// Add a new alias
    pub fn add(&mut self, name: String, mut pth: String) -> Result<(), String> {
        let path = Path::new(&pth);

        if !path.exists() {
            return Err(format!("path doesn't exist: {}", pth));
        }

        pth = self.replace_home_dir(pth);

        self.aliases.push(Alias {
            name,
            path: pth,
            is_commented: false,
        });

        return Ok(());
    }
    pub fn toggle(&mut self, name: String) {
        // mark for ignore the alias
        for a in self.aliases.iter_mut() {
            if a.name.eq(&name) {
                a.is_commented = !a.is_commented;
            }
        }
    }

    pub fn delete(&mut self, name: String) {
        self.aliases = self
            .aliases
            .drain(..)
            .filter(|a| !a.name.eq(&name))
            .collect();
    }

    // ------

    pub fn write_alias_file(&self) {
        let mut lines = String::from("");
        self.aliases.iter().for_each(|a| {
            let line = if a.is_commented { "#" } else { "" };
            let line = format!(
                "{}alias pp{}='cd {} && clear ; work'\n",
                line, a.name, a.path
            );
            lines.push_str(&line);
        });
        let path = format!("{}/{}", self.home_dir.to_str().unwrap(), self.alias_file);
        fs::write(path, lines).unwrap();
    }

    /// pretty print the known and ignored aliases
    pub fn print(&self) {
        // get the longest name (for padding)
        let mut max_len = 0;
        self.aliases.iter().for_each(|a| {
            max_len = if a.name.len() > max_len {
                a.name.len()
            } else {
                max_len
            }
        });

        self.aliases.iter().for_each(|a| {
            // if it's commented out, that means it's cached, but not active
            // show as strikeout
            if a.is_commented {
                let t = format!(
                    "{}{}  {}",
                    a.name,
                    " ".repeat(max_len - a.name.len()),
                    a.path
                )
                .strikethrough();
                eprintln!("{}", t);
            }
            // otherwise show it as normal
            else {
                eprintln!(
                    "{}{}  {}",
                    a.name,
                    " ".repeat(max_len - a.name.len()),
                    a.path
                )
            }
        });
    }

    pub fn print_terminal(&self) {
        println!("Projects:");
        self.aliases.iter().for_each(|a| {
            // if it's commented out, that means it's cached, but not active
            // show as strikeout
            if !a.is_commented {
                println!("  - {}", a.name);
            }
        });
    }

    /// parse the lines of the alias_file and get the aliases
    pub fn populate_aliases(&mut self, contents: String) {
        let mut aliases = Vec::new();

        let lines: Vec<&str> = contents.lines().collect();
        let lines = &lines[0..];

        for line in lines {
            let mut is_commented = false;
            if line.len() == 0 {
                continue;
            }
            if line.chars().nth(0).unwrap() == '#' {
                is_commented = true;
            }

            let splits: Vec<&str> = line.split("=").collect();
            let name = splits[0]
                .split(" ")
                .nth(1)
                .unwrap()
                .replacen("pp", "", 1)
                .replacen("#", "", 1);
            let mut path = splits[1].split(" ").nth(1).unwrap().to_string();

            path = self.replace_home_dir(path);

            aliases.push(Alias {
                name,
                path,
                is_commented,
            })
        }
        self.aliases = aliases;
    }

    fn replace_home_dir(&self, mut path: String) -> String {
        let home_dir = self.home_dir.to_str().unwrap();
        if path.starts_with(home_dir) {
            path = path.replace(home_dir, "~");
        }
        path
    }
}

/// Creates a new ProjectManager struct
pub fn new(alias_file: String) -> Result<PM, ()> {
    let home_dir: path::PathBuf = match home::home_dir() {
        Some(pth) => pth,
        None => {
            eprintln!("Unable to get your home dir");
            return Err(());
        }
    };
    let contents = read_file(&alias_file, &home_dir);
    let mut pm = PM {
        home_dir,
        alias_file,
        aliases: vec![],
    };

    pm.populate_aliases(contents);

    return Ok(pm);
}

/// Reads the alias file and returns the contents
fn read_file(alias_file: &String, home_dir: &PathBuf) -> String {
    let path = format!("{}/{}", home_dir.to_str().unwrap(), alias_file);
    match fs::read_to_string(&path) {
        Ok(s) => return s,
        Err(_) => {
            eprintln!("No alias file exists. Creating: {}", path);
            match fs::File::create(path) {
                Err(e) => {
                    panic!("{}", e);
                }
                _ => "",
            };
            eprintln!("Alias file created: ");
            return String::from("");
        }
    }
}
