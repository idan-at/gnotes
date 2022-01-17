mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::notes::write_note;
use gnotes::common::tags::{load_tags, update_tags};
use maplit::{hashmap, hashset};
use serde_json::json;
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};

#[test]
fn test_remove_note() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.default_note_path();

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    Command::cargo_bin("gnotes")?
        .args(vec!["remove", DEFAULT_NOTE_FILE_NAME])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());

    Ok(())
}

#[test]
fn test_remove_note_alias() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.default_note_path();

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    Command::cargo_bin("gnotes")?
        .args(vec!["rm", DEFAULT_NOTE_FILE_NAME])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());

    Ok(())
}

#[test]
fn test_remove_note_succeeds_when_note_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;

    Command::cargo_bin("gnotes")?
        .args(vec!["remove", DEFAULT_NOTE_FILE_NAME])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_remove_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.note_path("custom");

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    Command::cargo_bin("gnotes")?
        .args(vec!["remove", DEFAULT_NOTE_FILE_NAME, "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());

    Ok(())
}

#[test]
fn test_remove_note_also_removes_tag() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.default_note_path();
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.dir.path(), &tags)?;

    let expected_tags = hashmap! {
        String::from("tag1") => hashset! { String::from("notes/reminders") }
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["remove", DEFAULT_NOTE_FILE_NAME])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());
    assert_eq!(load_tags(setup.dir.path())?, expected_tags);

    Ok(())
}
