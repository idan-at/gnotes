use crate::common::notes::{get_note_identifier, resolve_dir};
use crate::common::tags::{load_tags, update_tags};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct TagCommand {
    /// The name of the note
    pub name: String,
    /// The list of tags
    pub tags: Vec<String>,
    /// The note directory. defaults to "notes"
    #[clap(long)]
    pub dir: Option<PathBuf>,
}

impl Run for TagCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("tag command {:?}", self);

        let dir = resolve_dir(&self.dir);
        let note_identifier = get_note_identifier("tag", &config.notes_dir, &self.name, &dir);

        let mut tags = load_tags(&config.notes_dir)?;

        for tag in &self.tags {
            match tags.get_mut(tag) {
                Some(tags_set) => {
                    tags_set.insert(note_identifier.clone());
                }
                _ => {
                    tags.insert(
                        tag.clone(),
                        HashSet::from_iter(vec![note_identifier.clone()]),
                    );
                }
            };
        }

        update_tags(&config.notes_dir, &tags)?;

        Ok(())
    }
}
