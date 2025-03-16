use std::path::PathBuf;

const DB_FILENAME: &str = "journal.db";

pub fn connect() -> rusqlite::Connection {
    rusqlite::Connection::open(db_path()).unwrap()
}

pub fn db_path() -> PathBuf {
    dirs::config_dir()
        .or_else(dirs::home_dir)
        .expect("Directory not found for database file")
        .join(DB_FILENAME)
}
