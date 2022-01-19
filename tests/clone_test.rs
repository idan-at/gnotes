mod setup;

use anyhow::Result;
use predicates::prelude::*;
use setup::{GitSetup, RunOptions, Setup};

#[test]
fn test_clone_fails_without_repository() -> Result<()> {
    let setup = Setup::new()?;

    setup
        .run(&["clone"], None)?
        .stderr(predicate::eq(
            "Can't clone without a repository. Please specify a repository in the config file.\n",
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_clone_succeeds() -> Result<()> {
    let setup = Setup::new()?;
    let git_setup = GitSetup::new(None)?;

    let run_options = RunOptions {
        stdin: None,
        repository: Some(git_setup.bare_dir.path().to_path_buf()),
    };

    setup.run(&["clone"], Some(run_options))?.code(0);

    assert!(setup.default_note_path().exists());

    Ok(())
}
