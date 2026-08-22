use std::fs::{self, File};
use std::path::Path;
use anyhow::{Context, Result};
use tar::Archive;
use zstd::stream::read::Decoder;

pub fn install_package(pkg_name: &str, target: &Path, repo: Option<&Path>) -> Result<()> {
    println!("  -> Installing package: {}", pkg_name);

    if let Some(repo_dir) = repo {
        let fpk_path = repo_dir.join(format!("{}.fpk", pkg_name));
        unpack_fpk(&fpk_path, target, pkg_name)?;
    } else {
        println!("     [recipe mode] Building {} from source...", pkg_name);
        // todo: fetch_recipe_and_build(pkg_name)
    }

    Ok(())
}

fn unpack_fpk(fpk_path: &Path, target: &Path, pkg_name: &str) -> Result<()> {
    let file = File::open(fpk_path)
        .with_context(|| format!("Failed to open package: {}", fpk_path.display()))?;
    let zstd_decoder = Decoder::new(file)?;
    let mut archive = Archive::new(zstd_decoder);

    let manifests_dir = target.join("var/lib/fpm/manifests");
    fs::create_dir_all(&manifests_dir)?;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();

        if path == Path::new("manifest.json") {
            let manifest_dest = manifests_dir.join(format!("{}.json", pkg_name));
            entry.unpack(&manifest_dest)?;
        } else if let Ok(rel_path) = path.strip_prefix("fs") {
            if rel_path.as_os_str().is_empty() {
                continue;
            }
            let dest_path = target.join(rel_path);
            
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            entry.unpack(&dest_path)?;
        }
    }

    Ok(())
}
