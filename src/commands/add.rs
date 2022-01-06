use crate::common::write_note;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct AddCommand {
    pub name: String,
    pub message: String,
    #[clap(long, default_value = "notes")]
    pub dir: PathBuf,
}

impl Run for AddCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("add command {:?}", self);

        let note_parent_dir = config.notes_dir.join(&self.dir);

        write_note(&note_parent_dir, &self.name, &self.message)?;

        Ok(())
    }
}
