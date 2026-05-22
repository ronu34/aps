use anyhow::Result;
use std::fs;

use crate::db;

pub fn remove_package(name: &str) -> Result<()> {
    let files = db::get_files(name)?;

    for file in files {
        if fs::metadata(&file).is_ok() {
            println!("Removing {}", file);
            fs::remove_file(&file)?;
        }
    }

    db::remove_package(name)?;

    println!("Removed {}", name);

    Ok(())
}