mod setup;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use setup::{GitSetup, Setup};

#[test]
fn test_clone_fails_without_repository() -> Result<()> {
    Command::cargo_bin("gnotes")?
        .args(vec!["clone"])
        .assert()
        .stderr(predicate::eq(
            "Can't clone without a repository. Please specify a repository in the config file.\n",
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_clone_succeeds() -> Result<()> {
    let setup = Setup::new()?;
    let git_setup = GitSetup::new()?;

    // TODO: Provide id_rsa explicitly (ATM this assumes ~/.ssh/id_rsa exists)
    Command::cargo_bin("gnotes")?
        .args(vec!["clone"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .env("GNOTES_REPOSITORY", git_setup.bare_dir.path())
        .assert()
        .code(0);

    assert!(setup.default_note_path().exists());

    Ok(())
}
