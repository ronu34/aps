use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use tar::Archive;
use std::os::unix::fs::symlink;

use crate::db;
use crate::package;

// remove comment when using it actually(use sudo)
const INSTALL_DIR: &str = "/opt/aps";
//const INSTALL_DIR: &str = "/tmp/aps_root";

pub fn install_package(path: &str) -> Result<()> {
    println!("Installing {}", path);

    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let temp_dir = "/tmp/aps_extract";

    if Path::new(temp_dir).exists() {
        fs::remove_dir_all(temp_dir)?;
    }

    fs::create_dir_all(temp_dir)?;

    archive.unpack(temp_dir)?;

    let manifest_path = format!("{}/aps.toml", temp_dir);

    let mut manifest_file = File::open(&manifest_path)?;
    let mut contents = String::new();

    manifest_file.read_to_string(&mut contents)?;

    let manifest = package::read_manifest(&contents)?;

    println!("Package: {}", manifest.package.name);
    println!("Version: {}", manifest.package.version);

    let files_dir = format!("{}/files", temp_dir);

    copy_dir_all(&files_dir, INSTALL_DIR, &manifest.package.name)?;

    db::add_package(&manifest.package.name, &manifest.package.version)?;

    println!("Installed successfully");

    Ok(())
}

fn copy_dir_all(src: &str, dst: &str, package: &str) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        let src_path = entry.path();
        let dst_path = Path::new(dst).join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(
                src_path.to_str().unwrap(),
                dst_path.to_str().unwrap(),
                package,
            )?;
        } else {
            fs::copy(&src_path, &dst_path)?;

            db::add_file(package, dst_path.to_str().unwrap())?;

            // Auto-create symlink for executables in usr/bin
            let dst_str = dst_path.to_string_lossy();

            if dst_str.starts_with("/opt/aps/usr/bin/") {
                let binary_name = dst_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy();

                let symlink_path =
                    format!("/usr/local/bin/{}", binary_name);

                if Path::new(&symlink_path).exists() {
                    fs::remove_file(&symlink_path)?;
                }

                symlink(&dst_path, &symlink_path)?;

                db::add_file(package, &symlink_path)?;

                println!(
                    "Linked {} -> {}",
                    symlink_path,
                    dst_path.display()
                );
            }

            println!("Installed {}", dst_path.display());
        }
    }

    Ok(())
}
