use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[command(flatten)]
    pub init: InitArgs,

    #[command(flatten)]
    pub runtime: RuntimeArgs,
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct RuntimeArgs {
    #[arg(short, long)]
    pub verbose: bool,
}
