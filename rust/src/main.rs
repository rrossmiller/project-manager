use std::{env, process::exit};

use clap::{Parser, Subcommand};
use pm;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    // Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    //config: Option<PathBuf>,
    //
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    list: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add an alias to a project
    #[command(arg_required_else_help(true))]
    Add { args: Vec<String> },

    /// Disactivates an alias
    #[command(name("rem"), arg_required_else_help(true))]
    Remove,

    /// Deletes an alias
    #[command(name("del"), arg_required_else_help(true))]
    Delete,
}

fn main() {
    // Get the program options
    let cli = CLI::parse();

    let alias_file = String::from("aliasfile");
    let mut pm: pm::PM;
    if let Ok(x) = pm::new(alias_file) {
        pm = x;
    } else {
        exit(1);
    }

    // default action (no args): print all the known aliases
    if env::args().len() == 1 || cli.list {
        pm.print();
        exit(0);
    }

    match cli.command {
        Commands::Add { args } => {
            if args.len() != 2 {
                eprintln!("You passed in {} args. `add` takes 2 args", args.len());
                eprintln!("Only pass in <name> <path>");
                exit(1);
            }
            match pm.add(args) {
                Ok(_) => {
                    pm.write_alias_file();
                }
                Err(s) => {
                    eprintln!("{}", s);
                }
            };
        }
        Commands::Remove => println!("not yet implemented"),
        Commands::Delete => println!("not yet implemented"),
    }
}
