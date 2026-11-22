use std::path::Path;

use anyhow::Result;
use rusqlite::{Connection, params};

pub struct LibraryItem {
    pub id: i64,
    pub title: String,
    pub format: String,
}

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                format TEXT NOT NULL,
                added_at TEXT NOT NULL
                    DEFAULT CURRENT_TIMESTAMP
            );",
        )?;

        Ok(Self { conn })
    }

    pub fn add(&self, title: &str, path: &Path, format: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO items (title, path, format)
             VALUES (?1, ?2, ?3)",
            params![title, path.to_string_lossy(), format],
        )?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<LibraryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, format
             FROM items ORDER BY id",
        )?;

        let items = stmt.query_map([], |row| {
            Ok(LibraryItem {
                id: row.get(0)?,
                title: row.get(1)?,
                format: row.get(2)?,
            })
        })?;

        Ok(items.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn search(&self, query: &str) -> Result<Vec<LibraryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, format FROM items
             WHERE instr(lower(title), lower(?1)) > 0
             ORDER BY id",
        )?;

        let items = stmt.query_map([query], |row| {
            Ok(LibraryItem {
                id: row.get(0)?,
                title: row.get(1)?,
                format: row.get(2)?,
            })
        })?;

        Ok(items.collect::<Result<Vec<_>, _>>()?)
    }
}
