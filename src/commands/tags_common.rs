use crate::config::Config;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process;

const TAGS_FILE_NAME: &'static str = ".tags";

pub type Tags = HashMap<String, HashSet<String>>;

fn assert_note_exists(command: &str, config: &Config, note_relative_path: &Path) {
    let note_file_path = config.notes_dir.join(&note_relative_path);

    if !note_file_path.exists() {
        eprintln!(
            "{} failed: file '{}' not found",
            command,
            String::from(note_file_path.to_string_lossy())
        );

        process::exit(1);
    }
}

pub fn get_note_identifier(command: &str, config: &Config, name: &str, dir: &Path) -> String {
    let note_relative_path = dir.join(name);

    assert_note_exists(command, config, &note_relative_path);

    String::from(note_relative_path.to_string_lossy())
}

pub fn load_tags(config: &Config) -> Result<Tags> {
    let tags_file_path = config.notes_dir.join(TAGS_FILE_NAME);

    let tags = if tags_file_path.exists() {
        let data = fs::read_to_string(&tags_file_path)?;

        serde_json::from_str::<Tags>(&data)?
    } else {
        HashMap::new()
    };

    Ok(tags)
}

pub fn update_tags(config: &Config, tags: &Tags) -> Result<()> {
    let tags_file_path = config.notes_dir.join(TAGS_FILE_NAME);

    fs::write(tags_file_path, serde_json::to_string(tags)?)?;

    Ok(())
}
