use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoPackage {
    pub version: String,
    pub file: String,
    pub sha256: String,
}

pub type RepoIndex = HashMap<String, RepoPackage>;
const CACHE_DIR: &str = "/tmp/aps_repo";
const REPO_URL: &str =
    "https://raw.githubusercontent.com/ronu34/aps-repo/main";

use sha2::{Digest, Sha256};

fn calculate_sha256(path: &str) -> Result<String> {
    let data = std::fs::read(path)?;

    let mut hasher = Sha256::new();
    hasher.update(data);

    Ok(hex::encode(hasher.finalize()))
}

fn download_index() -> Result<String> {
    fs::create_dir_all(CACHE_DIR)?;

    let url = format!("{}/index.json", REPO_URL);

    println!("Downloading {}", url);

    let body = reqwest::blocking::get(&url)?
        .text()?;

    let path = format!("{}/index.json", CACHE_DIR);

    fs::write(&path, body)?;

    Ok(path)
}

pub fn find_package(name: &str) -> Result<String> {
    println!("Looking for package: {}", name);

    let index_path = download_index()?;
    println!("Downloaded index to: {}", index_path);

    let contents = fs::read_to_string(&index_path)?;
    println!("Index loaded");

    let index: RepoIndex = serde_json::from_str(&contents)?;
    println!("JSON parsed");

    let package = index
        .get(name)
        .ok_or_else(|| anyhow!("Package '{}' not found", name))?;

    println!("Found package file: {}", package.file);

    let url = format!("{}/{}", REPO_URL, package.file);

    println!("Downloading {}", url);

    let bytes = reqwest::blocking::get(&url)?
        .bytes()?;

    let package_path = format!("{}/{}.aps", CACHE_DIR, name);

    fs::write(&package_path, bytes)?;

    println!("Saved to {}", package_path);
    
    let actual = calculate_sha256(&package_path)?;

    if actual != package.sha256 {
    	return Err(anyhow!(
        "SHA256 verification failed for {}",
        name
    ));
}

    Ok(package_path)
}

pub fn search_packages(query: &str) -> Result<()> {
    let index_path = format!("{}/index.json", REPO_URL);

    let contents = fs::read_to_string(index_path)?;

    let index: RepoIndex = serde_json::from_str(&contents)?;

    println!("Search results:");

    for (name, package) in index {
        if name.contains(query) {
            println!("{} {}", name, package.version);
        }
    }

    Ok(())
}