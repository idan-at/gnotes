mod setup;

use assert_cmd::Command;
use setup::Setup;
use std::fs;

#[test]
fn test_append_to_new_note() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["append", "chores", "do this and that"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path).unwrap(),
        String::from("do this and that\n")
    );
}

#[test]
fn test_append_custom_dir() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("custom").join("chores");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec![
        "append",
        "chores",
        "do this and that",
        "--dir",
        "custom",
    ])
    .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
    .assert()
    .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path).unwrap(),
        String::from("do this and that\n")
    );
}

#[test]
fn test_append_to_existing_note() {
    let setup = Setup::new();
    let expected_note_file_path = setup.dir.path().join("notes").join("chores");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    fs::create_dir_all(setup.dir.path().join("notes"));
    fs::write(&expected_note_file_path, "hello\n").unwrap();

    cmd.args(vec!["append", "chores", "do this and that"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(expected_note_file_path).unwrap(),
        String::from("hello\ndo this and that\n")
    );
}
