// See https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempdir::TempDir;

pub const DEFAULT_NOTES_DIR_NAME: &'static str = "notes";
pub const DEFAULT_NOTE_FILE_NAME: &'static str = "chores";

pub struct Setup {
    pub dir: TempDir,
}

impl Setup {
    pub fn new() -> Result<Self> {
        Ok(Self {
            dir: TempDir::new("gnotes_test")?,
        })
    }

    pub fn dir_path(&self) -> &Path {
        self.dir.path()
    }

    pub fn note_parent_dir(&self, dir: &str) -> PathBuf {
        self.dir_path().join(dir)
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

pub struct GitSetup {
    dir: TempDir,
    pub repo_path: PathBuf,
}

impl GitSetup {
    pub fn new() -> Result<Self> {
        let dir = TempDir::new("gnotes_repos")?;

        let repo_path = dir.path().join("notes_repo");

        fs::create_dir_all(repo_path.join(DEFAULT_NOTES_DIR_NAME))?;
        fs::write(
            repo_path
                .join(DEFAULT_NOTES_DIR_NAME)
                .join(DEFAULT_NOTE_FILE_NAME),
            "file content\n",
        )
        .expect("Failed to write file content");

        let repo_path = fs::canonicalize(&repo_path)?;

        GitSetup::run_git_command(&repo_path, &["init", "-b", "master"])?;
        GitSetup::run_git_command(&repo_path, &["add", "."])?;
        GitSetup::run_git_command(&repo_path, &["commit", "-m", "initial commit"])?;

        Ok(Self { dir, repo_path })
    }

    fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<()> {
        let mut cmd = Command::new("git")
            .args(args)
            .current_dir(&repo_path)
            .spawn()?;

        cmd.wait()?;

        Ok(())
    }
}
