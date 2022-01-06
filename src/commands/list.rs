use crate::common::{format_system_time, get_note_parent_dir};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use tabular::{Row, Table};

#[derive(Debug, Parser)]
pub struct ListCommand {
    #[clap(long)]
    pub dir: Option<String>,
    #[clap(long)]
    pub include_headers: bool,
    #[clap(short, long)]
    pub all: bool,
}

impl ListCommand {
    fn list_notes(&self, config: &Config) -> Result<Vec<DirEntry>> {
        if self.all {
            let dirs = self.list_entries_in(&config.notes_dir)?;

            let mut results = vec![];
            for dir in dirs {
                let entries = self.list_entries_in(&dir.path())?;

                results.extend(entries);
            }

            Ok(results)
        } else {
            let note_parent_dir = get_note_parent_dir(config, &self.dir);

            self.list_entries_in(&note_parent_dir)
        }
    }

    fn build_table(&self, entries: Vec<DirEntry>) -> Result<Table> {
        let mut table = Table::new("{:<} {:<} {:<} {:<}");

        if self.include_headers {
            table.add_row(
                Row::new()
                    .with_cell("Created")
                    .with_cell("Length")
                    .with_cell("Updated")
                    .with_cell("Path"),
            );
        }

        for entry in entries {
            let metadata = entry.metadata()?;

            table.add_row(
                Row::new()
                    .with_cell(format_system_time(metadata.created()?))
                    .with_cell(metadata.len())
                    .with_cell(format_system_time(metadata.modified()?))
                    .with_cell(entry.path().display()),
            );
        }

        Ok(table)
    }

    fn list_entries_in(&self, dir: &Path) -> Result<Vec<DirEntry>> {
        let results = if dir.exists() {
            fs::read_dir(dir)?.filter_map(|entry| entry.ok()).collect()
        } else {
            vec![]
        };

        Ok(results)
    }
}

impl Run for ListCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("list command {:?}", self);

        let results = self.list_notes(config)?;
        let total = results.len();
        let table = self.build_table(results)?;

        println!("total {}", total);
        if total > 0 {
            println!("{}", table);
        }

        Ok(())
    }
}
