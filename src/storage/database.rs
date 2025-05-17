// Module for managing the SQLite database
//
// This database is intended for caching known encoded/decoded string
// relations and collecting statistics on the performance of Ciphey
// search algorithms.

use super::super::CheckResult;
use super::super::CrackResult;
use chrono::DateTime;
use std::sync::OnceLock;
use uuid::Uuid;

/// Holds the global path to the database
pub static DB_PATH: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();

#[derive(Debug)]
/// Struct representing a row in the human_rejection table
pub struct HumanRejectionRow {
    /// Uuid for the human_rejection entry
    pub uuid: Uuid,
    /// Plaintext that has been marked as a failed decode
    pub plaintext: String,
    /// Name of the checker that was used to confirm the plaintext
    pub checker: String,
    /// When the decoding was run
    pub timestamp: String,
}

impl PartialEq for HumanRejectionRow {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
            && self.plaintext == other.plaintext
            && self.checker == other.checker
            && self.timestamp == other.timestamp
    }
}

#[derive(Debug)]
/// Struct representing a row in the cache table
pub struct CacheRow {
    /// UUID of the cache entry
    pub uuid: Uuid,
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
        self.uuid == other.uuid
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
    /// Uuid for the cache entry
    pub uuid: Uuid,
    /// Text before it is decoded
    pub encoded_text: String,
    /// Text after it is decoded
    pub decoded_text: String,
    /// Ordered list of decoding attempts
    pub path: Vec<CrackResult>,
    /// How long the decoding took in milliseconds
    pub execution_time_ms: i64,
}

/// Helper function get a DateTime formatted timestamp
fn get_timestamp() -> String {
    let timestamp: DateTime<chrono::Local> = std::time::SystemTime::now().into();
    timestamp.format("%Y-%m-%d %T").to_string()
}

/// Returns the path to the database file
fn get_database_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".ciphey");
    path.push("database.sqlite");
    path
}

/// Opens and returns a Connection to the SQLite database
///
/// If a path is specified in DB_PATH, returns a Connection to that path
/// Otherwise, opens a Connection to an in-memory database
fn get_db_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
    match DB_PATH.get() {
        Some(Some(path)) => rusqlite::Connection::open(path),
        _ => rusqlite::Connection::open_in_memory(),
    }
}

/// Public wrapper for setting up database
///
/// # Errors
///
/// On error setting up the database, returns a rusqlite::Error
/// If there's an error while setting the database path, prints warning
/// to console and continues with the default DB_PATH
pub fn setup_database() -> Result<(), rusqlite::Error> {
    match DB_PATH.get() {
        Some(_path) => (),
        None => {
            let db_result: Result<(), Option<std::path::PathBuf>> =
                DB_PATH.set(Some(get_database_path()));
            match db_result {
                Ok(_) => (),
                Err(_e) => {
                    crate::cli_pretty_printing::warning("Error setting database path");
                }
            }
        }
    };
    init_database()?;
    Ok(())
}

/// Initializes database with default schema
fn init_database() -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = get_db_connection()?;
    // Initializing cache table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            uuid TEXT PRIMARY KEY NOT NULL,
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

    // Initializing human checker table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS human_rejection (
            uuid TEXT PRIMARY KEY NOT NULL,
            plaintext TEXT NOT NULL,
            checker TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_stats_plaintext ON human_rejection(plaintext);",
        (),
    )?;

    Ok(conn)
}

