mod common;

use assert_cmd::Command;
use common::Setup;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_list_notes() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

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
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

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
fn test_list_notes_ignore_non_directories() {
    let setup = Setup::new();

    fs::write(&setup.dir.path().join("notes"), "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["list"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 0\n"));
}

#[test]
fn test_list_notes_custom_dir() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("custom").join("chores");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex).unwrap());
}

#[test]
fn test_list_notes_include_headers() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_headers_line_regex = "Created\\s+Length\\s+Updated\\s+Path\n";
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--include-headers"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_headers_line_regex).unwrap())
        .stdout(predicate::str::is_match(expected_line_regex).unwrap());
}

#[test]
fn test_list_notes_all() {
    let setup = Setup::new();
    let note1_file_path = setup.dir.path().join("notes").join("chores");
    let note2_file_path = setup.dir.path().join("reminders").join("doctor");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::create_dir_all(setup.dir.path().join("reminders")).unwrap();
    fs::write(&note1_file_path, "hello\n").unwrap();
    fs::write(&note2_file_path, "goodbye\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();
    let expected_line1_regex = format!(".+ 6 .+{}", note1_file_path.to_str().unwrap());
    let expected_line2_regex = format!(".+ 8 .+{}", note2_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::is_match(expected_line1_regex).unwrap())
        .stdout(predicate::str::is_match(expected_line2_regex).unwrap());
}

#[test]
fn test_list_notes_all_ignore_non_directories() {
    let setup = Setup::new();

    fs::write(&setup.dir.path().join("notes"), "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["list", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 0\n"));
}

#[test]
fn test_list_notes_custom_dir_with_all() {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["list", "--dir", "custom", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .code(1)
        .stderr(predicate::eq("--dir can't be used with --all\n"));
}
