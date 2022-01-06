use crate::common::{get_note_parent_dir, write_note};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;

#[derive(Debug, Parser)]
pub struct NewCommand {
    pub name: String,
    #[clap(long)]
    pub dir: Option<String>,
    #[clap(short, long)]
    pub message: Option<String>,
}

impl Run for NewCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("new command {:?}", self);

        let note_parent_dir = get_note_parent_dir(&config, &self.dir);

        match &self.message {
            Some(message) => write_note(&note_parent_dir, &self.name, &message)?,
            _ => {
                let note_file_path = note_parent_dir.join(&self.name);
                fs::create_dir_all(note_parent_dir)?;

                debug!("Opening editor for file {:?}", note_file_path);

                edit::edit_file(note_file_path)?;
            }
        }

        Ok(())
    }
}