/// Adds a new cache record to the cache table
///
/// Returns the number of successfully inserted rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
///
/// # Panics
///
/// Panics if the decoding path could not be serialized
pub fn insert_cache(cache_entry: &CacheEntry) -> Result<usize, rusqlite::Error> {
    let path: Vec<String> = cache_entry
        .path
        .iter()
        .map(|crack_result| crack_result.get_json().unwrap_or_default())
        .collect();

    let last_crack_result = cache_entry.path.last();
    let successful: bool = match last_crack_result {
        Some(crack_result) => crack_result.success,
        None => false,
    };

    let path_json = serde_json::to_string(&path).unwrap();
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "INSERT INTO cache (
            uuid,
            encoded_text,
            decoded_text,
            path,
            successful,
            execution_time_ms,
            timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
        (
            cache_entry.uuid.to_string(),
            cache_entry.encoded_text.clone(),
            cache_entry.decoded_text.clone(),
            path_json,
            successful,
            cache_entry.execution_time_ms,
            get_timestamp(),
        ),
    );
    transaction.commit()?;
    conn_result
}

/// Searches the database for a cache table row that matches the given encoded
/// text
///
/// On cache hit, returns a CacheRow
/// On cache miss, returns None
///
/// # Errors
///
/// Returns a ``rusqlite::Error``
pub fn read_cache(encoded_text: &String) -> Result<Option<CacheRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM cache WHERE encoded_text IS $1")?;
    let mut query = stmt.query_map([encoded_text], |row| {
        let path_str = row.get_unwrap::<usize, String>(3).to_owned();
        let crack_json_vec: Vec<String> =
            serde_json::from_str(&path_str.clone()).unwrap_or_default();

        Ok(CacheRow {
            uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str()).unwrap_or_default(),
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

/// Removes the cache row corresponding to the given encoded_text
///
/// Returns number of successfully deleted rows on success
///
/// # Errors
///
/// Returns sqlite::Error on error
pub fn delete_cache(encoded_text: &str) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "DELETE FROM cache WHERE encoded_text = $1",
        (encoded_text.to_owned(),),
    );
    transaction.commit()?;
    conn_result
}

/// Updates the values in a cache row corresponding to the encoded_text in
/// the given cache entry
///
/// Returns number of rows updated on success
///
/// # Errors
///
/// Returns sqlite::Error on error
pub fn update_cache(cache_entry: &CacheEntry) -> Result<usize, rusqlite::Error> {
    let path: Vec<String> = cache_entry
        .path
        .iter()
        .map(|crack_result| crack_result.get_json().unwrap_or_default())
        .collect();

    let last_crack_result = cache_entry.path.last();
    let successful = match last_crack_result {
        Some(crack_result) => crack_result.success,
        None => false,
    };

    let path_json = serde_json::to_string(&path).unwrap_or_default();
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "UPDATE cache SET 
            uuid = $1,
            decoded_text = $2,
            path = $3,
            successful = $4,
            execution_time_ms = $5,
            timestamp = $6
            WHERE encoded_text = $7;",
        (
            cache_entry.uuid.to_string(),
            cache_entry.decoded_text.clone(),
            path_json,
            successful,
            cache_entry.execution_time_ms,
            get_timestamp(),
            cache_entry.encoded_text.clone(),
        ),
    );
    transaction.commit()?;
    conn_result
}

/// Adds a new decode failure record to the human_rejection table
///
/// Returns the number of successfully inserted rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn insert_human_rejection(
    uuid: Uuid,
    plaintext: &str,
    check_result: &CheckResult,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "INSERT INTO human_rejection (
            uuid,
            plaintext,
            checker,
            timestamp)
        VALUES ($1, $2, $3, $4)",
        (
            uuid.to_string(),
            plaintext.to_owned(),
            check_result.checker_name,
            get_timestamp(),
        ),
    );
    transaction.commit()?;
    conn_result
}

/// Searches the database for a human_rejection table row that matches the given plaintext
///
/// On match, returns a HumanRejectionRow
/// Otherwise, returns None
///
/// # Errors
///
/// Returns a ``rusqlite::Error``
pub fn read_human_rejection(
    plaintext: &String,
) -> Result<Option<HumanRejectionRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM human_rejection WHERE plaintext IS $1")?;
    let mut query = stmt.query_map([plaintext], |row| {
        Ok(HumanRejectionRow {
            uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str()).unwrap_or_default(),
            plaintext: row.get_unwrap(1),
            checker: row.get_unwrap(2),
            timestamp: row.get_unwrap(3),
        })
    })?;
    let row = query.next();
    match row {
        Some(cache_row) => Ok(Some(cache_row?)),
        None => Ok(None),
    }
}

