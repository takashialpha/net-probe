use super::TomlOptions;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

pub fn resolve_path(cli_path: Option<PathBuf>, opts: &TomlOptions) -> PathBuf {
    cli_path.unwrap_or_else(|| default_path(opts))
}

fn default_path(opts: &TomlOptions) -> PathBuf {
    if let Some(dir) = &opts.config_dir {
        return dir.join(CONFIG_FILE);
    }

    let home = dirs::home_dir().expect("home directory unavailable");
    home.join(format!(".{}", opts.app_name)).join(CONFIG_FILE)
}
