use clap::{CommandFactory, Parser, Subcommand};
use pm::PM;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
enum Commands {
    /// List your projects
    #[command(short_flag('l'))]
    List { startup: Option<bool> },

    /// Add an alias to a project
    #[command(short_flag('a'), arg_required_else_help(true))]
    Add {
        #[arg(value_name = "NAME")]
        name: String,
        #[arg(value_name = "PATH")]
        path: String,
    },

    /// Disactivates an alias
    #[command(short_flag('t'), arg_required_else_help(true))]
    Toggle { name: String },

    /// Deletes an alias
    #[command(short_flag('d'), arg_required_else_help(true))]
    Delete { name: String },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    // Get the program options
    let cli = CLI::parse();
    let alias_file = ".project-aliases";
    let mut pm: pm::PM;
    if let Ok(x) = PM::new(alias_file) {
        pm = x;
    } else {
        exit(1);
    }

    match cli.command {
        Commands::List { startup } => match startup {
            Some(_) => pm.print_terminal(),
            None => pm.print(),
        },

        Commands::Add { name, path } => {
            match pm.add(name, path) {
                Ok(_) => pm.write_alias_file(),
                Err(s) => eprintln!("{}", s),
            };
        }

        Commands::Toggle { name } => {
            pm.toggle(name);
            pm.write_alias_file();
        }

        Commands::Delete { name } => {
            pm.delete(name);
            pm.write_alias_file();
        }
        // e.g. `$ cli completions bash`
        Commands::Completions { shell } => {
            shell.generate(&mut CLI::command(), &mut std::io::stdout());
        }
    }
}
