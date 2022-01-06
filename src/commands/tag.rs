use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

const TAGS_FILE_NAME: &'static str = ".tags";

#[derive(Debug, Parser)]
pub struct TagCommand {
    pub name: String,
    pub tags: Vec<String>,
    #[clap(long, default_value = "notes")]
    pub dir: PathBuf,
}

type Tags = HashMap<String, HashSet<String>>;

impl TagCommand {
    fn assert_note_exists(&self, config: &Config, note_relative_path: &Path) {
        let note_file_path = config.notes_dir.join(&note_relative_path);

        if !note_file_path.exists() {
            eprintln!(
                "tag failed: file '{}' not found",
                String::from(note_file_path.to_string_lossy())
            );

            process::exit(1);
        }
    }

    fn load_tags(&self, config: &Config) -> Result<Tags> {
        let tags_file_path = config.notes_dir.join(TAGS_FILE_NAME);

        let tags = if tags_file_path.exists() {
            let data = fs::read_to_string(&tags_file_path)?;

            serde_json::from_str::<Tags>(&data)?
        } else {
            HashMap::new()
        };

        Ok(tags)
    }
}

impl Run for TagCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("tag command {:?}", self);

        let note_relative_path = self.dir.join(&self.name);

        self.assert_note_exists(config, &note_relative_path);

        let note_identifier = String::from(note_relative_path.to_string_lossy());
        let tags_file_path = config.notes_dir.join(TAGS_FILE_NAME);

        let mut tags = self.load_tags(config)?;

        for tag in &self.tags {
            match tags.get_mut(tag) {
                Some(tags_list) => {
                    tags_list.insert(note_identifier.clone());
                }
                _ => {
                    tags.insert(
                        tag.clone(),
                        HashSet::from_iter(vec![note_identifier.clone()]),
                    );
                }
            };
        }

        fs::write(tags_file_path, serde_json::to_string(&tags)?)?;

        Ok(())
    }
}
