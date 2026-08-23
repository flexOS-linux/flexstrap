use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub packages: Vec<String>,
}

impl Profile {
    pub fn load(profile_name: &str, repo_path: Option<&Path>) -> Result<Self> {
        if let Some(repo) = repo_path {
            println!(
                "[+] Local repository provided at '{}'. Ignoring profile '{}'...",
                repo.display(),
                profile_name
            );

            if !repo.exists() {
                anyhow::bail!("Repository directory does not exist: {}", repo.display());
            }

            let mut packages = Vec::new();

            for entry in fs::read_dir(repo)
                .with_context(|| format!("Failed to read repo directory: {}", repo.display()))? 
            {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().map_or(false, |ext| ext == "fpk") {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        packages.push(file_stem.to_string());
                    }
                }
            }

            packages.sort();

            Ok(Self {
                name: format!("custom-repo ({})", repo.display()),
                packages,
            })
        } else {
            println!("[+] Loading profile '{}'...", profile_name);

            // todo: switch between profiles

            Ok(Self {
                name: profile_name.to_string(),
                packages: vec![
                    "glibc".into(),
                    "base-files".into(),
                    "bash".into(),
                    "coreutils".into(),
                ],
            })
        }
    }
}
