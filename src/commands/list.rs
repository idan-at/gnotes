use crate::common::notes::resolve_dir;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use chrono::Datelike;
use clap::Parser;
use log::debug;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::time::SystemTime;
use tabular::{Row, Table};

fn format_system_time(system_time: SystemTime) -> String {
    let date_time: DateTime<Utc> = system_time.into();
    let now: DateTime<Utc> = SystemTime::now().into();

    if date_time.year() == now.year()
        && date_time.month() == now.month()
        && date_time.day() == now.day()
    {
        format!("{}", date_time.format("%H:%M"))
    } else {
        format!("{}", date_time.format("%b %e %H:%M"))
    }
}

#[derive(Debug, Parser)]
pub struct ListCommand {
    /// The note directory. defaults to "notes"
    #[clap(long)]
    pub dir: Option<PathBuf>,
    #[clap(long)]
    /// Include the table headers
    pub include_headers: bool,
    /// Display all notes, regardless of the directory
    #[clap(short, long)]
    pub all: bool,
}

impl ListCommand {
    fn list_notes(&self, config: &Config) -> Result<Vec<DirEntry>> {
        if self.all {
            let entries = self.list_entries_in(&config.notes_dir)?;

            let mut results = vec![];
            for entry in entries {
                results.extend(self.list_entries_in(&entry.path())?);
            }

            Ok(results)
        } else {
            let dir = resolve_dir(&self.dir);
            let note_parent_dir = config.notes_dir.join(&dir);

            self.list_entries_in(&note_parent_dir)
        }
    }

    fn build_table(&self, config: &Config, entries: Vec<DirEntry>) -> Result<Table> {
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
            let path = entry.path();
            let path = path.strip_prefix(&config.notes_dir)?.display();

            table.add_row(
                Row::new()
                    .with_cell(format_system_time(metadata.created()?))
                    .with_cell(metadata.len())
                    .with_cell(format_system_time(metadata.modified()?))
                    .with_cell(path),
            );
        }

        Ok(table)
    }

    fn list_entries_in(&self, dir: &Path) -> Result<Vec<DirEntry>> {
        let results = if dir.exists() && dir.metadata()?.is_dir() {
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

        if self.dir.is_some() && self.all {
            eprintln!("--dir can't be used with --all");

            process::exit(1);
        }

        let results = self.list_notes(config)?;
        let total = results.len();
        let table = self.build_table(config, results)?;

        println!("total {}", total);
        if total > 0 {
            println!("{}", table);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn format_system_time_not_today() {
        let system_time = SystemTime::UNIX_EPOCH;

        let formatted = format_system_time(system_time);

        assert_eq!(formatted, String::from("Jan  1 00:00"))
    }

    #[test]
    fn format_system_time_today() {
        let system_time = SystemTime::now();
        let now: DateTime<Utc> = SystemTime::now().into();

        let formatted = format_system_time(system_time);

        assert_eq!(formatted, format!("{:02}:{:02}", now.hour(), now.minute()))
    }
}
