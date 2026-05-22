mod cli;
mod db;
mod install;
mod manifest;
mod package;
mod remove;
mod build;
mod repo;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    db::init()?;

    match cli.command {
        Commands::Install { package } => {
        let path = if package.ends_with(".aps") {
                package
            } else {
                repo::find_package(&package)?
            };

        install::install_package(&path)?;
        }
        Commands::Remove { name } => {
            remove::remove_package(&name)?;
        }

        Commands::Build { path } => {
            build::build_package(&path)?;
        }

        Commands::List => {
            let packages = db::list_packages()?;

            for pkg in packages {
                println!("{} {}", pkg.name, pkg.version);
            }
        }

        Commands::Search { query } => {
            repo::search_packages(&query)?;
        }
    }

    Ok(())
}