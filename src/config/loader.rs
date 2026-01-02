use super::{path::resolve_path, writer::write_default};
use crate::config::{ConfigError, TomlOptions};
use std::fs;
use std::path::PathBuf;

pub fn load<T>(cli_path: Option<PathBuf>, opts: TomlOptions) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    let path = resolve_path(cli_path, &opts);

    if !path.exists() {
        tracing::debug!(target: "app_base::config", path = ?path, "config file not found");
        write_default(&path, &T::default())?;
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
