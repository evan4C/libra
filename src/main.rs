mod cli;
mod storage;

use std::fs;

use anyhow::{Context, Result, bail};
use clap::Parser;

use cli::{Cli, Command};
use storage::Storage;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let data_dir = dirs::data_local_dir()
        .context("Cannot determine application data directory")?
        .join("libra");

    fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("library.db");
    let storage = Storage::open(&db_path)?;

    match cli.command {
        Command::Add { path } => {
            let path = fs::canonicalize(&path)
                .with_context(|| format!("Cannot access {}", path.display()))?;

            if !path.is_file() {
                bail!("Not a regular file: {}", path.display());
            }

            let format = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();

            if !matches!(format.as_str(), "epub" | "pdf" | "md") {
                bail!("Unsupported format: {format}");
            }

            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .context("Invalid UTF-8 file name")?;

            storage.add(title, &path, &format)?;

            println!("Added: {title}");
        }

        Command::List => {
            for item in storage.list()? {
                println!("{:>4}  [{:<4}] {}", item.id, item.format, item.title);
            }
        }

        Command::Search { query } => {
            for item in storage.search(&query)? {
                println!("{:>4}  [{:<4}] {}", item.id, item.format, item.title);
            }
        }
    }

    Ok(())
}
