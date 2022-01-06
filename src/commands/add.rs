use crate::common::{get_note_parent_dir, write_note};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;

#[derive(Debug, Parser)]
pub struct AddCommand {
    pub name: String,
    pub message: String,
    #[clap(long)]
    pub dir: Option<String>,
}

impl Run for AddCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("add command {:?}", self);

        let note_parent_dir = get_note_parent_dir(&config, &self.dir);
        write_note(&note_parent_dir, &self.name, &self.message)?;

        Ok(())
    }
}
