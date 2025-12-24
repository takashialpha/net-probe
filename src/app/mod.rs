use crate::cli::RuntimeArgs;
use crate::error::AppError;
use crate::signals::SignalHandler;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Context<C> {
    pub config: C,
    pub runtime: RuntimeArgs,
    pub signals: SignalHandler,
    config_path: Option<PathBuf>,
    config_opts: crate::config::TomlOptions,
}

impl<C> Context<C> {
    pub(crate) fn new(
        config: C,
        runtime: RuntimeArgs,
        signals: SignalHandler,
        config_path: Option<PathBuf>,
        config_opts: crate::config::TomlOptions,
    ) -> Self {
        Self {
            config,
            runtime,
            signals,
            config_path,
            config_opts,
        }
    }
}

impl<C> Context<C>
where
    C: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    pub fn reload_config(&mut self) -> Result<(), AppError> {
        tracing::debug!(target: "app_base::config", "reloading configuration");
        let new_config =
            crate::config::load::<C>(self.config_path.clone(), self.config_opts.clone())?;
        self.config = new_config;
        tracing::debug!(target: "app_base::config", "configuration reloaded successfully");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Privilege {
    User,
    Root,
}

pub trait App {
    type Config: serde::de::DeserializeOwned + serde::Serialize + Default;

    fn privilege() -> Privilege {
        Privilege::User
    }

    fn run(&self, ctx: Context<Self::Config>) -> Result<(), AppError>;
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
