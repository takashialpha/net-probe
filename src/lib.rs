#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(not(unix))]
compile_error!(
    "This library only supports Unix-like systems. Windows and other non-Unix platforms are not supported."
);

pub mod app;
pub mod cli;
pub mod config;
pub mod error;
pub mod signals;

pub use app::{App, AppConfigLocation, Context, Privilege};
pub use error::{AppError, ConfigError};
pub use signals::{Signal, SignalHandler};

use cli::CliArgs;

fn assert_privilege(required: Privilege) {
    if required == Privilege::Root && unsafe { libc::geteuid() } != 0 {
        eprintln!("ERROR: This application must be run as root.");
        std::process::exit(1);
    }
}

pub fn run<A: App>(app: A, cfg: Option<AppConfigLocation>, args: CliArgs) -> Result<(), AppError> {
    assert_privilege(A::privilege());

    let (config, config_opts) = match cfg {
        Some(cfg) => {
            let opts = cfg.to_toml_options();
            let config = config::load::<A::Config>(args.config.clone(), opts.clone())?;
            (config, Some(opts))
        }
        None => (A::Config::default(), None),
    };

    let signals = SignalHandler::new();

    let ctx = Context::new(
        config,
        args.clone(),
        signals,
        args.config.clone(),
        config_opts,
    );

    tracing::debug!(target: "app_base", "starting application");
    app.run(ctx)
}
