use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;
use std::process;
use log::debug;

#[derive(Debug, Parser)]
pub struct CloneCommand {}

const MISSING_REPOSITORY_ERROR_MESSAGE: &'static str = "Can't clone without a repository. Please specify a repository in the config file.";

impl CloneCommand {
    fn clone(&self, repository: &str, ssh_file_path: &Path, to: &Path) -> Result<Repository> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                ssh_file_path,
                None,
            )
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        Ok(builder.clone(repository, to)?)
    }
}

impl Run for CloneCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("clone command {:?}", self);

        match &config.repository {
            Some(repository) => {
                self.clone(&repository, &config.ssh_file_path, &config.notes_dir)?;
            }
            _ => {
                eprintln!("{}", MISSING_REPOSITORY_ERROR_MESSAGE);

                process::exit(1);
            }
        }

        Ok(())
    }
}
