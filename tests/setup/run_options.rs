use std::path::PathBuf;

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
