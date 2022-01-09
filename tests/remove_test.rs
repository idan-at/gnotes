mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::write_note;
use gnotes::common::{load_tags, update_tags};
use maplit::{hashmap, hashset};
use serde_json::json;
use setup::Setup;

#[test]
fn test_remove_note() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["remove", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());

    Ok(())
}

#[test]
fn test_remove_note_alias() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["rm", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());

    Ok(())
}

#[test]
fn test_remove_note_succeeds_when_note_does_not_exist() -> Result<()> {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["remove", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_remove_note_custom_dir() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.note_path("custom");

    write_note(&setup.note_parent_dir("custom"), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["remove", "chores", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());

    Ok(())
}

#[test]
fn test_remove_note_also_removes_tag() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;
    update_tags(setup.dir.path(), &tags)?;

    let expected_tags = hashmap! {
        String::from("tag1") => hashset! { String::from("notes/reminders") }
    };

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["remove", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());
    assert_eq!(load_tags(setup.dir.path())?, expected_tags);

    Ok(())
}
