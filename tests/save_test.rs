mod setup;

use crate::setup::{GitSetup, GitSetupOptions, RunOptions, Setup};
use anyhow::Result;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_save_fails_without_repository() -> Result<()> {
    let setup = Setup::new()?;

    setup
        .run(&["save"], None)?
        .stderr(predicate::eq(
            "Can't save without a repository. Please specify a repository in the config file.\n",
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_save_succeeds_on_empty_repository() -> Result<()> {
    let setup = Setup::new()?;
    let git_setup = GitSetup::new(Some(GitSetupOptions {
        with_changes: false,
    }))?;

    GitSetup::clone_to(git_setup.bare_dir.path(), setup.notes_dir_path())?;

    fs::create_dir(setup.default_note_parent_dir())?;
    fs::write(setup.default_note_path(), "updated file content\n")?;

    let run_options = RunOptions {
        stdin: None,
        repository: Some(git_setup.bare_dir.path().to_path_buf()),
    };

    setup.run(&["save"], Some(run_options))?.code(0);

    git_setup.update_clone()?;

    assert_eq!(
        fs::read_to_string(GitSetup::build_note_path(git_setup.clone_dir.path()))?,
        "updated file content\n"
    );

    Ok(())
}

#[test]
fn test_save_succeeds_on_non_empty_repository() -> Result<()> {
    let setup = Setup::new()?;
    let git_setup = GitSetup::new(None)?;

    GitSetup::clone_to(git_setup.bare_dir.path(), setup.notes_dir_path())?;

    fs::write(setup.default_note_path(), "updated file content\n")?;

    let run_options = RunOptions {
        stdin: None,
        repository: Some(git_setup.bare_dir.path().to_path_buf()),
    };

    setup.run(&["save"], Some(run_options))?.code(0);

    git_setup.update_clone()?;

    assert_eq!(
        fs::read_to_string(GitSetup::build_note_path(git_setup.clone_dir.path()))?,
        "updated file content\n"
    );

    Ok(())
}
