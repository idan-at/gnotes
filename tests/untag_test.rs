mod setup;

use assert_cmd::Command;
use gnotes::common::{load_tags, update_tags};
use maplit::{hashmap, hashset};
use serde_json::json;
use setup::Setup;
use std::fs;

#[test]
fn test_untag_note() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();
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
}

#[test]
fn test_untag_note_removes_tag_if_empty() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let tags = json!({"tag1":["notes/reminders"],"tag2":["notes/chores"]});

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();
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
}

#[test]
fn test_untag_note_does_not_exist() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["untag", "chores", "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "untag failed: file '{}' not found\n",
            note_file_path.to_str().unwrap()
        ))
        .code(1);
}

#[test]
fn test_untag_note_tag_does_not_exist() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let tags = json!({"tag1":["notes/chores"],"tag2":["notes/chores"]});

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();
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
}
