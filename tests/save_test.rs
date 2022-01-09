use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_save_fails_without_repository() -> Result<()> {
    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["save"])
        .assert()
        .stderr(predicate::eq(
            "Can't save without a repository. Please specify a repository in the config file.\n",
        ))
        .code(1);

    Ok(())
}

// TODO: Test save works.
