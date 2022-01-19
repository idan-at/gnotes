mod setup;

use crate::setup::RunOptions;
use crate::setup::DEFAULT_NOTE_FILE_NAME;
use anyhow::Result;
use gnotes::common::notes::write_note;
use setup::Setup;
use std::fs;

#[test]
fn test_edit_note() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.default_note_path();

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let mut stdin = String::new();

    stdin.push_str("dd"); // delete the line
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let run_options = RunOptions {
        stdin: Some(stdin),
        repository: None,
    };

    setup
        .run(&["edit", DEFAULT_NOTE_FILE_NAME], Some(run_options))?
        .success();

    assert_eq!(
        fs::read_to_string(&expected_note_file_path)?,
        String::from("")
    );

    Ok(())
}

#[test]
fn test_edit_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.note_path("custom");

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;

    let mut stdin = String::new();

    stdin.push_str("dd"); // delete the line
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let run_options = RunOptions {
        stdin: Some(stdin),
        repository: None,
    };

    setup
        .run(
            &["edit", DEFAULT_NOTE_FILE_NAME, "--dir", "custom"],
            Some(run_options),
        )?
        .success();

    assert_eq!(
        fs::read_to_string(&expected_note_file_path)?,
        String::from("")
    );

    Ok(())
}

#[test]
fn test_edit_none_existing_note() -> Result<()> {
    let setup = Setup::new()?;
    let expected_note_file_path = setup.default_note_path();

    let mut stdin = String::new();

    stdin.push('i'); // enter vim edit mode
    stdin.push_str("do this and that");
    stdin.push(27 as char); // ESC
    stdin.push_str(":wq\n");

    let run_options = RunOptions {
        stdin: Some(stdin),
        repository: None,
    };

    setup
        .run(&["edit", DEFAULT_NOTE_FILE_NAME], Some(run_options))?
        .success();

    assert!(expected_note_file_path.exists());
    assert_eq!(
        fs::read_to_string(expected_note_file_path)?,
        String::from("do this and that\n")
    );

    Ok(())
}
