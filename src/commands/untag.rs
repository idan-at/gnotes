use crate::common::{get_note_identifier, load_tags, resolve_dir, update_tags};
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
        let note_identifier = get_note_identifier("untag", &config.notes_dir, &self.name, &dir);

        let mut tags = load_tags(&config.notes_dir)?;

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

        update_tags(&config.notes_dir, &tags)?;

        Ok(())
    }
}
