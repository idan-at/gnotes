mod setup;

use assert_cmd::Command;
use gnotes::common::load_tags;
use maplit::{hashmap, hashset};
use setup::Setup;
use std::fs;

#[test]
fn test_tag_note() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected);
}

#[test]
fn test_tag_note_twice() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "tag", "tag"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = hashmap! {
      String::from("tag") => hashset! { String::from("notes/chores") },
    };

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected);
}

#[test]
fn test_tag_note_does_not_exist() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");
    let tags_file_path = setup.dir.path().join(".tags");

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .stderr(format!(
            "tag failed: file '{}' not found\n",
            note_file_path.to_str().unwrap()
        ))
        .code(1);

    assert!(!tags_file_path.exists());
}

#[test]
fn test_tag_note_custom_dir() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("custom").join("chores");

    fs::create_dir_all(setup.dir.path().join("custom")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.args(vec!["tag", "chores", "--dir", "custom", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("custom/chores") },
      String::from("tag2") => hashset! { String::from("custom/chores") },
    };

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected);
}

#[test]
fn test_tag_note_tag_already_exists_for_different_note() {
    let setup = Setup::new();
    let note1_file_path = setup.dir.path().join("notes").join("chores");
    let note2_file_path = setup.dir.path().join("notes").join("reminders");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note1_file_path, "hello\n").unwrap();
    fs::write(&note2_file_path, "gooebye\n").unwrap();

    let mut cmd1 = Command::cargo_bin("gnotes").unwrap();
    let mut cmd2 = Command::cargo_bin("gnotes").unwrap();

    cmd1.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    cmd2.args(vec!["tag", "reminders", "tag1"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores"), String::from("notes/reminders") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected);
}

#[test]
fn test_tag_note_tag_already_exists_for_this_note() {
    let setup = Setup::new();
    let note_file_path = setup.dir.path().join("notes").join("chores");

    fs::create_dir_all(setup.dir.path().join("notes")).unwrap();
    fs::write(&note_file_path, "hello\n").unwrap();

    let mut cmd1 = Command::cargo_bin("gnotes").unwrap();
    let mut cmd2 = Command::cargo_bin("gnotes").unwrap();

    cmd1.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    cmd2.args(vec!["tag", "chores", "tag1", "tag2"])
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();

    let expected = hashmap! {
      String::from("tag1") => hashset! { String::from("notes/chores") },
      String::from("tag2") => hashset! { String::from("notes/chores") },
    };

    assert_eq!(load_tags(setup.dir.path()).unwrap(), expected);
}
