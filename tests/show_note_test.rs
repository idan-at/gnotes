mod setup;

use assert_cmd::Command;
use predicates::prelude::*;
use setup::Setup;
use std::fs;

#[test]
fn test_show_note() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["show", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::eq("hello\n"))
        .success();
}

#[test]
fn test_show_note_custom_dir() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("custom").join("chores");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["show", "chores", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stdout(predicate::eq("hello\n"))
        .success();
}

#[test]
fn test_show_note_does_not_exist() {
    let setup = Setup::new();

    let expected_note_file_path = setup.dir.path().join("notes").join("chores");
    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["show", "chores"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "show failed: file '{}' not found\n",
            expected_note_file_path.to_str().unwrap()
        ))
        .code(1);
}
