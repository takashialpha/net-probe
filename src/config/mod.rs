mod error;
mod toml;

pub use error::ConfigError;
pub use toml::{TomlOptions, load};
