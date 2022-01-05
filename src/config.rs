use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

const GNOTES_DIR_NAME: &'static str = ".gnotes";
const CONFIG_FILE_NAME: &'static str = ".gnotes.toml";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid config: '{0}'")]
    InvalidConfig(String),
}

#[derive(Debug, Deserialize)]
struct ExternalConfig {
    notes_dir: Option<PathBuf>,
    auto_save: Option<bool>,
    repository: Option<String>,
}

impl Default for ExternalConfig {
    fn default() -> Self {
        Self {
            notes_dir: None,
            auto_save: None,
            repository: None,
        }
    }
}

impl ExternalConfig {
    pub fn merge(self, other: ExternalConfig) -> Self {
        Self {
            notes_dir: self.notes_dir.or(other.notes_dir),
            auto_save: self.auto_save.or(other.auto_save),
            repository: self.repository.or(other.repository),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub notes_dir: PathBuf,
    pub auto_save: bool,
    pub repository: Option<String>,
}

fn load_config_from_env() -> ExternalConfig {
    match envy::prefixed("GNOTES_").from_env::<ExternalConfig>() {
        Ok(config) => config,
        _ => ExternalConfig::default(),
    }
}

fn load_config_from_file(home_dir: &Path) -> ExternalConfig {
    let config_file_path = home_dir.join(CONFIG_FILE_NAME);

    if let Ok(content) = std::fs::read_to_string(config_file_path) {
        if let Ok(result) = toml::from_str::<ExternalConfig>(&content) {
            return result;
        }
    }

    ExternalConfig::default()
}

pub fn load_config(home_dir: &Path) -> Result<Config, ConfigError> {
    let env_config = load_config_from_env();
    let file_config = load_config_from_file(&home_dir);

    let external_config = env_config.merge(file_config);

    let config = Config {
        notes_dir: external_config
            .notes_dir
            .unwrap_or(home_dir.join(GNOTES_DIR_NAME)),
        auto_save: external_config.auto_save.unwrap_or_default(),
        repository: external_config.repository,
    };

    if config.auto_save && config.repository.is_none() {
        Err(ConfigError::InvalidConfig(String::from(
            "repository is mandatory when auto_save is enabled",
        )))
    } else {
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use tempdir::TempDir;

    fn create_temp_dir() -> TempDir {
        TempDir::new("gnotes_config_test")
            .expect("config tests: Failed to create a temporary directory")
    }

    fn write_config_file(home_dir: &TempDir, content: String) {
        let config_file = home_dir.path().join(".gnotes.toml");

        fs::write(config_file, format!("{}", content)).expect("Failed to write config file");
    }

    fn with_env_var<F>(key: &str, value: &str, f: F)
    where
        F: Fn(),
    {
        env::set_var(key, value);

        f();

        env::remove_var(key);
    }

    #[test]
    #[serial]
    fn test_notes_dir_default() {
        let home_dir = create_temp_dir();

        let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

        assert_eq!(config.notes_dir, home_dir.path().join(".gnotes"));
    }

    #[test]
    #[serial]
    fn test_auto_save_default() {
        let home_dir = create_temp_dir();

        let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

        assert_eq!(config.auto_save, false);
    }

    #[test]
    #[serial]
    fn test_repository_default() {
        let home_dir = create_temp_dir();

        let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

        assert_eq!(config.repository, None);
    }

    #[test]
    #[serial]
    fn test_notes_dir_from_config_file() {
        let home_dir = create_temp_dir();
        let notes_dir = home_dir.path().join("custom-notes-dir");

        write_config_file(
            &home_dir,
            format!("notes_dir = \"{}\"", notes_dir.to_str().unwrap()),
        );

        let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

        assert_eq!(config.notes_dir, notes_dir);
    }

    #[test]
    #[serial]
    fn test_auto_save_from_config_file() {
        let home_dir = create_temp_dir();

        write_config_file(
            &home_dir,
            String::from("auto_save = true\nrepository = \"something\""),
        );

        let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

        assert!(config.auto_save);
    }

    #[test]
    #[serial]
    fn test_repository_from_config_file() {
        let home_dir = create_temp_dir();

        write_config_file(&home_dir, String::from("repository = \"abc\""));

        let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

        assert_eq!(config.repository, Some(String::from("abc")));
    }

    #[test]
    #[serial]
    fn test_notes_dir_from_env() {
        let home_dir = create_temp_dir();
        let notes_dir = home_dir.path().join("custom-notes-dir");

        write_config_file(&home_dir, String::from("notes_dir = \"whatever\""));

        with_env_var("GNOTES_NOTES_DIR", notes_dir.to_str().unwrap(), || {
            let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

            assert_eq!(config.notes_dir, notes_dir);
        })
    }

    #[test]
    #[serial]
    fn test_auto_save_from_env() {
        let home_dir = create_temp_dir();

        write_config_file(
            &home_dir,
            String::from("auto_save = false\nrepository = \"something\""),
        );

        with_env_var("GNOTES_AUTO_SAVE", "true", || {
            let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

            assert!(config.auto_save);
        })
    }

    #[test]
    #[serial]
    fn test_repository_from_env() {
        let home_dir = create_temp_dir();

        write_config_file(&home_dir, String::from("repository = \"whatever\""));

        with_env_var("GNOTES_REPOSITORY", "abc", || {
            let config = load_config(home_dir.as_ref()).expect("Couldn't load config");

            assert_eq!(config.repository, Some(String::from("abc")));
        })
    }

    #[test]
    #[serial]
    fn test_auto_save_on_without_repository() {
        let home_dir = create_temp_dir();

        write_config_file(&home_dir, String::from("auto_save = true"));

        assert!(matches!(
            load_config(home_dir.as_ref()),
            Err(ConfigError::InvalidConfig(_))
        ));
    }
}
