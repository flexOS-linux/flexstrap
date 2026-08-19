mod cli;
mod fhs;
mod profile;
mod package;
mod db;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    println!("=== flexstrap v0.1.0 ===");
    println!("Target: {}", args.target.display());

    fhs::create_fhs(&args.target)?;
    let prof = profile::Profile::load(&args.profile, args.repo.as_deref())?;
    for pkg in &prof.packages {
        package::install_package(pkg, &args.target, args.repo.as_deref())?;
    }
    db::generate_installed_db(&args.target)?;

    println!("===> Bootstrapping complete!");
    Ok(())
}
