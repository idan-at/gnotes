mod setup;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use setup::{Setup, GitSetup};

#[test]
fn test_clone_fails_without_repository() -> Result<()> {
    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["clone"])
        .assert()
        .stderr(predicate::eq(
            "Can't clone without a repository. Please specify a repository in the config file.\n",
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_clone_succeeds() -> Result<()> {
    let setup = Setup::new();
    let git_setup = GitSetup::new()?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    // TODO: Provide id_rsa explicitly (ATM this assumes ~/.ssh/id_rsa exists)
    cmd.args(vec!["clone"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .env("GNOTES_REPOSITORY", &git_setup.repo_path)
        .assert()
        .code(0);

    assert!(setup.default_note_path().exists());

    Ok(())
}
