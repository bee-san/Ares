///! Module for managing the SQLite database
///!
///! This database is intended for caching known encoded/decoded string
///! relations and collecting statistics on the performance of Ares
///! search algorithms.
use rusqlite::Connection;

/// Returns the path to the database file
fn get_database_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push("Ares");
    path.push("database.sqlite");
    path
}

/// Public wrapper for setting up database
pub fn setup_database() -> Result<(), rusqlite::Error> {
    let path = get_database_path();
    let conn = Connection::open(&path)?;
    init_database(&conn)?;
    Ok(())
}

/// Initializes database with default schema
fn init_database(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    // Initializing cache table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            encoded_text TEXT NOT NULL,
            decoded_text TEXT NOT NULL,
            path JSON NOT NULL,
            successful BOOLEAN NOT NULL DEFAULT true,
            execution_time_ms INTEGER NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cache_encoded_text
            ON cache(encoded_text);",
        (),
    )?;

    // Initializing statistics table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS statistics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id TEXT NOT NULL,
            decoder_name TEXT NOT NULL,
            success_count INTEGER NOT NULL,
            total_attempts INTEGER NOT NULL,
            search_depth INTEGER NOT NULL,
            seen_strings_count INTEGER NOT NULL,
            prune_threshold INTEGER NOT NULL,
            max_memory_kb INTEGER NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stats_run_id ON statistics(run_id);",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stats_decoder ON statistics(decoder_name);",
        (),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn database_initialized() {
        let conn_result = Connection::open_in_memory();
        assert!(conn_result.is_ok());
        let conn = conn_result.unwrap();
        let db_result = init_database(&conn);
        assert!(db_result.is_ok());
    }

    #[test]
    fn cache_table_created() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = init_database(&conn);

        let stmt_result =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='cache';");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let query_result = stmt.query_map([], |row| row.get::<usize, String>(0));
        assert!(query_result.is_ok());
        assert_eq!(query_result.unwrap().count(), 1);
    }

    #[test]
    fn correct_cache_table_schema() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = init_database(&conn);

        let stmt_result = conn.prepare(
            "PRAGMA table_info(cache);"
        );
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let query_result = stmt.query_map([], |row| row.get::<usize, String>(0));
        assert!(query_result.is_ok());
        assert_eq!(query_result.unwrap().count(), 7);
    }
}
