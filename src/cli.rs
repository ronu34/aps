use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aps")]
#[command(version = "0.1")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        package: String,
    },

    Remove {
        name: String,
    },

    List,

    Build {
        path: String,
    },

    Search {
    query: String,
    }
}