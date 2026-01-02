pub mod error;

use crate::signals::SignalHandler;
use error::AppError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Context<C, A> {
    pub config: C,
    pub args: A,
    pub signals: SignalHandler,
    config_path: Option<PathBuf>,
    config_opts: Option<crate::config::TomlOptions>,
}

impl<C, A> Context<C, A> {
    pub(crate) fn new(
        config: C,
        args: A,
        signals: SignalHandler,
        config_path: Option<PathBuf>,
        config_opts: Option<crate::config::TomlOptions>,
    ) -> Self {
        Self {
            config,
            args,
            signals,
            config_path,
            config_opts,
        }
    }
}

impl<C, A> Context<C, A>
where
    C: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    pub fn reload_config(&mut self) -> Result<(), AppError>
    where
        C: serde::de::DeserializeOwned + serde::Serialize + Default,
    {
        let (path, opts) = match (&self.config_path, &self.config_opts) {
            (Some(p), Some(o)) => (Some(p.clone()), o.clone()),
            _ => {
                tracing::debug!(
                    target: "app_base::config",
                    "reload requested but no config is configured"
                );
                return Ok(());
            }
        };

        let new_config = crate::config::load::<C>(path, opts)?;
        self.config = new_config;

        tracing::debug!(target: "app_base::config", "configuration reloaded");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Privilege {
    User,
    Root,
}

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

#[derive(Debug, Clone)]
pub struct AppConfigLocation {
    pub app_name: String,
    pub config_dir: Option<PathBuf>,
}

impl AppConfigLocation {
    pub fn new(app_name: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
            config_dir: None,
        }
    }

    pub fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dir = Some(dir.into());
        self
    }

    pub fn to_toml_options(&self) -> crate::config::TomlOptions {
        crate::config::TomlOptions {
            app_name: self.app_name.clone(),
            config_dir: self.config_dir.clone(),
        }
    }
}
