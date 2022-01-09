mod common;

use assert_cmd::Command;
use common::{read_to_json, Setup};
use serde_json::json;
use std::fs;

#[test]
fn test_remove_note() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["remove", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());
}

#[test]
fn test_remove_note_alias() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["rm", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());
}

#[test]
fn test_remove_note_succeeds_when_note_does_not_exist() {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["remove", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();
}

#[test]
fn test_remove_note_custom_dir() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("custom").join("chores");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["remove", "chores", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());
}

#[test]
fn test_remove_note_also_removes_tag() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let tags_file_path = setup.dir.path().join(".tags");
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();
    fs::write(&tags_file_path, tags.to_string()).unwrap();

    let expected_tags = json!({"tag1":["notes/reminders"]});

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["remove", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(!note_file_path.exists());
    assert_eq!(read_to_json(&tags_file_path), expected_tags);
}
