use anyhow::Result;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const TAGS_FILE_NAME: &'static str = ".tags";

pub type Tags = HashMap<String, HashSet<String>>;

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
