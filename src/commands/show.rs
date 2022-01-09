use crate::common::resolve_dir;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;
use std::path::PathBuf;
use std::process;
use termimad::{Area, MadSkin, MadView};

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
        let note_parent_dir = config.notes_dir.join(&dir);
        let note_file_path = note_parent_dir.join(&self.name);

        if note_file_path.exists() {
            let content = fs::read_to_string(note_file_path)?;

            let area = Area::new(0, 1, 80, 10);
            let view = MadView::from(content, area, MadSkin::default());
            view.write().unwrap();
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
