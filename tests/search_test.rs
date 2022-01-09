mod common;

use assert_cmd::Command;
use common::{read_to_json, Setup};
use predicates::prelude::*;
use serde_json::json;
use std::fs;

#[test]
fn test_search_note() {
    let setup = Setup::new();
    let tags_file_path = setup.dir.path().join(".tags");
    let tags = json!({"tag":["notes/chores", "notes/reminders"]});

    fs::write(&tags_file_path, tags.to_string()).unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["search", "tag"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::contains("notes/chores\n"))
        .stdout(predicate::str::contains("notes/reminders\n"))
        .success();
}

#[test]
fn test_search_note_no_matches() {
    let setup = Setup::new();
    let tags_file_path = setup.dir.path().join(".tags");
    let tags = json!({"tag":["notes/chores", "notes/reminders"]});

    fs::write(&tags_file_path, tags.to_string()).unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["search", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::eq(""))
        .success();
}

#[test]
fn test_search_note_custom_dir() {
    let setup = Setup::new();
    let tags_file_path = setup.dir.path().join(".tags");
    let tags = json!({"tag":["custom1/chores", "custom/reminders"]});

    fs::write(&tags_file_path, tags.to_string()).unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["search", "tag", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::contains("custom/reminders\n"))
        .success();
}

#[test]
fn test_search_note_all() {
    let setup = Setup::new();
    let tags_file_path = setup.dir.path().join(".tags");
    let tags = json!({"tag":["notes/chores", "custom/reminders"]});

    fs::write(&tags_file_path, tags.to_string()).unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["search", "tag", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::contains("notes/chores\n"))
        .stdout(predicate::str::contains("custom/reminders\n"))
        .success();
}
