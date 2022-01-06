use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct RemoveCommand {
    pub name: String,
    #[clap(long, default_value = "notes")]
    pub dir: PathBuf,
}

impl Run for RemoveCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("remove command {:?}", self);

        let note_parent_dir = config.notes_dir.join(&self.dir);
        let note_file_path = note_parent_dir.join(&self.name);

        if note_file_path.exists() {
            fs::remove_file(note_file_path)?;
        }

        Ok(())
    }
}
