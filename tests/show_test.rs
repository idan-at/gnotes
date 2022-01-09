mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::write_note;
use predicates::prelude::*;
use setup::Setup;

#[test]
fn test_show_note() -> Result<()> {
    let setup = Setup::new();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["show", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("notes/chores:\n"))
        .stdout(predicate::str::contains("hello"))
        .success();

    Ok(())
}

#[test]
fn test_show_note_custom_dir() -> Result<()> {
    let setup = Setup::new();

    write_note(&setup.note_parent_dir("custom"), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["show", "chores", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("custom/chores:\n"))
        .stdout(predicate::str::contains("hello"))
        .success();

    Ok(())
}

#[test]
fn test_show_note_does_not_exist() -> Result<()> {
    let setup = Setup::new();

    let note_file_path = setup.default_note_path();
    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["show", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "show failed: file '{}' not found\n",
            note_file_path.to_str().unwrap()
        ))
        .code(1);

    Ok(())
}
