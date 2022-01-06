use crate::config::Config;
use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use log::debug;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

const DEFAULT_NOTE_DIR: &'static str = "notes";

// TODO: Unit tests
pub fn format_system_time(system_time: SystemTime) -> String {
    let dt: DateTime<Utc> = system_time.into();

    // TODO: drop date when it's today.
    format!("{}", dt.format("%b %e %H:%M"))
}

// TODO: Unit tests
pub fn get_note_parent_dir(config: &Config, dir: &Option<String>) -> PathBuf {
    let note_dir_name = dir.clone().unwrap_or(String::from(DEFAULT_NOTE_DIR));

    config.notes_dir.join(note_dir_name)
}

pub fn write_note(note_parent_dir: &Path, note_file_name: &str, content: &str) -> Result<()> {
    let note_file_path = note_parent_dir.join(note_file_name);

    debug!("Writing message '{}' to {:?}", content, note_file_path);

    fs::create_dir_all(note_parent_dir)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(note_file_path)
        .unwrap();

    writeln!(file, "{}", content)?;

    Ok(())
}
