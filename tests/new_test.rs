mod setup;

use anyhow::Result;
use assert_cmd::Command;
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};
use std::fs;

#[test]
fn test_new_note_with_message() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.default_note_path();

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec![
        "new",
        DEFAULT_NOTE_FILE_NAME,
        "-m",
        "do this and that",
    ])
    .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
    .assert()
    .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}

#[test]
fn test_new_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.note_path("custom");

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec![
        "new",
        DEFAULT_NOTE_FILE_NAME,
        "-m",
        "do this and that",
        "--dir",
        "custom",
    ])
    .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
    .assert()
    .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}

#[test]
fn test_new_note_interactive() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.default_note_path();

    let mut cmd = Command::cargo_bin("gnotes")?;

    let mut stdin = String::new();

    stdin.push('i'); // enter vim edit mode
    stdin.push_str("do this and that");
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    cmd.args(vec!["new", DEFAULT_NOTE_FILE_NAME])
        .env("EDITOR", "vim")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .write_stdin(stdin)
        .assert()
        .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}
