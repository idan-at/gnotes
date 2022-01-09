use crate::run::Run;
use crate::config::Config;
use clap::Parser;
use anyhow::Result;
use log::debug;
use std::process;

#[derive(Debug, Parser)]
pub struct SaveCommand {}

impl Run for SaveCommand {
  fn run(&self, config: &Config) -> Result<()> {
    debug!("save command {:?}", self);

    match &config.repository {
      Some(repository) => {
          println!("{} ok!", repository);
      }
      _ => {
        eprintln!("Can't save without a repository. Please specify a repository in the config file.");

        process::exit(1);
      }
  }

    Ok(())
  }
}