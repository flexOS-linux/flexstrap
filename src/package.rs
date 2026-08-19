use std::fs::File;
use std::path::Path;
use anyhow::{Context, Result};
use tar::Archive;
use zstd::stream::read::Decoder;

pub fn install_package(pkg_name: &str, target: &Path, repo: Option<&Path>) -> Result<()> {
    println!("  -> Installing package: {}", pkg_name);

    if let Some(_repo_dir) = repo {
        let fpk_path = target.join(format!("{}.fpk", pkg_name));
        unpack_fpk(&fpk_path, target)?;
    } else {
        println!("     [recipe mode] Building {} from source...", pkg_name);
        // todo: fetch_recipe_and_build(pkg_name)
    }

    Ok(())
}

fn unpack_fpk(fpk_path: &Path, target: &Path) -> Result<()> {
    let file = File::open(fpk_path)
        .with_context(|| format!("Failed to open package: {}", fpk_path.display()))?;
    let zstd_decoder = Decoder::new(file)?;
    let mut archive = Archive::new(zstd_decoder);

    archive.unpack(target)
        .with_context(|| format!("Failed to unpack {}", fpk_path.display()))?;

    Ok(())
}
