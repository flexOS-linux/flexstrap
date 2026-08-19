use std::path::Path;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub packages: Vec<String>,
}

impl Profile {
    pub fn load(name: &str, _repo_path: Option<&Path>) -> Result<Self> {
        println!("[+] Loading profile '{}'...", name);
        
        // todo: profiles logic
        
        Ok(Self {
            name: name.to_string(),
            packages: vec![
                "base-files".into(),
                "glibc".into(),
                "bash".into(),
                "coreutils".into(),
                "systemd".into(),
            ],
        })
    }
}
