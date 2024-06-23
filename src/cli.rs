use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub folder: PathBuf,

    #[arg(long = "max-depth", default_value = "50")]
    pub max_depth: usize,

    #[arg(short, default_value = "false")]
    pub enable_delete: bool,
}