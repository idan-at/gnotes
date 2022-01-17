mod setup;

use anyhow::{Context, Result};
use assert_cmd::Command;
use gnotes::common::notes::write_note;
use predicates::prelude::*;
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};

#[test]
fn test_show_note() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    Command::cargo_bin("gnotes")?
        .args(vec!["show", DEFAULT_NOTE_FILE_NAME])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("notes/chores:\n"))
        .stdout(predicate::str::contains("hello"))
        .success();

    Ok(())
}

#[test]
fn test_show_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    Command::cargo_bin("gnotes")?
        .args(vec!["show", DEFAULT_NOTE_FILE_NAME, "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("custom/chores:\n"))
        .stdout(predicate::str::contains("hello"))
        .success();

    Ok(())
}

#[test]
fn test_show_note_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;

    let note_file_path = setup.default_note_path();

    Command::cargo_bin("gnotes")?
        .args(vec!["show", DEFAULT_NOTE_FILE_NAME])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "show failed: file '{}' not found\n",
            note_file_path.to_str().context("note_file_path.to_str()")?
        ))
        .code(1);

    Ok(())
}
