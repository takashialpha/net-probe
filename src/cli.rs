use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct CliArgs {
    #[arg(long)]
    pub config: Option<PathBuf>,
}
