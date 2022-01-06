use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use clap::{Parser, Subcommand};
use gnotes::commands::{AddCommand, ListCommand, NewCommand, RemoveCommand};
use gnotes::config::{load_config, Config};
use log::{debug, LevelFilter};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tabular::{Row, Table};

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

fn format_system_time(system_time: SystemTime) -> String {
    let dt: DateTime<Utc> = system_time.into();

    // TODO: drop date when it's today.
    format!("{}", dt.format("%b %e %H:%M"))
}

fn get_note_parent_dir(config: &Config, dir: Option<String>) -> PathBuf {
    let note_dir_name = dir.unwrap_or(String::from(DEFAULT_NOTE_DIR));

    config.notes_dir.join(note_dir_name)
}

fn get_note_file_path(note_parent_dir: &Path, name: &str) -> PathBuf {
    note_parent_dir.join(name)
}

fn create_note(config: &Config, name: &str, dir: Option<String>) -> Result<PathBuf> {
    let note_parent_dir = get_note_parent_dir(config, dir);
    let note_file_path = get_note_file_path(&note_parent_dir, name);

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

                let note_parent_dir = get_note_parent_dir(&config, remove_command.dir);
                let note_file_path = get_note_file_path(&note_parent_dir, &remove_command.name);

                if note_file_path.exists() {
                    fs::remove_file(note_file_path)?;
                }
            }
            // TODO: Add flag for adding the header.
            Command::List(list_command) | Command::Ls(list_command) => {
                debug!("list command {:?}", list_command);

                let note_parent_dir = get_note_parent_dir(&config, list_command.dir);

                let result = if note_parent_dir.exists() {
                    fs::read_dir(note_parent_dir)?.collect()
                } else {
                    vec![]
                };
                let total = result.len();

                let mut table = Table::new("{:>} {:>} {:>} {:<}");
                for entry_result in result {
                    let entry = entry_result?;
                    let metadata = entry.metadata()?;

                    table.add_row(
                        Row::new()
                            .with_cell(format_system_time(metadata.created()?))
                            .with_cell(metadata.len())
                            .with_cell(format_system_time(metadata.modified()?))
                            .with_cell(entry.path().display()),
                    );
                }

                println!("total {}", total);
                if total > 0 {
                    println!("{}", table);
                }
            }
        }
    }

    Ok(())
}
