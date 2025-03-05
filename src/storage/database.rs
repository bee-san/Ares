///! Module for managing the SQLite database
///!
///! This database is intended for caching known encoded/decoded string
///! relations and collecting statistics on the performance of Ares
///! search algorithms.
use rusqlite::Connection;

/// Returns the path to the database file
pub fn get_database_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push("Ares");
    path.push("database.sqlite");
    path
}

/// Initializes database with default schema
pub fn setup_database(path: &std::path::Path) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(path)?;

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

    // Sets up database path specifically for testing
    fn get_test_database_path() -> std::path::PathBuf {
        let mut path = std::path::PathBuf::new();
        path.push("./test_database.sqlite");
        path
    }

    struct TestDatabase {
        pub path: std::path::PathBuf,
    }

    impl Default for TestDatabase {
        fn default() -> Self {
            TestDatabase {
                path: get_test_database_path(),
            }
        }
    }

    impl Drop for TestDatabase {
        fn drop(&mut self) {
            std::fs::remove_file(&self.path).unwrap();
        }
    }

    #[test]
    fn database_initialized() {
        let test_db = TestDatabase::default();
        let db_result = setup_database(&test_db.path);
        assert!(db_result.is_ok());
        let conn_result = Connection::open(&test_db.path);
        assert!(conn_result.is_ok());
    }

    #[test]
    fn cache_table_created() {
        let test_db = TestDatabase::default();
        let _ = setup_database(&test_db.path);

        let conn = Connection::open(&test_db.path).unwrap();
        let stmt_result =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='cache';");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let query_result = stmt.query_map([], |row| row.get::<usize, String>(0));
        assert!(query_result.is_ok());
        assert_eq!(query_result.unwrap().count(), 1);
    }
}
