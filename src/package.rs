use crate::manifest::Manifest;

#[derive(Debug)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
}

pub fn read_manifest(data: &str) -> anyhow::Result<Manifest> {
    let manifest: Manifest = toml::from_str(data)?;
    Ok(manifest)
}