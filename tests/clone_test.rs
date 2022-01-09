mod setup;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use setup::Setup;

#[test]
fn test_clone_fails_without_repository() -> Result<()> {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["clone"])
        .assert()
        .stderr(predicate::eq("Can't clone without a repository. Please specify a repository in the config file.\n"))
        .code(1);

    Ok(())
}
