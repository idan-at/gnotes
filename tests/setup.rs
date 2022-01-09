// See https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use tempdir::TempDir;

pub struct Setup {
    pub dir: TempDir,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            dir: TempDir::new("gnotes_test")
                .expect("new_note_test: Failed to create a temporary directory"),
        }
    }

    pub fn dir_path(&self) -> &Path {
        self.dir.path()
    }

    pub fn note_parent_dir(&self, dir: &str) -> PathBuf {
        self.dir_path().join(dir)
    }

    pub fn default_note_parent_dir(&self) -> PathBuf {
        self.note_parent_dir("notes")
    }

    pub fn note_path(&self, dir: &str) -> PathBuf {
        self.note_parent_dir(dir).join("chores")
    }

    pub fn default_note_path(&self) -> PathBuf {
        self.note_path("notes")
    }
}
