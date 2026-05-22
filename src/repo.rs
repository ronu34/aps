use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoPackage {
    pub version: String,
    pub file: String,
}

pub type RepoIndex = HashMap<String, RepoPackage>;

const REPO_PATH: &str = "./repo";

pub fn find_package(name: &str) -> Result<String> {
    let index_path = format!("{}/index.json", REPO_PATH);

    let contents = fs::read_to_string(index_path)?;

    let index: RepoIndex = serde_json::from_str(&contents)?;

    let package = index
        .get(name)
        .ok_or_else(|| anyhow!("Package not found"))?;

    let package_path = format!("{}/{}", REPO_PATH, package.file);

    Ok(package_path)
}

pub fn search_packages(query: &str) -> Result<()> {
    let index_path = format!("{}/index.json", REPO_PATH);

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