use crate::common::{get_note_identifier, load_tags, resolve_dir, update_tags};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct TagCommand {
    pub name: String,
    pub tags: Vec<String>,
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
