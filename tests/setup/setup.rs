// See https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

use super::constants::*;
use super::run_options::RunOptions;
use anyhow::Result;
use assert_cmd::assert::Assert;
use gnotes::config::Config;
use std::fs;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

pub struct Setup {
    home_dir: TempDir,
    notes_dir: TempDir,
}

impl Setup {
    pub fn new() -> Result<Self> {
        Ok(Self {
            home_dir: TempDir::new("gnotes_home")?,
            notes_dir: TempDir::new("gnotes_test")?,
        })
    }

    pub fn run(&self, args: &[&str], options: Option<RunOptions>) -> Result<Assert> {
        let options = options.unwrap_or_default();
        let ssh_file_path = self.home_dir.path().join("id_rsa");

        let config = Config {
            notes_dir: self.notes_dir.path().to_path_buf(),
            auto_save: false,
            repository: options
                .repository
                .map(|p| String::from(p.to_string_lossy())),
            ssh_file_path: ssh_file_path.to_path_buf(),
        };

        fs::write(&ssh_file_path, "TODO: write a valid id_rsa inside")?;
        fs::write(
            self.home_dir.path().join(".gnotes.toml"),
            toml::to_string(&config)?,
        )?;

        let stdin: String = options.stdin.unwrap_or_default();

        Ok(assert_cmd::Command::cargo_bin("gnotes")?
            .args(args)
            .env("EDITOR", "vim")
            .env("GNOTES_HOME_DIR", self.home_dir.path())
            .write_stdin(stdin)
            .assert())
    }

    pub fn notes_dir_path(&self) -> &Path {
        self.notes_dir.path()
    }

    pub fn note_parent_dir(&self, dir: &str) -> PathBuf {
        self.notes_dir_path().join(dir)
    }

    pub fn default_note_parent_dir(&self) -> PathBuf {
        self.note_parent_dir(DEFAULT_NOTES_DIR_NAME)
    }

    pub fn note_path(&self, dir: &str) -> PathBuf {
        self.note_parent_dir(dir).join(DEFAULT_NOTE_FILE_NAME)
    }

    pub fn default_note_path(&self) -> PathBuf {
        self.note_path(DEFAULT_NOTES_DIR_NAME)
    }
}
