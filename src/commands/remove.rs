use crate::common::{get_note_identifier, load_tags, resolve_dir, update_tags, Tags};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct RemoveCommand {
    pub name: String,
    #[clap(long)]
    pub dir: Option<PathBuf>,
}

impl Run for RemoveCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("remove command {:?}", self);

        let dir = resolve_dir(&self.dir);
        let note_parent_dir = config.notes_dir.join(&dir);
        let note_file_path = note_parent_dir.join(&self.name);

        if note_file_path.exists() {
            let note_identifier =
                get_note_identifier("remove", &config.notes_dir, &self.name, &dir);

            fs::remove_file(note_file_path)?;

            let tags = load_tags(&config.notes_dir)?;
            let mut new_tags: Tags = HashMap::new();

            for (tag, tags_set) in tags {
                if tags_set.contains(&note_identifier) {
                    let mut update_tags_set = tags_set.clone();

                    update_tags_set.remove(&note_identifier);

                    if update_tags_set.len() > 0 {
                        new_tags.insert(tag.clone(), update_tags_set);
                    }
                } else {
                    new_tags.insert(tag.clone(), tags_set.clone());
                }
            }

            update_tags(&config.notes_dir, &new_tags)?;
        }

        Ok(())
    }
}
