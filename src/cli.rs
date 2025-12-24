use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args, Clone)]
pub struct InitArgs {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Args, Default, Clone)]
pub struct RuntimeArgs {}

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub init: InitArgs,
    pub runtime: RuntimeArgs,
}

impl CliArgs {
    pub fn new(init: InitArgs, runtime: RuntimeArgs) -> Self {
        Self { init, runtime }
    }
}
