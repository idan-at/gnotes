mod setup;

use crate::setup::{GitSetup, Setup};
use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_save_fails_without_repository() -> Result<()> {
    Command::cargo_bin("gnotes")?
        .args(vec!["save"])
        .assert()
        .stderr(predicate::eq(
            "Can't save without a repository. Please specify a repository in the config file.\n",
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_save_succeeds_on_existing_repository() -> Result<()> {
    let setup = Setup::new()?;
    let git_setup = GitSetup::new()?;

    GitSetup::clone_to(git_setup.bare_dir.path(), setup.dir.path())?;

    fs::write(
        GitSetup::build_note_path(setup.dir.path()),
        "updated file content\n",
    )?;

    // TODO: Provide id_rsa explicitly (ATM this assumes ~/.ssh/id_rsa exists)
    Command::cargo_bin("gnotes")?
        .args(vec!["save"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .env("GNOTES_REPOSITORY", git_setup.bare_dir.path())
        .assert()
        .code(0);

    git_setup.update_clone()?;

    assert_eq!(
        fs::read_to_string(GitSetup::build_note_path(git_setup.clone_dir.path()))?,
        "updated file content\n"
    );

    Ok(())
}
