use crate::commands::tags_common::{get_note_identifier, load_tags, update_tags};
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
    #[clap(long, default_value = "notes")]
    pub dir: PathBuf,
}

impl Run for TagCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("tag command {:?}", self);

        let note_identifier = get_note_identifier("tag", config, &self.name, &self.dir);

        let mut tags = load_tags(config)?;

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

        update_tags(config, &tags)?;

        Ok(())
    }
}
