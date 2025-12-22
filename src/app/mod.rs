use crate::cli::CliArgs;
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

fn assert_privilege(required: Privilege) {
    if required == Privilege::Root {
        #[cfg(unix)]
        {
            if unsafe { libc::geteuid() } != 0 {
                eprintln!("this application must be run as root");
                std::process::exit(1);
            }
        }

        #[cfg(not(unix))]
        {
            eprintln!(
                "your platform is not supported. if you want to run this program, you must disable assert_privilege, which may break the application."
            );
            std::process::exit(1);
        }
    }
}

pub fn run<A: App>(app: A, cfg: AppConfigLocation, cli_args: CliArgs) -> Result<(), AppError> {
    assert_privilege(A::privilege());

    let opts = cfg.to_toml_options();
    let config = config::load::<A::Config>(cli_args.init.config, opts)?;

    let ctx = Context {
        config,
        runtime: cli_args.runtime,
    };

    app.run(ctx)
}
