use colored::Colorize;
use home;
use std::{
    fs,
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
    pub fn add(&self, args: Vec<String>) {
        //-> Result<(),path::>{
        //Add:
        //Return result
        //	any, some sort of path error
        //
        //Path exists? Make the path
        let name = &args[0];
        let pth = Path::new(&args[1]);
        println!("{} {}", name, pth.display());
here
        if !pth.exists() {
            println!("{} exists", pth.display())
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
    pub fn populate_aliases(&self, contents: String) -> Vec<Alias> {
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

            if path.starts_with(self.home_dir.to_str().unwrap()) {
                path = path.replace(self.home_dir.to_str().unwrap(), "~");
            }

            aliases.push(Alias {
                name,
                path,
                is_commented,
            })
        }

        return aliases;
    }
}

/// Creates a new ProjectManager struct
pub fn new(alias_file: String) -> Option<PM> {
    let home_dir: path::PathBuf = match home::home_dir() {
        Some(pth) => pth,
        None => {
            eprintln!("Unable to get your home dir");
            return None;
        }
    };
    let contents = read_file(&alias_file);
    let mut pm = PM {
        home_dir,
        alias_file,
        aliases: vec![],
    };

    //TODO don't return alias vec. just call pm.populate and have it internally store
    pm.aliases = pm.populate_aliases(contents);

    return Some(pm);
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
