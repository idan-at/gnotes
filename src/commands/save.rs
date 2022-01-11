use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use clap::Parser;
use git2::{Repository, Signature};
use log::debug;
use std::process;
use std::time::SystemTime;

fn now() -> String {
    let date_time: DateTime<Utc> = SystemTime::now().into();

    format!("{}", date_time.format("%Y-%m-%d][%H:%M:%S"))
}

#[derive(Debug, Parser)]
pub struct SaveCommand {
    #[clap(short, long)]
    message: Option<String>,
}

impl SaveCommand {
    fn open_repository(&self, repository: &str) -> Result<Repository> {
        match Repository::open(repository) {
            Ok(repo) => Ok(repo),
            Err(_) => Ok(Repository::init(repository)?),
        }
    }
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
                let local_repository = self.open_repository(repository)?;
                // TODO: Handle errors (maybe use `.remotes` and take head?)
                let mut remote = local_repository.find_remote("origin")?;

                let mut index = local_repository.index()?;
                index.add_path(&config.notes_dir)?;
                index.write()?;
                let tree_id = index.write_tree()?;

                let signature = Signature::now("gnotes", "gnotes@gnotes.com")?;

                local_repository.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    &message,
                    &local_repository.find_tree(tree_id)?,
                    &[],
                )?;

                remote.push::<&str>(&[], None)?;
            }
            _ => {
                eprintln!("Can't save without a repository. Please specify a repository in the config file.");

                process::exit(1);
            }
        }

        Ok(())
    }
}
