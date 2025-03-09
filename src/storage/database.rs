///! Module for managing the SQLite database
///!
///! This database is intended for caching known encoded/decoded string
///! relations and collecting statistics on the performance of Ares
///! search algorithms.
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Intermediary struct for serializing/deserializing CrackResults
/// into JSON format
///
/// It is the same as CrackResult, but it holds the Strings by value
/// instead of references
pub struct RawCrackResult {
    /// If our checkers return success, we change this bool to True
    pub success: bool,
    /// Encrypted text is the text _before_ we decrypt it.
    pub encrypted_text: String,
    /// Unencrypted text is what it looks like after.
    /// if decoder failed, this will be None
    pub unencrypted_text: Option<Vec<String>>,
    /// Decoder is the function we used to decode the text
    pub decoder: String,
    /// Checker which identified the text
    pub checker_name: String,
    /// Description is a short description of the checker
    pub checker_description: String,
    /// Key is optional as decoders do not use keys.
    pub key: Option<String>,
    /// Description is a short description of the decoder
    pub description: String,
    /// Link is a link to more info about the decoder
    pub link: String,
}

impl PartialEq for RawCrackResult {
    fn eq(&self, other: &Self) -> bool {
        self.success == other.success
            && self.encrypted_text == other.encrypted_text
            && self.unencrypted_text == other.unencrypted_text
            && self.decoder == other.decoder
            && self.checker_name == other.checker_name
            && self.checker_description == other.checker_description
            && self.key == other.key
            && self.description == other.description
            && self.link == other.link
    }
}

impl Clone for RawCrackResult {
    fn clone(&self) -> Self {
        RawCrackResult {
            success: self.success.clone(),
            encrypted_text: self.encrypted_text.clone(),
            unencrypted_text: self.unencrypted_text.clone(),
            decoder: self.decoder.clone(),
            checker_name: self.checker_description.clone(),
            checker_description: self.checker_description.clone(),
            key: self.key.clone(),
            description: self.description.clone(),
            link: self.link.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Struct representing a row in the cache table
pub struct CacheRow {
    /// Index of row in cache table
    pub id: usize,
    /// Text before it is decoded
    pub encoded_text: String,
    /// Text after it is decoded
    pub decoded_text: String,
    /// Ordered list of decoding attempts
    pub path: Vec<RawCrackResult>,
    /// Whether or not the decoding was successful
    pub successful: bool,
    /// How long the decoding took in milliseconds
    pub execution_time_ms: u64,
    /// When the decoding was run
    pub timestamp: String,
}

impl PartialEq for CacheRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.encoded_text == other.encoded_text
            && self.decoded_text == other.decoded_text
            && self.path == other.path
            && self.successful == other.successful
            && self.execution_time_ms == other.execution_time_ms
            && self.timestamp == other.timestamp
    }
}

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

/// Adds a new cache record to the cache table
pub fn add_row(
    conn: &rusqlite::Connection,
    encoded_text: &String,
    decoded_text: &String,
    path: &Vec<RawCrackResult>,
    successful: &bool,
    execution_time_ms: &u64,
    timestamp: &String,
) -> Result<(), rusqlite::Error> {
    let path_json = serde_json::to_string(&path.clone()).unwrap();
    let _conn_result = conn.execute(
        "INSERT INTO cache (
            encoded_text,
            decoded_text,
            path,
            successful,
            execution_time_ms,
            timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)",
        (
            encoded_text.clone(),
            decoded_text.clone(),
            path_json,
            successful.clone(),
            execution_time_ms.clone(),
            timestamp.clone(),
        ),
    );
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

        let stmt_result = conn.prepare("PRAGMA table_info(cache);");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let name_result = stmt.query_map([], |row| row.get::<usize, String>(1));
        assert!(name_result.is_ok());
        let name_query = name_result.unwrap();
        let name_list: Vec<String> = name_query.map(|row| row.unwrap()).collect();
        assert_eq!(name_list[0], "id");
        assert_eq!(name_list[1], "encoded_text");
        assert_eq!(name_list[2], "decoded_text");
        assert_eq!(name_list[3], "path");
        assert_eq!(name_list[4], "successful");
        assert_eq!(name_list[5], "execution_time_ms");
        assert_eq!(name_list[6], "timestamp");

        let type_result = stmt.query_map([], |row| row.get::<usize, String>(2));
        assert!(type_result.is_ok());
        let type_query = type_result.unwrap();
        let type_list: Vec<String> = type_query.map(|row| row.unwrap()).collect();
        assert_eq!(type_list[0], "INTEGER");
        assert_eq!(type_list[1], "TEXT");
        assert_eq!(type_list[2], "TEXT");
        assert_eq!(type_list[3], "JSON");
        assert_eq!(type_list[4], "BOOLEAN");
        assert_eq!(type_list[5], "INTEGER");
        assert_eq!(type_list[6], "DATETIME");
    }

    #[test]
    fn cache_record_empty_success() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = init_database(&conn);

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();
            let crack_result_vec: Vec<RawCrackResult> =
                serde_json::from_str(&path_str.clone()).unwrap();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: crack_result_vec,
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                timestamp: row.get_unwrap(6),
            })
        });
        assert!(query_result.is_ok());
        let empty_rows = query_result.unwrap();
        assert_eq!(empty_rows.count(), 0);
    }

    #[test]
    fn cache_record_entry_success() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = init_database(&conn);

        let mock_crack_result = RawCrackResult {
            success: true,
            encrypted_text: String::from("aGVsbG8gd29ybGQK"),
            unencrypted_text: Some(vec![String::from("hello world")]),
            decoder: String::from("Base64"),
            checker_name: String::from("Mock Checker"),
            checker_description: String::from("A mock checker for testing"),
            key: None,
            description: String::from("Mock decoder description"),
            link: String::from("https://mockdecoderwebsite.com"),
        };

        let expected_cache_row = CacheRow {
            id: 1,
            encoded_text: String::from("aGVsbG8gd29ybGQK"),
            decoded_text: String::from("hello world"),
            path: vec![mock_crack_result.clone()],
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 14:16:00"),
        };

        let row_result = add_row(
            &conn,
            &expected_cache_row.encoded_text,
            &expected_cache_row.decoded_text,
            &expected_cache_row.path,
            &expected_cache_row.successful,
            &expected_cache_row.execution_time_ms,
            &expected_cache_row.timestamp,
        );
        assert!(row_result.is_ok());

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();
            let crack_result_vec: Vec<RawCrackResult> =
                serde_json::from_str(&path_str.clone()).unwrap();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: crack_result_vec,
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                timestamp: row.get_unwrap(6),
            })
        });
        assert!(query_result.is_ok());
        let cache_row: CacheRow = query_result.unwrap().next().unwrap().unwrap();
        assert_eq!(cache_row, expected_cache_row);
    }
}
