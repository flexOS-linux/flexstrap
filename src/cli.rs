use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "flexstrap", version = "0.1.0", about = "flexOS system bootstrapper")]
pub struct Cli {
    pub target: PathBuf,

    #[arg(short, long, default_value = "base")]
    pub profile: String,

    #[arg(short, long)]
    pub repo: Option<PathBuf>,
}
