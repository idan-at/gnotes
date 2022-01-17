mod setup;

use anyhow::{Context, Result};
use assert_cmd::Command;
use gnotes::common::notes::write_note;
use gnotes::common::tags::load_tags;
use maplit::{hashmap, hashset};
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};

#[test]
fn test_tag_note() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", DEFAULT_NOTE_FILE_NAME, "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected);

    Ok(())
}

#[test]
fn test_tag_note_twice() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected = hashmap! {
      String::from("tag") => hashset! { String::from("notes/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", DEFAULT_NOTE_FILE_NAME, "tag", "tag"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected);

    Ok(())
}

#[test]
fn test_tag_note_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.default_note_path();
    let tags_file_path = setup.dir.path().join(".tags");

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", DEFAULT_NOTE_FILE_NAME, "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "tag failed: file '{}' not found\n",
            note_file_path.to_str().context("note_file_path.to_str()")?
        ))
        .code(1);

    assert!(!tags_file_path.exists());

    Ok(())
}

#[test]
fn test_tag_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("custom/chores") },
      String::from("tag2") => hashset! { String::from("custom/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec![
            "tag",
            DEFAULT_NOTE_FILE_NAME,
            "--dir",
            "custom",
            "tag1",
            "tag2",
        ])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected);

    Ok(())
}

#[test]
fn test_tag_note_tag_already_exists_for_different_note() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    write_note(&setup.default_note_parent_dir(), "reminders", "goodbye")?;

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores"), String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", DEFAULT_NOTE_FILE_NAME, "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", "reminders", "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected);

    Ok(())
}

#[test]
fn test_tag_note_tag_already_exists_for_this_note() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", DEFAULT_NOTE_FILE_NAME, "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    Command::cargo_bin("gnotes")?
        .args(vec!["tag", DEFAULT_NOTE_FILE_NAME, "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path())?, expected);

    Ok(())
}
