use clap::{Parser, Subcommand};
use gnotes::config::load_config;
use log::{debug, LevelFilter};
use std::fs;
use gnotes::commands::NewCommand;
use anyhow::Result;

const DEFAULT_NOTE_DIR: &'static str = "notes";

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
}

fn init_logger(debug: bool) {
    let level = if debug { LevelFilter::Debug } else { LevelFilter::Info };

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
            Command::New(new_command) => {
                debug!("new command {:?}", new_command);

                // TODO: can be shared
                let note_dir_name = new_command.dir.unwrap_or(String::from(DEFAULT_NOTE_DIR));
                let note_parent_dir = config.notes_dir.join(note_dir_name);
                let note_file_path = note_parent_dir.join(new_command.name);

                fs::create_dir_all(note_parent_dir)?;
                // end of TODO can be shared

                match new_command.message {
                    Some(message) => {
                        debug!("Writing message '{}' to {:?}", message, note_file_path);

                        fs::write(note_file_path, message)?;
                    },
                    _ => {
                        debug!("Opening editor for file {:?}", note_file_path);

                        edit::edit_file(note_file_path)?;
                    }
                }
            }
        }
    }

    Ok(())
}
