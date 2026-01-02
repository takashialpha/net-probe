use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to serialize default config: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("unable to determine configuration directory")]
    ConfigDirNotFound,

    #[error("invalid TOML in config file {path}\n{source}")]
    InvalidToml {
        path: PathBuf,
        source: toml::de::Error,
    },
}
