use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "{}", e),
            AppError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Serialize(toml::ser::Error),
    ConfigDirNotFound,

    InvalidToml {
        path: PathBuf,
        source: toml::de::Error,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => {
                write!(f, "filesystem error: {}", err)
            }

            ConfigError::Serialize(err) => {
                write!(f, "failed to serialize default config: {}", err)
            }

            ConfigError::ConfigDirNotFound => {
                write!(f, "unable to determine configuration directory")
            }

            ConfigError::InvalidToml { path, source } => {
                write!(
                    f,
                    "invalid TOML in config file {}\n{}",
                    path.display(),
                    source
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}
