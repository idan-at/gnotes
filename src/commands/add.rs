use crate::common::notes::{resolve_dir, write_note};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct AddCommand {
    /// The name of the note
    pub name: String,
    /// The content to be added to the end of the note
    pub message: String,
    /// The note directory. defaults to "notes"
    #[clap(long)]
    pub dir: Option<PathBuf>,
}

impl Run for AddCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("add command {:?}", self);

        let dir = resolve_dir(&self.dir);
        let note_parent_dir = config.notes_dir.join(&dir);

        write_note(&note_parent_dir, &self.name, &self.message)?;

        Ok(())
    }
}
