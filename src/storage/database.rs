///! Module for managing the SQLite database
///!
///! This database is intended for caching known encoded/decoded string
///! relations and collecting statistics on the performance of Ares
///! search algorithms.
use rusqlite::Connection;

/// Initializes database with default schema
pub fn setup_database() -> Result<(), rusqlite::Error> {
    let db_path = "./database.sqlite";
    let conn = Connection::open(db_path)?;

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
    );", ())?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cache_encoded_text
            ON cache(encoded_text);"
    , ())?;

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
    );"
    , ())?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stats_run_id ON statistics(run_id);"
    , ())?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stats_decoder ON statistics(decoder_name);"
    , ())?;

    Ok(())
}

