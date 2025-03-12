use super::super::CrackResult;
///! Module for managing the SQLite database
///!
///! This database is intended for caching known encoded/decoded string
///! relations and collecting statistics on the performance of Ares
///! search algorithms.
use chrono::DateTime;
use std::sync::OnceLock;

static DB_PATH: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();

#[derive(Debug)]
/// Struct representing a row in the cache table
pub struct CacheRow {
    /// Index of row in cache table
    pub id: usize,
    /// Text before it is decoded
    pub encoded_text: String,
    /// Text after it is decoded
    pub decoded_text: String,
    /// Ordered list of decoding attempts
    pub path: Vec<String>,
    /// Whether or not the decoding was successful
    pub successful: bool,
    /// How long the decoding took in milliseconds
    pub execution_time_ms: i64,
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

#[derive(Debug)]
/// Represents an entry into the cache table
pub struct CacheEntry {
    /// Text before it is decoded
    pub encoded_text: String,
    /// Text after it is decoded
    pub decoded_text: String,
    /// Ordered list of decoding attempts
    pub path: Vec<CrackResult>,
    /// How long the decoding took in milliseconds
    pub execution_time_ms: i64,
}

/// Returns the path to the database file
fn get_database_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push("Ares");
    path.push("database.sqlite");
    path
}

/// Opens and returns a Connection to the SQLite database
///
/// If a path is specified in DB_PATH, returns a Connection to that path
/// Otherwise, opens a Connection to an in-memory database
fn get_db_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
    match DB_PATH.get() {
        Some(db_path) => match db_path {
            Some(path) => rusqlite::Connection::open(path),
            None => rusqlite::Connection::open_in_memory(),
        },
        None => rusqlite::Connection::open_in_memory(),
    }
}

/// Public wrapper for setting up database
pub fn setup_database() -> Result<(), rusqlite::Error> {
    let path = get_database_path();
    DB_PATH.set(Some(path)); // TODO: Handle errors from this Result
    init_database()?;
    Ok(())
}

/// Initializes database with default schema
fn init_database() -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = get_db_connection()?;
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

    Ok(conn)
}

/// Adds a new cache record to the cache table
pub fn insert_cache(cache_entry: &CacheEntry) -> Result<(), rusqlite::Error> {
    let path: Vec<String> = cache_entry
        .path
        .iter()
        .map(|crack_result| match crack_result.get_json() {
            Ok(json) => json,
            Err(_) => String::new(),
        })
        .collect();

    let last_crack_result = cache_entry.path.get(cache_entry.path.len() - 1);
    let successful;
    match last_crack_result {
        Some(crack_result) => {
            successful = crack_result.success;
        }
        None => {
            successful = false;
        }
    }

    let timestamp: DateTime<chrono::Local> = std::time::SystemTime::now().into();

    let path_json = serde_json::to_string(&path).unwrap();
    let conn = get_db_connection()?;
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
            cache_entry.encoded_text.clone(),
            cache_entry.decoded_text.clone(),
            path_json,
            successful.clone(),
            cache_entry.execution_time_ms.clone(),
            timestamp.format("%Y-%m-%d %T").to_string(),
        ),
    );
    Ok(())
}

