use anyhow::Result;
use clap::{Parser, Subcommand};
use gnotes::commands::{AddCommand, ListCommand, NewCommand, RemoveCommand};
use gnotes::config::load_config;
use gnotes::run::Run;
use log::{debug, LevelFilter};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    debug: bool,
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    New(NewCommand),
    Add(AddCommand),
    Remove(RemoveCommand),
    Rm(RemoveCommand),
    List(ListCommand),
    Ls(ListCommand),
}

fn init_logger(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::builder().filter_level(level).init()
}

// TODO: use https://docs.rs/termimad/latest/termimad/ for show.
fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logger(cli.debug);

    debug!("cli options {:?}", cli);

    let home_dir = dirs::home_dir().expect("Unexpected error: home directory can't be located.");
    let config = load_config(&home_dir)?;

    debug!("loaded config {:?}", config);

    if let Some(command) = cli.command {
        match command {
            Command::New(new_command) => new_command.run(&config)?,
            Command::Add(add_command) => add_command.run(&config)?,
            Command::Remove(remove_command) | Command::Rm(remove_command) => {
                remove_command.run(&config)?
            }
            Command::List(list_command) | Command::Ls(list_command) => list_command.run(&config)?,
        }
    }

    Ok(())
}
