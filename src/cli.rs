use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "flexstrap", version, about = "Official flexOS system bootstrapper")]
pub struct Cli {
    pub target: PathBuf,

    #[arg(short, long, default_value = "base")]
    pub profile: String,

    #[arg(short, long)]
    pub repo: Option<PathBuf>,
}
