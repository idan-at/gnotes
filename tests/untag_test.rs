mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::notes::write_note;
use gnotes::common::tags::{load_tags, update_tags};
use maplit::{hashmap, hashset};
use serde_json::json;
use setup::Setup;

#[test]
fn test_untag_note() -> Result<()> {
    let setup = Setup::new();
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;
    update_tags(setup.dir.path(), &tags).unwrap();

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["untag", "chores", "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_custom_dir() -> Result<()> {
    let setup = Setup::new();
    let tags = json!({"tag1":["custom/chores", "notes/reminders"],"tag2":["custom/chores"]});

    write_note(&setup.note_parent_dir("custom"), "chores", "hello")?;
    update_tags(setup.dir.path(), &tags).unwrap();

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("custom/chores") },
    };

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["untag", "chores", "tag1", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_removes_tag_if_empty() -> Result<()> {
    let setup = Setup::new();
    let tags = json!({"tag1":["notes/reminders"],"tag2":["notes/chores"]});

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;
    update_tags(setup.dir.path(), &tags).unwrap();

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
    };

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["untag", "chores", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_does_not_exist() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["untag", "chores", "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "untag failed: file '{}' not found\n",
            note_file_path.to_str().unwrap()
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_untag_note_tag_does_not_exist() -> Result<()> {
    let setup = Setup::new();
    let tags = json!({"tag1":["notes/chores"],"tag2":["notes/chores"]});

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;
    update_tags(setup.dir.path(), &tags).unwrap();

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["untag", "chores", "tag3"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected_tags);

    Ok(())
}
