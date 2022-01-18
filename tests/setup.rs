// See https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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
    pub bare_dir: TempDir,
    pub clone_dir: TempDir,
}

pub struct GitSetupOptions {
    pub with_changes: bool,
}

impl Default for GitSetupOptions {
    fn default() -> Self {
        Self { with_changes: true }
    }
}

impl GitSetup {
    pub fn new(options: Option<GitSetupOptions>) -> Result<Self> {
        let options = options.unwrap_or_default();

        let bare_dir = GitSetup::create_bare_repository()?;
        let clone_dir = GitSetup::create_clone_repository(bare_dir.path(), options.with_changes)?;

        Ok(Self {
            bare_dir,
            clone_dir,
        })
    }

    pub fn build_note_path(path: &Path) -> PathBuf {
        path.join(DEFAULT_NOTES_DIR_NAME)
            .join(DEFAULT_NOTE_FILE_NAME)
    }

    pub fn clone_to(from: &Path, to: &Path) -> Result<()> {
        let from_str = from.to_str().context("from.to_str()")?;

        GitSetup::run_git_command(to, &["clone", &from_str, "."])?;

        Ok(())
    }

    pub fn update_clone(&self) -> Result<()> {
        GitSetup::run_git_command(self.clone_dir.path(), &["pull"])?;

        Ok(())
    }

    fn create_bare_repository() -> Result<TempDir> {
        let base_dir = TempDir::new("gnotes_bare_repo")?;

        GitSetup::run_git_command(base_dir.path(), &["init", "--bare", "-b", "main"])?;

        Ok(base_dir)
    }

    fn create_clone_repository(bare_dir: &Path, with_changes: bool) -> Result<TempDir> {
        let clone_dir = TempDir::new("gnotes_repo_clone")?;

        GitSetup::clone_to(bare_dir, clone_dir.path())?;

        if with_changes {
            fs::create_dir(clone_dir.path().join(DEFAULT_NOTES_DIR_NAME))?;
            fs::write(
                GitSetup::build_note_path(clone_dir.path()),
                "file content\n",
            )
            .context("Failed to write file content")?;

            GitSetup::run_git_command(clone_dir.path(), &["add", "."])?;
            GitSetup::run_git_command(clone_dir.path(), &["commit", "-m", "initial commit"])?;
            GitSetup::run_git_command(clone_dir.path(), &["push"])?;
        }

        Ok(clone_dir)
    }

    fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<()> {
        Command::new("git")
            .args(args)
            .current_dir(&repo_path)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(())
    }
}
