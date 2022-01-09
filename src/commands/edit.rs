use crate::common::notes::resolve_dir;
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
    #[clap(long)]
    pub dir: Option<PathBuf>,
}

impl Run for EditCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("edit command {:?}", self);

        let dir = resolve_dir(&self.dir);
        let note_parent_dir = config.notes_dir.join(&dir);
        let note_file_path = note_parent_dir.join(&self.name);

        fs::create_dir_all(note_parent_dir)?;

        debug!("Opening editor for file {:?}", note_file_path);

        edit::edit_file(note_file_path)?;

        Ok(())
    }
}
