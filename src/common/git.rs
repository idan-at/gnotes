use anyhow::{Context, Result};
use git2::{IndexAddOption, Repository, Signature};
use std::path::Path;

const GNOTES_GIT_USER_NAME: &'static str = "gnotes";
const GNOTES_GIT_EMAIL: &'static str = "gnotes@gnotes.com";

fn open_repository(notes_path: &Path, remote: &str) -> Result<Repository> {
    let repository = match Repository::open(notes_path) {
        Ok(repository) => repository,
        Err(_) => {
            let repository = Repository::init(notes_path)?;

            // TODO: handle the case no remote exists / remote name isn't origin.
            repository.remote_set_url("origin", remote)?;

            repository
        }
    };

    Ok(repository)
}

pub fn commit_and_push(notes_path: &Path, remote: &str, message: &str) -> Result<()> {
    let repository = open_repository(notes_path, remote)?;

    let remotes_list = repository.remotes()?;
    let remote_name = remotes_list.get(0).context("Failed to find remote")?;
    let mut remote = repository.find_remote(remote_name)?;

    let mut index = repository.index()?;
    index.add_all(["."].iter(), IndexAddOption::CHECK_PATHSPEC, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;

    let signature = Signature::now(GNOTES_GIT_USER_NAME, GNOTES_GIT_EMAIL)?;

    match repository.head().ok().and_then(|h| h.target()) {
        Some(parent) => {
            let parent = repository
                .find_commit(parent)
                .context("Failed to find commit for parent")?;

            repository.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &repository.find_tree(tree_id)?,
                &[&parent],
            )?;
        }
        _ => {
            repository.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &repository.find_tree(tree_id)?,
                &[],
            )?;
        }
    };

    // TODO: find the ref dynamically?
    remote.push::<&str>(&["refs/heads/master"], None)?;

    Ok(())
}
