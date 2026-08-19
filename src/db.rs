use std::fs::{self, File};
use std::path::Path;
use anyhow::Result;
use serde_json::{Value, json};

pub fn generate_installed_db(target: &Path) -> Result<()> {
    println!("[+] Generating fpm database (installed.json)...");
    
    let manifests_dir = target.join("var/lib/fpm/manifests");
    let mut db = json!({});

    if manifests_dir.exists() {
        for entry in fs::read_dir(manifests_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                let file = File::open(&path)?;
                let manifest: Value = serde_json::from_reader(file)?;
                
                if let Some(name) = manifest.get("name").and_then(|n| n.as_str()) {
                    db[name] = manifest;
                }
            }
        }
    }

    let db_path = target.join("var/lib/fpm/installed.json");
    let db_file = File::create(db_path)?;
    serde_json::to_writer_pretty(db_file, &db)?;

    println!("[+] Database updated successfully.");
    Ok(())
}
