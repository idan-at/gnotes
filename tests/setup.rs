// See https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// TODO: split to multiple files
use anyhow::{Context, Result};
use assert_cmd::assert::Assert;
use gnotes::config::Config;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempdir::TempDir;

pub const DEFAULT_NOTES_DIR_NAME: &'static str = "notes";
pub const DEFAULT_NOTE_FILE_NAME: &'static str = "chores";

pub struct Setup {
    home_dir: TempDir,
    notes_dir: TempDir,
}

pub struct RunOptions {
    pub stdin: Option<String>,
    pub repository: Option<PathBuf>,
}

impl Default for RunOptions {
    fn default() -> Self {
        RunOptions {
            stdin: None,
            repository: None,
        }
    }
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
