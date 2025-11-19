//! Configuration management
//!
//! Handles loading and saving app configurations.

use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

/// Get the standard config directory for wonderland apps
pub fn config_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "wonderland")
        .map(|d| d.config_dir().to_path_buf())
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config").join("wonderland")
        })
}

/// Load a TOML config file
pub fn load<T: DeserializeOwned>(name: &str) -> Result<T, ConfigError> {
    let path = config_dir().join(format!("{}.toml", name));
    let content = std::fs::read_to_string(&path)
        .map_err(|e| ConfigError::Io(path.clone(), e))?;
    toml::from_str(&content).map_err(|e| ConfigError::Parse(path, e))
}

/// Save a TOML config file
pub fn save<T: Serialize>(name: &str, config: &T) -> Result<(), ConfigError> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| ConfigError::Io(dir.clone(), e))?;

    let path = dir.join(format!("{}.toml", name));
    let content = toml::to_string_pretty(config)
        .map_err(|e| ConfigError::Serialize(e))?;

    std::fs::write(&path, content)
        .map_err(|e| ConfigError::Io(path, e))
}

/// Load config or create default
pub fn load_or_default<T: DeserializeOwned + Default + Serialize>(name: &str) -> T {
    match load(name) {
        Ok(config) => config,
        Err(_) => {
            let config = T::default();
            let _ = save(name, &config);
            config
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error at {0}: {1}")]
    Io(PathBuf, std::io::Error),

    #[error("Parse error in {0}: {1}")]
    Parse(PathBuf, toml::de::Error),

    #[error("Serialization error: {0}")]
    Serialize(toml::ser::Error),
}
