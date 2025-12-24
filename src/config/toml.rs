use crate::error::ConfigError;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone)]
pub struct TomlOptions {
    pub app_name: String,
    pub config_dir: Option<PathBuf>,
}

impl Default for TomlOptions {
    fn default() -> Self {
        Self {
            app_name: "rust_unnamed_app".into(),
            config_dir: None,
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
        tracing::debug!(target: "app_base::config", path = ?path, "config file not found, creating default");
        create_default(&path, &T::default())?;
    }

    tracing::debug!(target: "app_base::config", path = ?path, "loading config");
    let contents = fs::read_to_string(&path).map_err(ConfigError::Io)?;
    match toml::from_str(&contents) {
        Ok(cfg) => {
            tracing::debug!(target: "app_base::config", path = ?path, "config loaded successfully");
            Ok(cfg)
        }
        Err(err) => {
            tracing::debug!(target: "app_base::config", path = ?path, error = %err, "invalid toml");
            Err(ConfigError::InvalidToml { path, source: err })
        }
    }
}

fn default_path(opts: &TomlOptions) -> Result<PathBuf, ConfigError> {
    let dir = match &opts.config_dir {
        Some(dir) => dir.clone(),
        None => {
            let base = dirs::config_dir().ok_or(ConfigError::ConfigDirNotFound)?;
            base.join(format!(".{}", &opts.app_name))
        }
    };
    Ok(dir.join(CONFIG_FILE))
}

fn create_default<T: serde::Serialize>(path: &PathBuf, defaults: &T) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(ConfigError::Io)?;
    }
    let toml = toml::to_string_pretty(defaults).map_err(ConfigError::Serialize)?;
    let mut file = fs::File::create(path).map_err(ConfigError::Io)?;
    file.write_all(toml.as_bytes()).map_err(ConfigError::Io)?;
    tracing::debug!(target: "app_base::config", path = ?path, "created default config");
    Ok(())
}
