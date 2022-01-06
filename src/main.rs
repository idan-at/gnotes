use anyhow::Result;
use clap::{Parser, Subcommand};
use gnotes::commands::{AddCommand, NewCommand, RemoveCommand};
use gnotes::config::{load_config, Config};
use log::{debug, LevelFilter};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

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
    Add(AddCommand),
    Remove(RemoveCommand),
    Rm(RemoveCommand),
}

fn init_logger(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::builder().filter_level(level).init()
}

fn build_note_file_path(config: &Config, name: &str, dir: Option<String>) -> (PathBuf, PathBuf) {
    let note_dir_name = dir.unwrap_or(String::from(DEFAULT_NOTE_DIR));
    let note_parent_dir = config.notes_dir.join(note_dir_name);
    let note_file_path = note_parent_dir.join(name);

    (note_parent_dir, note_file_path)
}

fn create_note(config: &Config, name: &str, dir: Option<String>) -> Result<PathBuf> {
    let (note_parent_dir, note_file_path) = build_note_file_path(config, name, dir);

    fs::create_dir_all(note_parent_dir)?;

    Ok(note_file_path)
}

fn write_note(note_file_path: &Path, content: &str) -> Result<()> {
    debug!("Writing message '{}' to {:?}", content, note_file_path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(note_file_path)
        .unwrap();

    writeln!(file, "{}", content)?;

    Ok(())
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

                let note_file_path = create_note(&config, &new_command.name, new_command.dir)?;

                match new_command.message {
                    Some(message) => write_note(&note_file_path, &message)?,
                    _ => {
                        debug!("Opening editor for file {:?}", note_file_path);

                        edit::edit_file(note_file_path)?;
                    }
                }
            }
            Command::Add(add_command) => {
                debug!("add command {:?}", add_command);

                let note_file_path = create_note(&config, &add_command.name, add_command.dir)?;

                write_note(&note_file_path, &add_command.message)?;
            }
            Command::Remove(remove_command) | Command::Rm(remove_command) => {
                debug!("remove command {:?}", remove_command);

                let (_, note_file_path) =
                    build_note_file_path(&config, &remove_command.name, remove_command.dir);

                if note_file_path.exists() {
                    fs::remove_file(note_file_path)?;
                }
            }
        }
    }

    Ok(())
}
