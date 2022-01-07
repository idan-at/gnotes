use serde_json::Value;
use std::fs;
use std::path::Path;
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
}

// See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn read_to_json(path: &Path) -> Value {
    let content = fs::read_to_string(path).unwrap();

    serde_json::from_str::<Value>(&content).unwrap()
}