/// Updates a human_rejection row for a given plaintext
///
/// Returns the number of update rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn update_human_rejection(
    uuid: Uuid,
    plaintext: &str,
    check_result: &CheckResult,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "UPDATE human_rejection SET 
            uuid = $1,
            checker = $2,
            timestamp = $3
            WHERE plaintext = $4;",
        (
            uuid.to_string(),
            check_result.checker_name,
            get_timestamp(),
            plaintext.to_owned(),
        ),
    );
    transaction.commit()?;
    conn_result
}

/// Removes the human_rejection row corresponding to the given plaintext
///
/// Returns number of successfully deleted rows on success
///
/// # Errors
///
/// Returns sqlite::Error on error
pub fn delete_human_rejection(plaintext: &str) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "DELETE FROM human_rejection WHERE plaintext = $1",
        (plaintext.to_owned(),),
    );
    transaction.commit()?;
    conn_result
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::super::super::decoders::interface::{Crack, Decoder};
    use super::CrackResult;
    use super::*;
    use crate::checkers::{
        athena::Athena,
        checker_result::CheckResult,
        checker_type::{Check, Checker},
        english::EnglishChecker,
        CheckerTypes,
    };
    use uuid::Uuid;

    struct MockDecoder;
    impl Crack for Decoder<MockDecoder> {
        fn new() -> Decoder<MockDecoder> {
            Decoder {
                name: "MockEncoding",
                description: "A mocked decoder for testing",
                link: "https://en.wikipedia.org/wiki/Mock_object",
                tags: vec!["mock", "decoder", "base"],
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
        /// Gets the description for the current decoder
        fn get_description(&self) -> &str {
            self.description
        }
        /// Gets the link for the current decoder
        fn get_link(&self) -> &str {
            self.link
        }
    }

    fn set_test_db_path() {
        let path = std::path::PathBuf::from(String::from("file::memory:?cache=shared"));
        let _ = DB_PATH.set(Some(path));
    }

    /// Helper function for generating a cache row
    fn generate_cache_row(
        uuid: Uuid,
        encoded_text: &str,
        decoded_text: &str,
    ) -> (CrackResult, CacheRow, CacheEntry) {
        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result = CrackResult::new(&mock_decoder, encoded_text.to_owned());
        mock_crack_result.success = true;
        mock_crack_result.unencrypted_text = Some(vec![decoded_text.to_owned()]);

        let expected_cache_row = CacheRow {
            uuid,
            encoded_text: encoded_text.to_owned(),
            decoded_text: decoded_text.to_owned(),
            path: match serde_json::to_string(&mock_crack_result) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            timestamp: String::new(),
        };

        let cache_entry = CacheEntry {
            uuid,
            encoded_text: encoded_text.to_owned(),
            decoded_text: decoded_text.to_owned(),
            path: vec![mock_crack_result.clone()],
            execution_time_ms: 100,
        };
        (mock_crack_result, expected_cache_row, cache_entry)
    }

    /// Helper function for generating a new human_rejection row
    fn generate_human_rejection_row<Type>(
        uuid: Uuid,
        encoded_text: &str,
        checker_used: Checker<Type>,
    ) -> (CheckResult, HumanRejectionRow) {
        let check_result = CheckResult {
            is_identified: false,
            text: "".to_string(),
            checker_name: checker_used.name,
            checker_description: checker_used.description,
            description: "".to_string(),
            link: checker_used.link,
        };

        let expected_row = HumanRejectionRow {
            uuid,
            plaintext: encoded_text.to_owned(),
            checker: String::from(check_result.checker_name),
            timestamp: String::new(),
        };
        (check_result, expected_row)
    }

    #[test]
    fn database_initialized() {
        set_test_db_path();
        let db_result = init_database();
        assert!(db_result.is_ok());
    }

    #[test]
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
        assert_eq!(name_list[0], "uuid");
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
        assert_eq!(type_list[0], "TEXT");
        assert_eq!(type_list[1], "TEXT");
        assert_eq!(type_list[2], "TEXT");
        assert_eq!(type_list[3], "JSON");
        assert_eq!(type_list[4], "BOOLEAN");
        assert_eq!(type_list[5], "INTEGER");
        assert_eq!(type_list[6], "DATETIME");
    }

    #[test]
    fn correct_human_rejection_table_schema() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let stmt_result = conn.prepare("PRAGMA table_info(human_rejection);");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let name_result = stmt.query_map([], |row| row.get::<usize, String>(1));
        assert!(name_result.is_ok());
        let name_query = name_result.unwrap();
        let name_list: Vec<String> = name_query.map(|row| row.unwrap()).collect();
        assert_eq!(name_list[0], "uuid");
        assert_eq!(name_list[1], "plaintext");
        assert_eq!(name_list[2], "checker");
        assert_eq!(name_list[3], "timestamp");

        let type_result = stmt.query_map([], |row| row.get::<usize, String>(2));
        assert!(type_result.is_ok());
        let type_query = type_result.unwrap();
        let type_list: Vec<String> = type_query.map(|row| row.unwrap()).collect();
        assert_eq!(type_list[0], "TEXT");
        assert_eq!(type_list[1], "TEXT");
        assert_eq!(type_list[2], "TEXT");
        assert_eq!(type_list[3], "DATETIME");
    }

    #[test]
    fn cache_insert_empty_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str())
                    .unwrap_or_default(),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: serde_json::from_str(&path_str).unwrap_or_default(),
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
    fn cache_insert_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");
        let uuid = Uuid::new_v4();

        let (_mock_crack_result, mut expected_cache_row, cache_entry) =
            generate_cache_row(uuid, &encoded_text, &decoded_text);
        let row_result = insert_cache(&cache_entry);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str())
                    .unwrap_or_default(),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: serde_json::from_str(&path_str).unwrap_or_default(),
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
    fn cache_insert_2_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_1);
        let row_result = insert_cache(&cache_entry_1);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");
        let uuid_2 = Uuid::new_v4();

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(uuid_2, &encoded_text_2, &decoded_text_2);
        let row_result = insert_cache(&cache_entry_2);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str())
                    .unwrap_or_default(),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: serde_json::from_str(&path_str).unwrap_or_default(),
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
    fn cache_read_hit() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result, mut expected_cache_row, cache_entry) =
            generate_cache_row(uuid_1, &encoded_text, &decoded_text);
        let _row_result = insert_cache(&cache_entry);

        let cache_result = read_cache(&encoded_text);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row);
    }

    #[test]
    fn cache_read_2_hit() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_1);
        let _row_result = insert_cache(&cache_entry_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");
        let uuid_2 = Uuid::new_v4();

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(uuid_2, &encoded_text_2, &decoded_text_2);
        let _row_result = insert_cache(&cache_entry_2);

        let cache_result = read_cache(&encoded_text_1);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row_1.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row_1);

        let cache_result = read_cache(&encoded_text_2);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row_2.timestamp = cache_row.timestamp.clone();
        assert_eq!(cache_row, expected_cache_row_2);
    }

    #[test]
    fn cache_read_empty_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");

        let cache_result = read_cache(&encoded_text);
        assert!(cache_result.is_ok());
        let cache_row: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row.is_none());
    }

    #[test]
    fn cache_read_2_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result_1, _expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let _decoded_text_2 = String::from("world hello");
        let uuid_2 = Uuid::new_v4();

        let (_mock_crack_result_2, _expected_cache_row_2, cache_entry_2) =
            generate_cache_row(uuid_2, &encoded_text_1, &decoded_text_1);

        let _row_result = insert_cache(&cache_entry_1);
        let _row_result = insert_cache(&cache_entry_2);

        let cache_result = read_cache(&encoded_text_2);
        assert!(cache_result.is_ok());
        let cache_row: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row.is_none());
    }

    #[test]
    fn cache_delete_success_one_entry() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");
        let uuid = Uuid::new_v4();

        let (_mock_crack_result, _expected_cache_row, cache_entry) =
            generate_cache_row(uuid, &encoded_text, &decoded_text);
        let _row_result = insert_cache(&cache_entry);
        let _read_result = read_cache(&encoded_text);
        let delete_result = delete_cache(&encoded_text);
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 1);
        let read_result = read_cache(&encoded_text);
        assert!(read_result.is_ok());
        assert!(read_result.unwrap().is_none());
    }

    #[test]
    fn cache_delete_success_with_two_entries() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");
        let uuid_2 = Uuid::new_v4();

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(uuid_2, &encoded_text_2, &decoded_text_2);

        let _row_result = insert_cache(&cache_entry_1);
        let _row_result = insert_cache(&cache_entry_2);

        let read_result = read_cache(&encoded_text_1).unwrap();
        assert!(read_result.is_some());
        let row: CacheRow = read_result.unwrap();
        expected_cache_row_1.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_cache_row_1);

        let read_result = read_cache(&encoded_text_2).unwrap();
        assert!(read_result.is_some());
        let row: CacheRow = read_result.unwrap();
        expected_cache_row_2.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_cache_row_2);

        let delete_result = delete_cache(&encoded_text_1);
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 1);
        let read_result = read_cache(&encoded_text_1);
        assert!(read_result.is_ok());
        assert!(read_result.unwrap().is_none());

        let read_result = read_cache(&encoded_text_2).unwrap();
        assert!(read_result.is_some());
        let row: CacheRow = read_result.unwrap();
        assert_eq!(row, expected_cache_row_2);
    }

    #[test]
    fn cache_delete_missing() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let delete_result = delete_cache(&encoded_text);
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 0);
    }

    #[test]
    fn cache_delete_missing_with_entries() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result_1, _expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_1);
        let row_result = insert_cache(&cache_entry_1);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");

        let delete_result = delete_cache(&encoded_text_2);

        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 0);
    }

    #[test]
    fn cache_update_1_change_1_entry_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");
        let decoded_text_err = String::from("hello world oops");
        let uuid = Uuid::new_v4();
        let uuid_new = Uuid::new_v4();

        let (_mock_crack_result, mut expected_cache_row, cache_entry) =
            generate_cache_row(uuid, &encoded_text, &decoded_text_err);
        let _row_result = insert_cache(&cache_entry);

        let (_mock_crack_result_new, mut expected_cache_row_new, cache_entry_new) =
            generate_cache_row(uuid_new, &encoded_text, &decoded_text);
        let update_result = update_cache(&cache_entry_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_cache(&encoded_text);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_new.timestamp = row.timestamp.clone();
        expected_cache_row.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_cache_row_new);
        assert_ne!(row, expected_cache_row);
    }

    #[test]
    fn cache_update_1_change_2_entry_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let decoded_text_err = String::from("hello world oops");
        let uuid_1 = Uuid::new_v4();
        let uuid_new = Uuid::new_v4();

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_err);
        let _row_result = insert_cache(&cache_entry_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");
        let uuid_2 = Uuid::new_v4();

        let (_mock_crack_result_2, _expected_cache_row_2, cache_entry_2) =
            generate_cache_row(uuid_2, &encoded_text_2, &decoded_text_2);
        let _row_result = insert_cache(&cache_entry_2);

        let (_mock_crack_result_new, mut expected_cache_row_new, cache_entry_new) =
            generate_cache_row(uuid_new, &encoded_text_1, &decoded_text_1);
        let update_result = update_cache(&cache_entry_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_cache(&encoded_text_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_new.timestamp = row.timestamp.clone();
        expected_cache_row_1.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_cache_row_new);
        assert_ne!(row, expected_cache_row_1);
    }

    #[test]
    fn cache_update_1_change_2_entry_no_match() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");
        let uuid_1 = Uuid::new_v4();

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(uuid_1, &encoded_text_1, &decoded_text_1);
        let _row_result = insert_cache(&cache_entry_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");
        let uuid_2 = Uuid::new_v4();

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(uuid_2, &encoded_text_2, &decoded_text_2);
        let _row_result = insert_cache(&cache_entry_2);

        let encoded_text_new = String::from("c29tZSBuZXcgdGV4dAo=");
        let decoded_text_new = String::from("some new text");
        let uuid_new = Uuid::new_v4();

        let (_mock_crack_result_new, mut expected_cache_row_new, cache_entry_new) =
            generate_cache_row(uuid_new, &encoded_text_new, &decoded_text_new);

        let update_result = update_cache(&cache_entry_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);

        let row_result = read_cache(&encoded_text_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_new.timestamp = row.timestamp.clone();
        expected_cache_row_1.timestamp = row.timestamp.clone();
        assert_ne!(row, expected_cache_row_new);
        assert_eq!(row, expected_cache_row_1);

        let row_result = read_cache(&encoded_text_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_2.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_cache_row_2);
    }

    #[test]
    fn cache_update_empty() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");
        let uuid = Uuid::new_v4();

        let (_mock_crack_result, mut _expected_cache_row, cache_entry) =
            generate_cache_row(uuid, &encoded_text, &decoded_text);

        let update_result = update_cache(&cache_entry);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);
    }

    #[test]
    fn human_rejection_insert_empty_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let stmt_result = conn.prepare("SELECT * FROM human_rejection;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            Ok(HumanRejectionRow {
                uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str())
                    .unwrap_or_default(),
                plaintext: row.get_unwrap(1),
                checker: row.get_unwrap(2),
                timestamp: row.get_unwrap(3),
            })
        });
        assert!(query_result.is_ok());
        let empty_rows = query_result.unwrap();
        assert_eq!(empty_rows.count(), 0);
    }

    #[test]
    fn human_rejection_insert_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let uuid = Uuid::new_v4();

        let checker_used = Checker::<Athena>::new();

        let (check_result, mut expected_row) =
            generate_human_rejection_row(uuid, &plaintext, checker_used);

        let result = insert_human_rejection(uuid, &plaintext, &check_result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM human_rejection;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            Ok(HumanRejectionRow {
                uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str())
                    .unwrap_or_default(),
                plaintext: row.get_unwrap(1),
                checker: row.get_unwrap(2),
                timestamp: row.get_unwrap(3),
            })
        });
        assert!(query_result.is_ok());
        let mut query = query_result.unwrap();
        let row: HumanRejectionRow = query.next().unwrap().unwrap();
        expected_row.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row);
    }

    #[test]
    fn human_rejection_insert_2_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext1");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();

        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);

        let result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let uuid_2 = Uuid::new_v4();

        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(uuid_2, &plaintext_2, checker_used_2);

        let result = insert_human_rejection(uuid_2, &plaintext_2, &check_result_2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM human_rejection;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            Ok(HumanRejectionRow {
                uuid: Uuid::parse_str(row.get_unwrap::<usize, String>(0).as_str())
                    .unwrap_or_default(),
                plaintext: row.get_unwrap(1),
                checker: row.get_unwrap(2),
                timestamp: row.get_unwrap(3),
            })
        });
        assert!(query_result.is_ok());
        let mut query = query_result.unwrap();
        let row: HumanRejectionRow = query.next().unwrap().unwrap();
        expected_row_1.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_1);
        let row: HumanRejectionRow = query.next().unwrap().unwrap();
        expected_row_2.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_2);
    }

    #[test]
    fn failed_decode_read_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_used = Checker::<Athena>::new();
        let uuid = Uuid::new_v4();

        let (check_result, mut expected_row) =
            generate_human_rejection_row(uuid, &plaintext, checker_used);

        let _result = insert_human_rejection(uuid, &plaintext, &check_result);

        let row_result = read_human_rejection(&plaintext);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row);
    }

    #[test]
    fn failed_decode_read_2_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();

        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);

        let _result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let uuid_2 = Uuid::new_v4();

        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(uuid_2, &plaintext_2, checker_used_2);

        let _result = insert_human_rejection(uuid_2, &plaintext_2, &check_result_2);

        let row_result = read_human_rejection(&plaintext_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_1.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_1);

        let row_result = read_human_rejection(&plaintext_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_2.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_2);
    }

    #[test]
    fn human_rejection_read_empty_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_used = Checker::<Athena>::new();
        let uuid = Uuid::new_v4();

        let (check_result, _expected_row) =
            generate_human_rejection_row(uuid, &plaintext, checker_used);

        let _result = insert_human_rejection(uuid, &plaintext, &check_result);
        let row_result = read_human_rejection(&String::from("not plaintext"));
        assert!(row_result.is_ok());
        assert!(row_result.unwrap().is_none());
    }

    #[test]
    fn human_rejection_read_2_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();

        let (check_result_1, _expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);
        let _result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let uuid_2 = Uuid::new_v4();

        let (check_result_2, _expected_row_2) =
            generate_human_rejection_row(uuid_2, &plaintext_2, checker_used_2);
        let _result = insert_human_rejection(uuid_2, &plaintext_2, &check_result_2);

        let row_result = read_human_rejection(&String::from("not plaintext"));
        assert!(row_result.is_ok());
        assert!(row_result.unwrap().is_none());
    }

    #[test]
    fn human_rejection_delete_success_one_entry() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_used = Checker::<Athena>::new();
        let uuid = Uuid::new_v4();

        let (check_result, _expected_row) =
            generate_human_rejection_row(uuid, &plaintext, checker_used);
        let _result = insert_human_rejection(uuid, &plaintext, &check_result);
        let _row_result = read_human_rejection(&String::from("not plaintext"));
        let delete_result = delete_human_rejection(&plaintext);
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 1);
        let read_result = read_human_rejection(&plaintext);
        assert!(read_result.is_ok());
        assert!(read_result.unwrap().is_none());
    }

    #[test]
    fn human_rejection_delete_success_two_entries() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();
        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);
        let _result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let uuid_2 = Uuid::new_v4();
        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(uuid_2, &plaintext_2, checker_used_2);
        let _result = insert_human_rejection(uuid_2, &plaintext_2, &check_result_2);

        let read_result = read_human_rejection(&plaintext_1).unwrap();
        assert!(read_result.is_some());
        let row: HumanRejectionRow = read_result.unwrap();
        expected_row_1.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_1);

        let read_result = read_human_rejection(&plaintext_2).unwrap();
        assert!(read_result.is_some());
        let row: HumanRejectionRow = read_result.unwrap();
        expected_row_2.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_2);

        let delete_result = delete_human_rejection(&plaintext_1);
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 1);
        let read_result = read_human_rejection(&plaintext_1);
        assert!(read_result.is_ok());
        assert!(read_result.unwrap().is_none());

        let read_result = read_human_rejection(&plaintext_2).unwrap();
        assert!(read_result.is_some());
        let row: HumanRejectionRow = read_result.unwrap();
        assert_eq!(row, expected_row_2);
    }

    #[test]
    fn human_rejection_delete_missing() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let delete_result = delete_human_rejection(&plaintext);
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 0);
    }

    #[test]
    fn human_rejection_delete_missing_with_entries() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();
        let (check_result_1, _expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);
        let row_result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let plaintext_2 = String::from("plaintext2");

        let delete_result = delete_human_rejection(&plaintext_2);

        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 0);
    }

    #[test]
    fn human_rejection_update_1_change_1_entry_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_used = Checker::<Athena>::new();
        let uuid = Uuid::new_v4();
        let (check_result, mut expected_row) =
            generate_human_rejection_row(uuid, &plaintext, checker_used);
        let _row_result = insert_human_rejection(uuid, &plaintext, &check_result);

        let checker_new = Checker::<EnglishChecker>::new();
        let uuid_new = Uuid::new_v4();
        let (check_result_new, mut expected_row_new) =
            generate_human_rejection_row(uuid_new, &plaintext, checker_new);
        let update_result = update_human_rejection(uuid_new, &plaintext, &check_result_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_human_rejection(&plaintext);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row.timestamp = row.timestamp.clone();
        expected_row_new.timestamp = row.timestamp.clone();
        assert_ne!(row, expected_row);
        assert_eq!(row, expected_row_new);
    }

    #[test]
    fn human_rejection_update_1_change_2_entry_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext1");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();
        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);
        let _row_result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let uuid_2 = Uuid::new_v4();
        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(uuid_2, &plaintext_2, checker_used_2);
        let _row_result = insert_human_rejection(uuid_2, &plaintext_2, &check_result_2);

        let checker_new = Checker::<EnglishChecker>::new();
        let uuid_new = Uuid::new_v4();
        let (check_result_new, mut expected_row_new) =
            generate_human_rejection_row(uuid_new, &plaintext_1, checker_new);

        let update_result = update_human_rejection(uuid_new, &plaintext_1, &check_result_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_human_rejection(&plaintext_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_1.timestamp = row.timestamp.clone();
        expected_row_new.timestamp = row.timestamp.clone();
        assert_ne!(row, expected_row_1);
        assert_eq!(row, expected_row_new);

        let row_result = read_human_rejection(&plaintext_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_2.timestamp = row.timestamp.clone();
        expected_row_new.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_2);
        assert_ne!(row, expected_row_new);
    }

    #[test]
    fn human_rejection_update_1_change_2_entry_no_match() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext1");
        let checker_used_1 = Checker::<Athena>::new();
        let uuid_1 = Uuid::new_v4();
        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(uuid_1, &plaintext_1, checker_used_1);
        let _row_result = insert_human_rejection(uuid_1, &plaintext_1, &check_result_1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let uuid_2 = Uuid::new_v4();
        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(uuid_2, &plaintext_2, checker_used_2);
        let _row_result = insert_human_rejection(uuid_2, &plaintext_2, &check_result_2);

        let plaintext_new = String::from("new plaintext");

        let checker_new = Checker::<EnglishChecker>::new();
        let uuid_new = Uuid::new_v4();
        let (check_result_new, mut expected_row_new) =
            generate_human_rejection_row(uuid_new, &plaintext_new, checker_new);

        let update_result = update_human_rejection(uuid_new, &plaintext_new, &check_result_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);

        let row_result = read_human_rejection(&plaintext_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_1.timestamp = row.timestamp.clone();
        expected_row_new.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_1);
        assert_ne!(row, expected_row_new);

        let row_result = read_human_rejection(&plaintext_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_2.timestamp = row.timestamp.clone();
        expected_row_new.timestamp = row.timestamp.clone();
        assert_eq!(row, expected_row_2);
        assert_ne!(row, expected_row_new);
    }

    #[test]
    fn human_rejection_update_empty() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_new = Checker::<EnglishChecker>::new();
        let uuid = Uuid::new_v4();
        let (check_result_new, _expected_row_new) =
            generate_human_rejection_row(uuid, &plaintext, checker_new);
        let update_result = update_human_rejection(uuid, &plaintext, &check_result_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);
    }
}
