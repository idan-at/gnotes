mod setup;

use assert_cmd::Command;
use predicates::prelude::*;
use setup::Setup;
use std::fs;

#[test]
fn test_list_notes() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line_regex = format!(".+ 6 .+{}", expected_note_file_path.to_str().unwrap());

    cmd.args(vec!["list"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex).unwrap());
}

#[test]
fn test_list_notes_alias() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line_regex = format!(".+ 6 .+{}", expected_note_file_path.to_str().unwrap());

    cmd.args(vec!["ls"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex).unwrap());
}

#[test]
fn test_list_notes_does_not_exist() {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["list"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::eq("total 0\n"));
}

#[test]
fn test_list_notes_custom_dir() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("custom").join("chores");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line_regex = format!(".+ 6 .+{}", expected_note_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex).unwrap());
}
