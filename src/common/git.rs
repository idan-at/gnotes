use anyhow::{Context, Result};
use git2::{Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature};
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

pub fn auth_callbacks(ssh_file_path: &Path) -> RemoteCallbacks {
    let mut callbacks = RemoteCallbacks::new();

    // TODO: This part is not covered in the clone/save tests.
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.expect("Failed to extract username from git repository"),
            None,
            ssh_file_path,
            None,
        )
    });

    callbacks
}

pub fn commit_and_push(
    notes_path: &Path,
    ssh_file_path: &Path,
    remote: &str,
    message: &str,
) -> Result<()> {
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

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(auth_callbacks(ssh_file_path));

    // TODO: find the ref dynamically?
    remote.push::<&str>(&["refs/heads/main"], Some(&mut push_options))?;

    Ok(())
}
