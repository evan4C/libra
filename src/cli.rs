use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "libra")]
#[command(about = "A terminal-based digital library")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a book or article to the library
    Add { path: PathBuf },

    /// List all items in the library
    List,

    /// Search the library by title
    Search { query: String },
}
