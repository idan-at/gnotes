mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::write_note;
use setup::Setup;
use std::fs;

#[test]
fn test_edit_note() -> Result<()> {
    let setup = Setup::new();
    let expected_note_file_path = setup.default_note_path();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut stdin = String::new();

    stdin.push_str("dd"); // delete the line
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["edit", "chores"])
        .env("EDITOR", "vim")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .write_stdin(stdin)
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&expected_note_file_path)?,
        String::from("")
    );

    Ok(())
}

#[test]
fn test_edit_note_custom_dir() -> Result<()> {
    let setup = Setup::new();
    let expected_note_file_path = setup.note_path("custom");

    write_note(&setup.note_parent_dir("custom"), "chores", "hello")?;

    let mut stdin = String::new();

    stdin.push_str("dd"); // delete the line
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["edit", "chores", "--dir", "custom"])
        .env("EDITOR", "vim")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .write_stdin(stdin)
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&expected_note_file_path)?,
        String::from("")
    );

    Ok(())
}

#[test]
fn test_edit_none_existing_note() -> Result<()> {
    let setup = Setup::new();
    let expected_note_file_path = setup.default_note_path();

    let mut cmd = Command::cargo_bin("gnotes")?;

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
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}
