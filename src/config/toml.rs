use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TomlOptions {
    pub app_name: String,
    pub config_dir: Option<PathBuf>,
}
