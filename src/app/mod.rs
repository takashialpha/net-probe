use crate::cli;
use crate::config;
use crate::error::AppError;
use std::path::PathBuf;

pub mod context {
    use crate::cli::RuntimeArgs;

    #[derive(Debug)]
    pub struct Context<C> {
        pub config: C,
        pub runtime: RuntimeArgs,
    }
}

pub use context::Context;

pub trait App {
    type Config: serde::de::DeserializeOwned + serde::Serialize + Default;

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

pub fn run<A: App>(app: A, cfg: AppConfigLocation) -> Result<(), AppError> {
    let cli_args = cli::parse();

    let opts = cfg.to_toml_options();

    let config = config::load::<A::Config>(cli_args.init.config, opts)?;

    let ctx = Context {
        config,
        runtime: cli_args.runtime,
    };

    app.run(ctx)
}
