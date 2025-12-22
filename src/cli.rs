use clap::Args;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliArgs {
    pub init: InitArgs,
    pub runtime: RuntimeArgs,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Args, Default)]
pub struct RuntimeArgs {}
