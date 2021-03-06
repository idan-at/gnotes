use crate::common::git::commit_and_push;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use clap::Parser;
use log::debug;
use std::process;
use std::time::SystemTime;

fn now() -> String {
    let date_time: DateTime<Utc> = SystemTime::now().into();

    format!("{}", date_time.format("[%Y-%m-%d][%H:%M:%S]"))
}

#[derive(Debug, Parser)]
pub struct SaveCommand {
    /// The commit message
    #[clap(short, long)]
    message: Option<String>,
}

impl Run for SaveCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("save command {:?}", self);

        let message = self
            .message
            .clone()
            .unwrap_or(format!("gnotes manual save {}", now()));

        match &config.repository {
            Some(repository) => {
                commit_and_push(
                    &config.notes_dir,
                    &config.ssh_file_path,
                    repository,
                    &message,
                )?;
            }
            _ => {
                eprintln!("Can't save without a repository. Please specify a repository in the config file.");

                process::exit(1);
            }
        }

        Ok(())
    }
}