/// Searches the database for a cache table row that matches the given encoded
/// text
///
/// On cache hit, returns a CacheRow
/// On cache miss, returns None
/// On error, returns a ``rusqlite::Error``
pub fn read_row(encoded_text: &String) -> Result<Option<CacheRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM cache WHERE encoded_text IS $1")?;
    let mut query = stmt.query_map([encoded_text], |row| {
        let path_str = row.get_unwrap::<usize, String>(3).to_owned();
        let crack_json_vec: Vec<String> = serde_json::from_str(&path_str.clone()).unwrap();

        Ok(CacheRow {
            id: row.get_unwrap(0),
            encoded_text: row.get_unwrap(1),
            decoded_text: row.get_unwrap(2),
            path: crack_json_vec,
            successful: row.get_unwrap(4),
            execution_time_ms: row.get_unwrap(5),
            timestamp: row.get_unwrap(6),
        })
    })?;
    let row = query.next();
    match row {
        Some(cache_row) => Ok(Some(cache_row?)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::checkers::CheckerTypes;
    use super::super::super::decoders::interface::{Crack, Decoder};
    use super::super::super::CrackResult;
    use super::*;
    use serial_test::serial;
    use uuid::Uuid;

    struct MockDecoder;
    impl Crack for Decoder<MockDecoder> {
        fn new() -> Decoder<MockDecoder> {
            Decoder {
                name: "MockEncoding",
                description: "A mocked decoder for testing",
                link: "https://en.wikipedia.org/wiki/Mock_object",
                tags: vec!["mock", "url", "decoder", "base"],
                popularity: 1.0,
                phantom: std::marker::PhantomData,
            }
        }

        /// Mocked cracking function
        fn crack(&self, text: &str, _checker: &CheckerTypes) -> CrackResult {
            let mut results = CrackResult::new(self, text.to_string());
            results.unencrypted_text = Some(vec![String::from("mock decoded text")]);
            results
        }

        /// Gets all tags for this decoder
        fn get_tags(&self) -> &Vec<&str> {
            &self.tags
        }
        /// Gets the name for the current decoder
        fn get_name(&self) -> &str {
            self.name
        }
    }

    fn set_test_db_path() {
        let test_id = Uuid::new_v4();
        let path = std::path::PathBuf::from(
            String::from("file::") + test_id.to_string().as_str() + "db?mode=memory&cache=shared",
        );
        let _ = DB_PATH.set(Some(path));
    }

    #[test]
    #[serial]
    fn database_initialized() {
        set_test_db_path();
        let db_result = init_database();
        assert!(db_result.is_ok());
    }

    #[test]
    #[serial]
    fn cache_table_created() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let stmt_result =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='cache';");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let query_result = stmt.query_map([], |row| row.get::<usize, String>(0));
        assert!(query_result.is_ok());
        assert_eq!(query_result.unwrap().count(), 1);
    }

    #[test]
    #[serial]
    fn correct_cache_table_schema() {
        set_test_db_path();
        let conn = init_database().unwrap();

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
    #[serial]
    fn cache_record_empty_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: match serde_json::from_str(&path_str) {
                    Ok(path) => path,
                    Err(_) => vec![],
                },
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
    #[serial]
    fn cache_record_entry_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");

        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result = CrackResult::new(&mock_decoder, encoded_text.clone());
        mock_crack_result.success = true;
        mock_crack_result.unencrypted_text = Some(vec![decoded_text.clone()]);

        let mut expected_cache_row = CacheRow {
            id: 1,
            encoded_text: encoded_text.clone(),
            decoded_text: decoded_text.clone(),
            path: match serde_json::to_string(&mock_crack_result) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 14:16:00"),
        };

        let cache_entry = CacheEntry {
            encoded_text: encoded_text.clone(),
            decoded_text: decoded_text.clone(),
            path: vec![mock_crack_result.clone()],
            execution_time_ms: 100,
        };

        let _row_result = insert_cache(&cache_entry);

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: match serde_json::from_str(&path_str) {
                    Ok(path) => path,
                    Err(_) => vec![],
                },
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                timestamp: row.get_unwrap(6),
            })
        });
        assert!(query_result.is_ok());
        let cache_row: CacheRow = query_result.unwrap().next().unwrap().unwrap();
        expected_cache_row.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row);
    }

    #[test]
    #[serial]
    fn cache_record_2_entries_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result_1 = CrackResult::new(&mock_decoder, encoded_text_1.clone());
        mock_crack_result_1.success = true;
        mock_crack_result_1.unencrypted_text = Some(vec![decoded_text_1.clone()]);

        let mut expected_cache_row_1 = CacheRow {
            id: 1,
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: match serde_json::to_string(&mock_crack_result_1) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 14:16:00"),
        };

        let mut mock_crack_result_2 = CrackResult::new(&mock_decoder, encoded_text_2.clone());
        mock_crack_result_2.success = true;
        mock_crack_result_2.unencrypted_text = Some(vec![decoded_text_2.clone()]);

        let mut expected_cache_row_2 = CacheRow {
            id: 2,
            encoded_text: encoded_text_2.clone(),
            decoded_text: decoded_text_2.clone(),
            path: match serde_json::to_string(&mock_crack_result_2) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },

            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 15:12:00"),
        };

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: vec![mock_crack_result_1.clone()],
            execution_time_ms: 100,
        });

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text_2.clone(),
            decoded_text: decoded_text_2.clone(),
            path: vec![mock_crack_result_2.clone()],
            execution_time_ms: 100,
        });

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: match serde_json::from_str(&path_str) {
                    Ok(path) => path,
                    Err(_) => vec![],
                },
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                timestamp: row.get_unwrap(6),
            })
        });
        let mut query = query_result.unwrap();
        let cache_row: CacheRow = query.next().unwrap().unwrap();
        expected_cache_row_1.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row_1);
        let cache_row: CacheRow = query.next().unwrap().unwrap();
        expected_cache_row_2.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row_2);
    }

    #[test]
    #[serial]
    fn cache_record_read_hit() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");

        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result = CrackResult::new(&mock_decoder, encoded_text.clone());
        mock_crack_result.success = true;
        mock_crack_result.unencrypted_text = Some(vec![decoded_text.clone()]);

        let mut expected_cache_row = CacheRow {
            id: 1,
            encoded_text: encoded_text.clone(),
            decoded_text: decoded_text.clone(),
            path: match serde_json::to_string(&mock_crack_result) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 14:16:00"),
        };

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text.clone(),
            decoded_text: decoded_text.clone(),
            path: vec![mock_crack_result.clone()],
            execution_time_ms: 100,
        });

        let cache_result = read_row(&encoded_text);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row);
    }

    #[test]
    #[serial]
    fn cache_multiple_record_read_hit() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result_1 = CrackResult::new(&mock_decoder, encoded_text_1.clone());
        mock_crack_result_1.success = true;
        mock_crack_result_1.unencrypted_text = Some(vec![decoded_text_1.clone()]);

        let mut expected_cache_row_1 = CacheRow {
            id: 1,
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: match serde_json::to_string(&mock_crack_result_1) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 14:16:00"),
        };

        let mut mock_crack_result_2 = CrackResult::new(&mock_decoder, encoded_text_2.clone());
        mock_crack_result_2.success = true;
        mock_crack_result_2.unencrypted_text = Some(vec![decoded_text_2.clone()]);

        let mut expected_cache_row_2 = CacheRow {
            id: 2,
            encoded_text: encoded_text_2.clone(),
            decoded_text: decoded_text_2.clone(),
            path: match serde_json::to_string(&mock_crack_result_2) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 15:12:00"),
        };

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: vec![mock_crack_result_1.clone()],
            execution_time_ms: 100,
        });

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text_2.clone(),
            decoded_text: decoded_text_2.clone(),
            path: vec![mock_crack_result_2.clone()],
            execution_time_ms: 100,
        });

        let cache_result = read_row(&encoded_text_1);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row_1.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row_1);

        let cache_result = read_row(&encoded_text_2);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row_2.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row_2);
    }

    #[test]
    #[serial]
    fn cache_empty_read_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");

        let cache_result = read_row(&encoded_text);
        assert!(cache_result.is_ok());
        let cache_row: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row.is_none());
    }

    #[test]
    #[serial]
    fn cache_multiple_record_read_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let _decoded_text_2 = String::from("world hello");

        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result_1 = CrackResult::new(&mock_decoder, encoded_text_1.clone());
        mock_crack_result_1.success = true;
        mock_crack_result_1.unencrypted_text = Some(vec![decoded_text_1.clone()]);

        let _expected_cache_row_1 = CacheRow {
            id: 1,
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: match serde_json::to_string(&mock_crack_result_1) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 14:16:00"),
        };

        let mock_crack_result_2 = CrackResult::new(&mock_decoder, encoded_text_1.clone());
        mock_crack_result_1.success = true;
        mock_crack_result_1.unencrypted_text = Some(vec![decoded_text_1.clone()]);

        let _expected_cache_row_2 = CacheRow {
            id: 2,
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: match serde_json::to_string(&mock_crack_result_2) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::from("2025-05-29 15:12:00"),
        };

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: vec![mock_crack_result_1.clone()],
            execution_time_ms: 100,
        });

        let _row_result = insert_cache(&CacheEntry {
            encoded_text: encoded_text_1.clone(),
            decoded_text: decoded_text_1.clone(),
            path: vec![mock_crack_result_2.clone()],
            execution_time_ms: 100,
        });

        let cache_result = read_row(&encoded_text_2);
        assert!(cache_result.is_ok());
        let cache_row: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row.is_none());
    }
}
