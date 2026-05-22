use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::Path;
use tar::Builder;

use crate::package;

pub fn build_package(path: &str) -> Result<()> {
    let manifest_path = format!("{}/aps.toml", path);

    let manifest_contents = std::fs::read_to_string(&manifest_path)?;

    let manifest = package::read_manifest(&manifest_contents)?;

    let output_name = format!("{}.aps", manifest.package.name);

    println!("Building {}", output_name);

    let tar_gz = File::create(&output_name)?;

    let encoder = GzEncoder::new(tar_gz, Compression::default());

    let mut tar = Builder::new(encoder);

    tar.append_dir_all(".", Path::new(path))?;

    tar.finish()?;

    println!("Built {}", output_name);

    Ok(())
}