use crate::cli;
use crate::config;
use crate::error::AppError;

mod context;
pub use context::Context;

pub trait App {
    type Config: serde::de::DeserializeOwned + serde::Serialize + Default;

    fn run(&self, ctx: Context<Self::Config>) -> Result<(), AppError>;
}

pub fn run<A: App>(app: A) -> Result<(), AppError> {
    let cli = cli::parse();

    let config = config::load::<A::Config>(cli.init.config, config::TomlOptions::default())?;

    let ctx = Context {
        config,
        runtime: cli.runtime,
    };

    app.run(ctx)
}
