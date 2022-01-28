mod setup;

use crate::setup::{DEFAULT_NOTES_DIR_NAME, DEFAULT_NOTE_FILE_NAME};
use anyhow::{Context, Result};
use gnotes::common::notes::write_note;
use predicates::prelude::*;
use setup::Setup;
use std::fs;
use std::path::Path;

#[test]
fn test_list_notes() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected_note_path = Path::new(DEFAULT_NOTES_DIR_NAME).join(DEFAULT_NOTE_FILE_NAME);
    let expected_line_regex = format!(
        ".+ 6 .+{}",
        expected_note_path
            .to_str()
            .context("expected_note_path.to_str()")?
    );

    setup
        .run(&["list"], None)?
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_alias() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected_note_path = Path::new(DEFAULT_NOTES_DIR_NAME).join(DEFAULT_NOTE_FILE_NAME);
    let expected_line_regex = format!(
        ".+ 6 .+{}",
        expected_note_path
            .to_str()
            .context("expected_note_path.to_str()")?
    );

    setup
        .run(&["ls"], None)?
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;

    setup
        .run(&["list"], None)?
        .success()
        .stdout(predicate::str::contains("total 0\n"));

    Ok(())
}

#[test]
fn test_list_notes_ignore_non_directories() -> Result<()> {
    let setup = Setup::new()?;

    fs::write(&setup.default_note_parent_dir(), "hello\n")?;

    setup
        .run(&["list"], None)?
        .success()
        .stdout(predicate::str::contains("total 0\n"));

    Ok(())
}

#[test]
fn test_list_notes_custom_dir() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected_note_path = Path::new("custom").join(DEFAULT_NOTE_FILE_NAME);
    let expected_line_regex = format!(
        ".+ 6 .+{}",
        expected_note_path
            .to_str()
            .context("expected_note_path.to_str()")?
    );

    setup
        .run(&["list", "--dir", "custom"], None)?
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_include_headers() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let expected_note_path = Path::new(DEFAULT_NOTES_DIR_NAME).join(DEFAULT_NOTE_FILE_NAME);
    let expected_headers_line_regex = "Created\\s+Length\\s+Updated\\s+Path\n";
    let expected_line_regex = format!(
        ".+ 6 .+{}",
        expected_note_path
            .to_str()
            .context("expected_note_path.to_str()")?
    );

    setup
        .run(&["list", "--include-headers"], None)?
        .success()
        .stdout(predicate::str::contains("total 1\n"))
        .stdout(predicate::str::is_match(expected_headers_line_regex)?)
        .stdout(predicate::str::is_match(expected_line_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_all() -> Result<()> {
    let setup = Setup::new()?;

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    write_note(
        &setup.notes_dir_path().join("reminders"),
        "doctor",
        "goodbye",
    )?;

    let expected_note1_path = Path::new(DEFAULT_NOTES_DIR_NAME).join(DEFAULT_NOTE_FILE_NAME);
    let expected_line1_regex = format!(
        ".+ 6 .+{}",
        expected_note1_path
            .to_str()
            .context("expected_note1_path.to_str()")?
    );
    let expected_note2_path = Path::new("reminders").join("doctor");
    let expected_line2_regex = format!(
        ".+ 8 .+{}",
        expected_note2_path
            .to_str()
            .context("expected_note_path.to_str()")?
    );

    setup
        .run(&["list", "--all"], None)?
        .success()
        .stdout(predicate::str::contains("total 2\n"))
        .stdout(predicate::str::is_match(expected_line1_regex)?)
        .stdout(predicate::str::is_match(expected_line2_regex)?);

    Ok(())
}

#[test]
fn test_list_notes_all_ignore_non_directories() -> Result<()> {
    let setup = Setup::new()?;

    fs::write(&setup.default_note_parent_dir(), "hello\n")?;

    setup
        .run(&["list", "--all"], None)?
        .success()
        .stdout(predicate::str::contains("total 0\n"));

    Ok(())
}

#[test]
fn test_list_notes_custom_dir_with_all() -> Result<()> {
    let setup = Setup::new()?;

    setup
        .run(&["list", "--dir", "custom", "--all"], None)?
        .code(1)
        .stderr(predicate::eq("--dir can't be used with --all\n"));

    Ok(())
}
