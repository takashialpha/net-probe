use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub struct TomlOptions {
    pub app_name: String,
    pub file_name: String,
}

impl Default for TomlOptions {
    fn default() -> Self {
        Self {
            app_name: "app".into(),
            file_name: "config.toml".into(),
        }
    }
}

pub fn load<T>(cli_path: Option<PathBuf>, opts: TomlOptions) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    let path = match cli_path {
        Some(path) => path,
        None => default_path(&opts)?,
    };

    if !path.exists() {
        create_default(&path, &T::default())?;
    }

    let contents = std::fs::read_to_string(&path).map_err(ConfigError::Io)?;
    toml::from_str(&contents).map_err(ConfigError::Parse)
}

fn default_path(opts: &TomlOptions) -> Result<PathBuf, ConfigError> {
    let base = dirs::config_dir().ok_or(ConfigError::ConfigDirNotFound)?;
    Ok(base.join(&opts.app_name).join(&opts.file_name))
}

fn create_default<T: serde::Serialize>(path: &PathBuf, defaults: &T) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(ConfigError::Io)?;
    }

    let toml = toml::to_string_pretty(defaults).map_err(ConfigError::Serialize)?;

    let mut file = fs::File::create(path).map_err(ConfigError::Io)?;
    file.write_all(toml.as_bytes()).map_err(ConfigError::Io)
}
