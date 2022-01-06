use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct EditCommand {
    pub name: String,
    #[clap(long, default_value = "notes")]
    pub dir: PathBuf,
}

impl Run for EditCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("edit command {:?}", self);

        let note_parent_dir = config.notes_dir.join(&self.dir);
        let note_file_path = note_parent_dir.join(&self.name);

        fs::create_dir_all(note_parent_dir)?;

        debug!("Opening editor for file {:?}", note_file_path);

        edit::edit_file(note_file_path)?;

        Ok(())
    }
}
