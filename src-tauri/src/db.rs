use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub fn database_path(app_data_dir: PathBuf) -> PathBuf {
    app_data_dir.join("recruiting-workspace.sqlite3")
}

pub fn open(app_data_dir: PathBuf) -> Result<Connection> {
    fs::create_dir_all(&app_data_dir).map_err(|_| rusqlite::Error::InvalidPath(app_data_dir.clone()))?;
    let conn = Connection::open(database_path(app_data_dir))?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<()> {
    conn.execute_batch(include_str!("../migrations/0001_initial.sql"))?;
    Ok(())
}
