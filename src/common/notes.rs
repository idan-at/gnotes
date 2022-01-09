use anyhow::Result;
use log::debug;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

const DEFAULT_NOTES_DIR: &'static str = "notes";

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
