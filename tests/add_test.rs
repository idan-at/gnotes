mod setup;

use crate::setup::DEFAULT_NOTE_FILE_NAME;
use anyhow::Result;
use gnotes::common::notes::write_note;
use setup::Setup;
use std::fs;

#[test]
fn test_add_to_new_note() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.default_note_path();

    setup
        .run(&["add", DEFAULT_NOTE_FILE_NAME, "do this and that"], None)?
        .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}

#[test]
fn test_add_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.note_path("custom");

    setup
        .run(
            &[
                "add",
                DEFAULT_NOTE_FILE_NAME,
                "do this and that",
                "--dir",
                "custom",
            ],
            None,
        )?
        .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}

#[test]
fn test_add_to_existing_note() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.default_note_path();

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    setup
        .run(&["add", DEFAULT_NOTE_FILE_NAME, "do this and that"], None)?
        .success();

    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("hello\ndo this and that\n")
    );

    Ok(())
}
