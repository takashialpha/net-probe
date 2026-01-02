use super::error::AppError;
use super::{Context, Privilege};
use std::path::PathBuf;

pub trait ConfigPath {
    fn config_path(&self) -> Option<PathBuf>;
}

pub trait App {
    type Config: serde::de::DeserializeOwned + serde::Serialize + Default;
    type Cli: ConfigPath + Clone;

    fn privilege() -> Privilege {
        Privilege::User
    }

    fn run(&self, ctx: Context<Self::Config, Self::Cli>) -> Result<(), AppError>;
}
