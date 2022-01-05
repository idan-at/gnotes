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
