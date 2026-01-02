use crate::config::TomlOptions;
use std::path::PathBuf;

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

    pub fn to_toml_options(&self) -> TomlOptions {
        TomlOptions {
            app_name: self.app_name.clone(),
            config_dir: self.config_dir.clone(),
        }
    }
}
