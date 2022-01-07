mod common;

use assert_cmd::Command;
use common::Setup;
use std::fs;

#[test]
fn test_edit_note() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut stdin = String::new();

    stdin.push_str("dd"); // delete the line
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["edit", "chores"])
        .env("EDITOR", "vim")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .write_stdin(stdin)
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&expected_note_file_path).unwrap(),
        String::from("")
    );
}

#[test]
fn test_edit_note_custom_dir() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("custom").join("chores");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    let mut stdin = String::new();

    stdin.push_str("dd"); // delete the line
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["edit", "chores", "--dir", "custom"])
        .env("EDITOR", "vim")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .write_stdin(stdin)
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&expected_note_file_path).unwrap(),
        String::from("")
    );
}

#[test]
fn test_edit_none_existing_note() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    let mut stdin = String::new();

    stdin.push('i'); // enter vim edit mode
    stdin.push_str("do this and that");
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    cmd.args(vec!["edit", "chores"])
        .env("EDITOR", "vim")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .write_stdin(stdin)
        .assert()
        .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path).unwrap(),
        String::from("do this and that\n")
    );
}
