use crate::commands::tags_common::{get_note_identifier, load_tags, update_tags};
use crate::common::resolve_dir;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct UntagCommand {
    pub name: String,
    pub tag: String,
    #[clap(long)]
    pub dir: Option<PathBuf>,
}

impl Run for UntagCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("untag command {:?}", self);

        let dir = resolve_dir(&self.dir);
        let note_identifier = get_note_identifier("untag", config, &self.name, &dir);

        let mut tags = load_tags(config)?;

        match tags.get_mut(&self.tag) {
            Some(tags_set) => {
                if tags_set.contains(&note_identifier) {
                    if tags_set.len() > 1 {
                        tags_set.remove(&note_identifier);
                    } else {
                        tags.remove(&self.tag);
                    }
                }
            }
            _ => {}
        }

        update_tags(config, &tags)?;

        Ok(())
    }
}
