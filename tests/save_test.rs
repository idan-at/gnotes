mod setup;

use crate::setup::GitSetup;
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
    let git_setup = GitSetup::new()?;

    fs::write(
        GitSetup::build_note_path(git_setup.clone_dir.path()),
        "updated file content\n",
    )?;

    // TODO: Provide id_rsa explicitly (ATM this assumes ~/.ssh/id_rsa exists)
    Command::cargo_bin("gnotes")?
        .args(vec!["save"])
        .env("GNOTES_NOTES_DIR", git_setup.clone_dir.path())
        .env("GNOTES_REPOSITORY", git_setup.bare_dir.path())
        .assert()
        .code(0);

    let file_content = fs::read_to_string(GitSetup::build_note_path(git_setup.bare_dir.path()))?;

    println!("**** {}", file_content);

    Ok(())
}
