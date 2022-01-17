mod setup;

use anyhow::{Context, Result};
use assert_cmd::Command;
use gnotes::common::notes::write_note;
use gnotes::common::tags::{load_tags, update_tags};
use maplit::{hashmap, hashset};
use serde_json::json;
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};

#[test]
fn test_untag_note() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.dir.path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["untag", DEFAULT_NOTE_FILE_NAME, "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["custom/chores", "notes/reminders"],"tag2":["custom/chores"]});

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.dir.path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("custom/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec![
            "untag",
            DEFAULT_NOTE_FILE_NAME,
            "tag1",
            "--dir",
            "custom",
        ])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_removes_tag_if_empty() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["notes/reminders"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.dir.path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["untag", DEFAULT_NOTE_FILE_NAME, "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.default_note_path();

    Command::cargo_bin("gnotes")?
        .args(vec!["untag", DEFAULT_NOTE_FILE_NAME, "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "untag failed: file '{}' not found\n",
            note_file_path.to_str().context("note_file_path.to_str()")?
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_untag_note_tag_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["notes/chores"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.dir.path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["untag", DEFAULT_NOTE_FILE_NAME, "tag3"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected_tags);

    Ok(())
}
