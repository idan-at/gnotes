mod setup;

use assert_cmd::Command;
use serde_json::{json, Value};
use setup::Setup;
use std::fs;
use std::path::Path;

fn read_to_json(path: &Path) -> Value {
    let content = fs::read_to_string(path).unwrap();

    serde_json::from_str::<Value>(&content).unwrap()
}

// TODO: support untag
#[test]
fn test_tag_note() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let expected_tags_file_path = setup.dir.path().join(".tags");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = json!({"tag1":["notes/chores"],"tag2":["notes/chores"]});

    assert!(expected_tags_file_path.exists());
    assert_eq!(read_to_json(&expected_tags_file_path), expected);
}

#[test]
fn test_tag_note_twice() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let expected_tags_file_path = setup.dir.path().join(".tags");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "tag", "tag"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = json!({"tag":["notes/chores"]});

    assert!(expected_tags_file_path.exists());
    assert_eq!(read_to_json(&expected_tags_file_path), expected);
}

#[test]
fn test_tag_note_does_not_exist() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let tags_file_path = setup.dir.path().join(".tags");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "tag failed: file '{}' not found\n",
            note_file_path.to_str().unwrap()
        ))
        .code(1);

    assert!(!tags_file_path.exists());
}

#[test]
fn test_tag_note_custom_dir() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("custom").join("chores");
    let expected_tags_file_path = setup.dir.path().join(".tags");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "--dir", "custom", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = json!({"tag1":["custom/chores"],"tag2":["custom/chores"]});

    assert!(expected_tags_file_path.exists());
    assert_eq!(read_to_json(&expected_tags_file_path), expected);
}

#[test]
fn test_tag_note_tag_already_exists_for_different_note() {
    let setup = Setup::new();
    let note1_file_path = setup.dir.path().join("notes").join("chores");
    let note2_file_path = setup.dir.path().join("notes").join("reminders");
    let expected_tags_file_path = setup.dir.path().join(".tags");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note1_file_path, "hello\n").unwrap();
    fs::write(&note2_file_path, "gooebye\n").unwrap();

    let mut cmd1 = Command::cargo_bin("gnotes").unwrap();
    let mut cmd2 = Command::cargo_bin("gnotes").unwrap();

    cmd1.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    cmd2.args(vec!["tag", "reminders", "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    assert!(expected_tags_file_path.exists());
    assert_eq!(read_to_json(&expected_tags_file_path), expected);
}

#[test]
fn test_tag_note_tag_already_exists_for_this_note() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let expected_tags_file_path = setup.dir.path().join(".tags");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd1 = Command::cargo_bin("gnotes").unwrap();
    let mut cmd2 = Command::cargo_bin("gnotes").unwrap();

    cmd1.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    cmd2.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = json!({"tag1":["notes/chores"],"tag2":["notes/chores"]});

    assert!(expected_tags_file_path.exists());
    assert_eq!(read_to_json(&expected_tags_file_path), expected);
}
