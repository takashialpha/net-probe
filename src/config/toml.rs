use crate::error::ConfigError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

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
    let path = cli_path.unwrap_or_else(|| default_path(&opts));

    if !path.exists() {
        tracing::debug!(
            target: "app_base::config",
            path = ?path,
            "config file not found"
        );
        create_default(&path, &T::default())?;
    }

    let contents = fs::read_to_string(&path).map_err(ConfigError::Io)?;
    toml::from_str(&contents).map_err(|err| {
        tracing::debug!(
            target: "app_base::config",
            path = ?path,
            error = %err,
            "invalid toml"
        );
        ConfigError::InvalidToml { path, source: err }
    })
}

fn default_path(opts: &TomlOptions) -> PathBuf {
    if let Some(dir) = &opts.config_dir {
        return dir.join(CONFIG_FILE);
    }

    let home = dirs::home_dir().expect("home directory unavailable");
    home.join(format!(".{}", opts.app_name)).join(CONFIG_FILE)
}

fn create_default<T: serde::Serialize>(path: &Path, defaults: &T) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(ConfigError::Io)?;
    }

    let toml = toml::to_string_pretty(defaults).map_err(ConfigError::Serialize)?;

    let tmp = path.with_extension("tmp");
    {
        let mut file = fs::File::create(&tmp).map_err(ConfigError::Io)?;
        file.write_all(toml.as_bytes()).map_err(ConfigError::Io)?;
        file.sync_all().map_err(ConfigError::Io)?;
    }

    fs::rename(tmp, path).map_err(ConfigError::Io)?;
    tracing::debug!(
        target: "app_base::config",
        path = ?path,
        "created default config"
    );
    Ok(())
}
