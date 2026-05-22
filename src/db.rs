use anyhow::Result;
use rusqlite::{params, Connection};

use crate::package::InstalledPackage;

const DB_PATH: &str = "aps.db";

pub fn init() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS packages (
            name TEXT PRIMARY KEY,
            version TEXT NOT NULL
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS files (
            package TEXT NOT NULL,
            path TEXT NOT NULL
        )
        ",
        [],
    )?;

    Ok(())
}

pub fn add_package(name: &str, version: &str) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "INSERT OR REPLACE INTO packages (name, version) VALUES (?1, ?2)",
        params![name, version],
    )?;

    Ok(())
}

pub fn add_file(package: &str, path: &str) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "INSERT INTO files (package, path) VALUES (?1, ?2)",
        params![package, path],
    )?;

    Ok(())
}

pub fn get_files(package: &str) -> Result<Vec<String>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(
        "SELECT path FROM files WHERE package = ?1"
    )?;

    let rows = stmt.query_map(params![package], |row| {
        row.get(0)
    })?;

    let mut files = Vec::new();

    for file in rows {
        files.push(file?);
    }

    Ok(files)
}


pub fn remove_package(name: &str) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "DELETE FROM packages WHERE name = ?1",
        params![name],
    )?;

    Ok(())
}

pub fn list_packages() -> Result<Vec<InstalledPackage>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(
        "SELECT name, version FROM packages"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(InstalledPackage {
            name: row.get(0)?,
            version: row.get(1)?,
        })
    })?;

    let mut packages = Vec::new();

    for pkg in rows {
        packages.push(pkg?);
    }

    Ok(packages)
}