use anyhow::Result;
use clap::{Parser, Subcommand};
use gnotes::commands::{
    AddCommand, CloneCommand, EditCommand, ListCommand, NewCommand, RemoveCommand, SaveCommand,
    SearchCommand, ShowCommand, TagCommand, UntagCommand,
};
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
    Show(ShowCommand),
    Edit(EditCommand),
    Tag(TagCommand),
    Untag(UntagCommand),
    Search(SearchCommand),
    Clone(CloneCommand),
    Save(SaveCommand),
}

fn init_logger(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::builder().filter_level(level).init()
}

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
            Command::Show(show_command) => show_command.run(&config)?,
            Command::Edit(edit_command) => edit_command.run(&config)?,
            Command::Tag(tag_command) => tag_command.run(&config)?,
            Command::Untag(untag_command) => untag_command.run(&config)?,
            Command::Search(search_command) => search_command.run(&config)?,
            Command::Clone(clone_command) => clone_command.run(&config)?,
            Command::Save(save_command) => save_command.run(&config)?,
        }
    }

    Ok(())
}
