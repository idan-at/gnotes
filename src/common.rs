use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use chrono::Datelike;
use log::debug;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::time::SystemTime;

// TODO: Split to multiple modules.

const DEFAULT_NOTES_DIR: &'static str = "notes";
const TAGS_FILE_NAME: &'static str = ".tags";

pub type Tags = HashMap<String, HashSet<String>>;

pub fn format_system_time(system_time: SystemTime) -> String {
    let date_time: DateTime<Utc> = system_time.into();
    let now: DateTime<Utc> = SystemTime::now().into();

    if date_time.year() == now.year()
        && date_time.month() == now.month()
        && date_time.day() == now.day()
    {
        format!("{}", date_time.format("%H:%M"))
    } else {
        format!("{}", date_time.format("%b %e %H:%M"))
    }
}

pub fn resolve_dir(dir: &Option<PathBuf>) -> PathBuf {
    match dir {
        Some(dir) => dir.clone(),
        _ => PathBuf::from(DEFAULT_NOTES_DIR),
    }
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

fn assert_note_exists(command: &str, notes_dir: &Path, note_relative_path: &Path) {
    let note_file_path = notes_dir.join(&note_relative_path);

    if !note_file_path.exists() {
        eprintln!(
            "{} failed: file '{}' not found",
            command,
            String::from(note_file_path.to_string_lossy())
        );

        process::exit(1);
    }
}

pub fn get_note_identifier(command: &str, notes_dir: &Path, name: &str, dir: &Path) -> String {
    let note_relative_path = dir.join(name);

    assert_note_exists(command, notes_dir, &note_relative_path);

    String::from(note_relative_path.to_string_lossy())
}

pub fn write_as_markdown(notes_dir: &Path, note_identifier: &str) -> Result<()> {
    let note_file_path = notes_dir.join(note_identifier);
    let content = fs::read_to_string(note_file_path)?;

    println!("{}:", note_identifier);
    termimad::print_text(&content);

    Ok(())
}

pub fn load_tags(notes_dir: &Path) -> Result<Tags> {
    let tags_file_path = notes_dir.join(TAGS_FILE_NAME);

    let tags = if tags_file_path.exists() {
        let data = fs::read_to_string(&tags_file_path)?;

        serde_json::from_str::<Tags>(&data)?
    } else {
        HashMap::new()
    };

    Ok(tags)
}

pub fn update_tags<T: Serialize>(notes_dir: &Path, tags: &T) -> Result<()> {
    let tags_file_path = notes_dir.join(TAGS_FILE_NAME);

    fs::write(tags_file_path, serde_json::to_string(tags)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn resolve_dir_default() {
        let expected = PathBuf::from("notes");

        let dir = None;

        assert_eq!(resolve_dir(&dir), expected);
    }

    #[test]
    fn resolve_dir_custom() {
        let expected = PathBuf::from("something");

        let dir = Some(expected.clone());

        assert_eq!(resolve_dir(&dir), expected);
    }

    #[test]
    fn format_system_time_not_today() {
        let system_time = SystemTime::UNIX_EPOCH;

        let formatted = format_system_time(system_time);

        assert_eq!(formatted, String::from("Jan  1 00:00"))
    }

    #[test]
    fn format_system_time_today() {
        let system_time = SystemTime::now();
        let now: DateTime<Utc> = SystemTime::now().into();

        let formatted = format_system_time(system_time);

        assert_eq!(formatted, format!("{:02}:{:02}", now.hour(), now.minute()))
    }
}
