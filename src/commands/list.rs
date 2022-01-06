use crate::common::{format_system_time, get_note_parent_dir};
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs;
use tabular::{Row, Table};

// TODO: What about list all?
#[derive(Debug, Parser)]
pub struct ListCommand {
    #[clap(long)]
    pub dir: Option<String>,
    #[clap(long)]
    pub include_headers: bool
}

impl Run for ListCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("list command {:?}", self);

        let note_parent_dir = get_note_parent_dir(&config, &self.dir);

        let result = if note_parent_dir.exists() {
            fs::read_dir(note_parent_dir)?.collect()
        } else {
            vec![]
        };
        let total = result.len();

        let mut table = Table::new("{:<} {:<} {:<} {:<}");

        if self.include_headers {
            table.add_row(Row::new().with_cell("Created").with_cell("Length").with_cell("Updated").with_cell("Path"));
        }

        for entry_result in result {
            let entry = entry_result?;
            let metadata = entry.metadata()?;

            table.add_row(
                Row::new()
                    .with_cell(format_system_time(metadata.created()?))
                    .with_cell(metadata.len())
                    .with_cell(format_system_time(metadata.modified()?))
                    .with_cell(entry.path().display()),
            );
        }

        println!("total {}", total);
        if total > 0 {
            println!("{}", table);
        }

        Ok(())
    }
}
