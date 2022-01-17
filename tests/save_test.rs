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
#[ignore]
fn test_save_succeeds() -> Result<()> {
    let setup = Setup::new()?;
    let git_setup = GitSetup::new()?;

    // TODO: Provide id_rsa explicitly (ATM this assumes ~/.ssh/id_rsa exists)
    Command::cargo_bin("gnotes")?
        .args(vec!["clone"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .env("GNOTES_REPOSITORY", &git_setup.repo_path)
        .assert()
        .code(0);

    fs::write(&setup.default_note_path(), "updated file content\n")?;

    // TODO: Provide id_rsa explicitly (ATM this assumes ~/.ssh/id_rsa exists)
    Command::cargo_bin("gnotes")?
        .args(vec!["save"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .env("GNOTES_REPOSITORY", &git_setup.repo_path)
        .assert()
        .code(0);

    let file_content = fs::read_to_string(&setup.default_note_path())?;

    println!("**** {}", file_content);

    Ok(())
}
