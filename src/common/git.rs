use anyhow::Result;
use git2::{Repository, Signature};
use std::path::Path;

const GNOTES_GIT_USER_NAME: &'static str = "gnotes";
const GNOTES_GIT_EMAIL: &'static str = "gnotes@gnotes.com";

fn open_repository(repository: &str) -> Result<Repository> {
    match Repository::open(repository) {
        Ok(repo) => Ok(repo),
        Err(_) => Ok(Repository::init(repository)?),
    }
}

pub fn commit_and_push(repository: &str, path: &Path, message: &str) -> Result<()> {
    let local_repository = open_repository(repository)?;

    let remotes_list = local_repository.remotes()?;
    // TODO: Replace expect with error (Option -> Result)
    let remote_name = remotes_list.get(0).expect("Failed to find remote");
    let mut remote = local_repository.find_remote(remote_name)?;

    let mut index = local_repository.index()?;
    index.add_path(path)?;
    index.write()?;

    let tree_id = index.write_tree()?;

    let signature = Signature::now(GNOTES_GIT_USER_NAME, GNOTES_GIT_EMAIL)?;

    local_repository.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &local_repository.find_tree(tree_id)?,
        &[],
    )?;

    remote.push::<&str>(&[], None)?;

    Ok(())
}
