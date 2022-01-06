use crate::common::get_note_parent_dir;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;

#[derive(Debug, Parser)]
pub struct RemoveCommand {
    pub name: String,
    #[clap(long)]
    pub dir: Option<String>,
}

impl Run for RemoveCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("remove command {:?}", self);

        let note_parent_dir = get_note_parent_dir(&config, &self.dir);
        let note_file_path = note_parent_dir.join(&self.name);

        if note_file_path.exists() {
            fs::remove_file(note_file_path)?;
        }

        Ok(())
    }
}
