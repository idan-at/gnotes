use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn write_as_markdown(notes_dir: &Path, note_identifier: &str) -> Result<()> {
    let note_file_path = notes_dir.join(note_identifier);
    let content = fs::read_to_string(note_file_path)?;

    println!("{}:", note_identifier);
    termimad::print_text(&content);

    Ok(())
}
