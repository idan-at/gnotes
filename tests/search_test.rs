mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::notes::write_note;
use gnotes::common::tags::update_tags;
use predicates::prelude::*;
use serde_json::json;
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};

#[test]
fn test_search_note() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag":["notes/chores", "notes/reminders"]});

    update_tags(setup.dir.path(), &tags)?;

    Command::cargo_bin("gnotes")?
        .args(vec!["search", "tag"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .assert()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::contains("notes/chores\n"))
        .stdout(predicate::str::contains("notes/reminders\n"))
        .success();

    Ok(())
}

#[test]
fn test_search_note_no_matches() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag":["notes/chores", "notes/reminders"]});

    update_tags(setup.dir.path(), &tags)?;

    Command::cargo_bin("gnotes")?
        .args(vec!["search", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .assert()
        .stdout(predicate::eq(""))
        .success();

    Ok(())
}

#[test]
fn test_search_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag":["custom1/chores", "custom/reminders"]});

    update_tags(setup.dir.path(), &tags)?;

    Command::cargo_bin("gnotes")?
        .args(vec!["search", "tag", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .assert()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::contains("custom/reminders\n"))
        .success();

    Ok(())
}

#[test]
fn test_search_note_all() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag":["notes/chores", "custom/reminders"]});

    update_tags(setup.dir.path(), &tags)?;

    Command::cargo_bin("gnotes")?
        .args(vec!["search", "tag", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .assert()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::contains("notes/chores\n"))
        .stdout(predicate::str::contains("custom/reminders\n"))
        .success();

    Ok(())
}

#[test]
fn test_search_note_show() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag":["notes/chores", "custom/reminders"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.dir.path(), &tags)?;

    Command::cargo_bin("gnotes")?
        .args(vec!["search", "tag", "--show"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .assert()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::contains("notes/chores:\n"))
        .stdout(predicate::str::contains("hello"))
        .success();

    Ok(())
}

#[test]
fn test_search_note_custom_dir_with_all() -> Result<()> {
    let setup = Setup::new()?;

    Command::cargo_bin("gnotes")?
        .args(vec!["search", "tag", "--dir", "custom", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.path())
        .assert()
        .stderr(predicate::eq("--dir can't be used with --all\n"))
        .code(1);

    Ok(())
}
