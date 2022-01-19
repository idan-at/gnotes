mod setup;

use anyhow::{Context, Result};
use gnotes::common::notes::write_note;
use gnotes::common::tags::{load_tags, update_tags};
use maplit::{hashmap, hashset};
use serde_json::json;
use setup::{Setup, DEFAULT_NOTE_FILE_NAME};

#[test]
fn test_untag_note() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["notes/chores", "notes/reminders"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.notes_dir_path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    setup
        .run(&["untag", DEFAULT_NOTE_FILE_NAME, "tag1"], None)?
        .success();

    assert_eq!(load_tags(setup.notes_dir_path())?, expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_custom_dir() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["custom/chores", "notes/reminders"],"tag2":["custom/chores"]});

    write_note(
        &setup.note_parent_dir("custom"),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.notes_dir_path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("custom/chores") },
    };

    setup
        .run(
            &["untag", DEFAULT_NOTE_FILE_NAME, "tag1", "--dir", "custom"],
            None,
        )?
        .success();

    assert_eq!(load_tags(setup.notes_dir_path())?, expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_removes_tag_if_empty() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["notes/reminders"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.notes_dir_path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/reminders") },
    };

    setup
        .run(&["untag", DEFAULT_NOTE_FILE_NAME, "tag2"], None)?
        .success();

    assert_eq!(load_tags(setup.notes_dir_path())?, expected_tags);

    Ok(())
}

#[test]
fn test_untag_note_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;
    let note_file_path = setup.default_note_path();

    setup
        .run(&["untag", DEFAULT_NOTE_FILE_NAME, "tag1"], None)?
        .stderr(format!(
            "untag failed: file '{}' not found\n",
            note_file_path.to_str().context("note_file_path.to_str()")?
        ))
        .code(1);

    Ok(())
}

#[test]
fn test_untag_note_tag_does_not_exist() -> Result<()> {
    let setup = Setup::new()?;
    let tags = json!({"tag1":["notes/chores"],"tag2":["notes/chores"]});

    write_note(
        &setup.default_note_parent_dir(),
        DEFAULT_NOTE_FILE_NAME,
        "hello",
    )?;
    update_tags(setup.notes_dir_path(), &tags)?;

    let expected_tags = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    setup
        .run(&["untag", DEFAULT_NOTE_FILE_NAME, "tag3"], None)?
        .success();

    assert_eq!(load_tags(setup.notes_dir_path())?, expected_tags);

    Ok(())
}
