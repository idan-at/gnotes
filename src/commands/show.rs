use crate::common::notes::{get_note_identifier, resolve_dir};
use crate::common::writers::write_as_markdown;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
pub struct ShowCommand {
    pub name: String,
    #[clap(long)]
    pub dir: Option<PathBuf>,
}

impl Run for ShowCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("show command {:?}", self);

        let dir = resolve_dir(&self.dir);
        let note_file_path = config.notes_dir.join(&dir).join(&self.name);
        let note_identifier = get_note_identifier("show", &config.notes_dir, &self.name, &dir);

        if note_file_path.exists() {
            write_as_markdown(&config.notes_dir, &note_identifier)?;
        } else {
            eprintln!(
                "show failed: file '{}' not found",
                String::from(note_file_path.to_string_lossy())
            );

            process::exit(1);
        }

        Ok(())
    }
}
