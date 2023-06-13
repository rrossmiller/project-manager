use colored::Colorize;
use home::{self, env::Env};
use std::{
    fs::{self, File},
    io::Write,
    path::{self, Path},
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
    pub fn add(&mut self, args: Vec<String>) -> Result<(), String> {
        let name = args[0].clone();
        let mut pth = args[1].clone();
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

    pub fn write_alias_file(&self) {
        if let Ok(mut alias_file) = File::create(&self.alias_file) {
            self.aliases.iter().for_each(|a| {
                let line = if a.is_commented { "#" } else { "" };
                let line = format!(
                    "{}alias pp{}='cd {} && clear ; work'\n",
                    line, a.name, a.path
                );

                alias_file.write_all(line.as_bytes()).unwrap();
            });

            println!("{}", fs::read_to_string(&self.alias_file).unwrap());
        }
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
                println!("{}", t);
            }
            // otherwise show it as normal
            else {
                println!(
                    "{}{}  {}",
                    a.name,
                    " ".repeat(max_len - a.name.len()),
                    a.path
                )
            }
        });
    }

    /// parse the lines of the alias_file and get the aliases
    pub fn populate_aliases(&mut self, contents: String) {
        let mut aliases = Vec::new();

        let lines: Vec<&str> = contents.lines().collect();
        let lines = &lines[1..];

        for line in lines {
            let mut is_commented = false;
            if line.len() == 0 {
                continue;
            }
            if line.chars().nth(0).unwrap() == '#' {
                is_commented = true;
            }

            let splits: Vec<&str> = line.split("=").collect();
            let name = splits[0].split(" ").nth(1).unwrap().replacen("pp", "", 1);
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
    let contents = read_file(&alias_file);
    let mut pm = PM {
        home_dir,
        alias_file,
        aliases: vec![],
    };

    pm.populate_aliases(contents);

    return Ok(pm);
}

/// Reads the alias file and returns the contents
fn read_file(alias_file: &String) -> String {
    match fs::read_to_string(&alias_file) {
        Ok(s) => return s,
        Err(_) => {
            eprintln!("No alias file exists. Creating: {}", &alias_file);
            fs::write(&alias_file, "").expect(format!("Error creating {}", &alias_file).as_str());
            eprintln!("Alias file created: ");
            return String::from("");
        }
    }
}
