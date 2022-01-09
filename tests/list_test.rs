mod setup;

use anyhow::Result;
use assert_cmd::Command;
use gnotes::common::write_note;
use predicates::prelude::*;
use setup::Setup;
use std::fs;

#[test]
fn test_list_notes() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

    cmd.args(vec!["list"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_alias() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

    cmd.args(vec!["ls"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_does_not_exist() -> Result<()> {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["list"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::eq("total 0\n"));

    Ok(())
}

#[test]
fn test_list_notes_ignore_non_directories() -> Result<()> {
    let setup = Setup::new();

    fs::write(&setup.dir.path().join("notes"), "hello\n")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["list"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 0\n"));

    Ok(())
}

#[test]
fn test_list_notes_custom_dir() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.note_path("custom");

    write_note(&setup.note_parent_dir("custom"), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--dir", "custom"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_include_headers() -> Result<()> {
    let setup = Setup::new();
    let note_file_path = setup.default_note_path();

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;

    let mut cmd = Command::cargo_bin("gnotes")?;
    let expected_headers_line_regex = "Created\\s+Length\\s+Updated\\s+Path\n";
    let expected_line_regex = format!(".+ 6 .+{}", note_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--include-headers"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_headers_line_regex)?)
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_all() -> Result<()> {
    let setup = Setup::new();
    let note1_file_path = setup.default_note_path();
    let note2_file_path = setup.dir.path().join("reminders").join("doctor");

    write_note(&setup.default_note_parent_dir(), "chores", "hello")?;
    write_note(&setup.dir.path().join("reminders"), "doctor", "goodbye")?;

    let mut cmd = Command::cargo_bin("gnotes")?;
    let expected_line1_regex = format!(".+ 6 .+{}", note1_file_path.to_str().unwrap());
    let expected_line2_regex = format!(".+ 8 .+{}", note2_file_path.to_str().unwrap());

    cmd.args(vec!["list", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::is_match(expected_line1_regex)?)
        .stdout(predicate::str::is_match(expected_line2_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_all_ignore_non_directories() -> Result<()> {
    let setup = Setup::new();

    fs::write(&setup.dir.path().join("notes"), "hello\n")?;

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["list", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success()
        .stdout(predicate::str::contains("total 0\n"));

    Ok(())
}

#[test]
fn test_list_notes_custom_dir_with_all() -> Result<()> {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes")?;

    cmd.args(vec!["list", "--dir", "custom", "--all"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .code(1)
        .stderr(predicate::eq("--dir can't be used with --all\n"));

    Ok(())
}
