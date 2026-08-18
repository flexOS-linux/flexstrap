use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use anyhow::{Context, Result};

pub fn create_fhs(target: &Path) -> Result<()> {
    println!("[+] Creating Merged-usr FHS hierarchy at {}", target.display());

    let dirs = [
        "dev", "proc", "sys", "etc", "run", "tmp", "root",
        "usr/bin", "usr/lib", "usr/lib64", "var/log",
        "var/lib/fpm/manifests", "var/cache/fpm/pkg",
    ];

    for dir in &dirs {
        fs::create_dir_all(target.join(dir))
            .with_context(|| format!("Failed to create directory: {}", dir))?;
    }

    let symlinks = [
        ("usr/bin", "bin"),
        ("usr/lib", "lib"),
        ("usr/lib64", "lib64"),
    ];

    for (src, dst) in &symlinks {
        let link_path = target.join(dst);
        if !link_path.exists() {
            symlink(src, &link_path)
                .with_context(|| format!("Failed to create symlink {} -> {}", dst, src))?;
        }
    }

    Ok(())
}
