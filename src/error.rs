use std::fmt;

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
    ConfigDirNotFound,
    Io(std::io::Error),
    Parse(toml::de::Error),
    Serialize(toml::ser::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ConfigDirNotFound => write!(f, "configuration directory not found"),
            ConfigError::Io(e) => write!(f, "I/O error: {}", e),
            ConfigError::Parse(e) => write!(f, "TOML parse error: {}", e),
            ConfigError::Serialize(e) => write!(f, "TOML serialize error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}
