use crate::config::ConfigError;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write_default<T: serde::Serialize>(path: &Path, defaults: &T) -> Result<(), ConfigError> {
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
    Ok(())
}
