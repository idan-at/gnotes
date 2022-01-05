use assert_cmd::Command;
use tempdir::TempDir;

struct Setup {
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

#[test]
fn test_new_note() {
    let setup = Setup::new();

    let mut cmd = Command::cargo_bin("gnotes").unwrap();

    cmd.arg("new")
        .env("GNOTES_NOTES_DIR", setup.dir.as_ref())
        .assert()
        .success();
}
