// Module for managing the SQLite database
//
// This database is intended for caching known encoded/decoded string
// relations and collecting statistics on the performance of Ciphey
// search algorithms.

use super::super::CheckResult;
use super::super::CrackResult;
use chrono::DateTime;
use std::sync::RwLock;

/// Holds the global path to the database.
/// Using RwLock instead of OnceLock to allow resetting the path in tests.
pub static DB_PATH: RwLock<Option<std::path::PathBuf>> = RwLock::new(None);

/// Sets the database path. Returns true if successful, false if the path was already set.
/// In test mode, this will overwrite the existing path.
///
/// # Panics
///
/// Panics if the RwLock is poisoned.
pub fn set_db_path(path: Option<std::path::PathBuf>) -> bool {
    let mut db_path = DB_PATH.write().expect("DB_PATH RwLock poisoned");
    *db_path = path;
    true
}

/// Clears the database path, allowing it to be set again.
/// This is primarily used for testing purposes.
///
/// # Panics
///
/// Panics if the RwLock is poisoned.
#[doc(hidden)]
pub fn clear_db_path() {
    let mut db_path = DB_PATH.write().expect("DB_PATH RwLock poisoned");
    *db_path = None;
}

#[derive(Debug)]
/// Struct representing a row in the human_rejection table
pub struct HumanRejectionRow {
    /// Auto-incrementing ID for the human_rejection entry
    pub id: i64,
    /// Plaintext that has been marked as a failed decode
    pub plaintext: String,
    /// Original encoded text that led to this rejection (NULL if not available)
    pub encoded_text: Option<String>,
    /// Name of the checker that was used to confirm the plaintext
    pub checker: String,
    /// Description of the checker
    pub checker_description: Option<String>,
    /// Description of what the checker thought it found
    pub check_description: Option<String>,
    /// JSON-serialized decoder path that led to this false positive (NULL if not available)
    pub decoder_path: Option<String>,
    /// Number of times this plaintext+checker combination has been rejected
    pub rejection_count: i64,
    /// When this rejection was first recorded
    pub first_rejected: String,
    /// When this rejection was last recorded
    pub last_rejected: String,
}

impl PartialEq for HumanRejectionRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.plaintext == other.plaintext
            && self.encoded_text == other.encoded_text
            && self.checker == other.checker
            && self.checker_description == other.checker_description
            && self.check_description == other.check_description
            && self.decoder_path == other.decoder_path
            && self.rejection_count == other.rejection_count
            && self.first_rejected == other.first_rejected
            && self.last_rejected == other.last_rejected
    }
}

#[derive(Debug)]
/// Struct representing a row in the wordlist table
pub struct WordlistRow {
    /// Auto-incrementing ID for the wordlist entry
    pub id: i64,
    /// The word stored in the wordlist
    pub word: String,
    /// Source of the word (e.g., "user_import", "rockyou", "ctf_flags")
    pub source: String,
    /// Whether this word is enabled for matching (disabled words are ignored)
    pub enabled: bool,
    /// When the word was added to the database
    pub added_date: String,
    /// Foreign key to wordlist_files table (NULL for legacy imports)
    pub file_id: Option<i64>,
}

impl PartialEq for WordlistRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.word == other.word
            && self.source == other.source
            && self.enabled == other.enabled
            && self.added_date == other.added_date
            && self.file_id == other.file_id
    }
}

#[derive(Debug, Clone)]
/// Struct representing a row in the wordlist_files table
pub struct WordlistFileRow {
    /// Auto-incrementing ID for the wordlist file entry
    pub id: i64,
    /// Display filename (e.g., "rockyou.txt")
    pub filename: String,
    /// Full file path used for deduplication
    pub file_path: String,
    /// Source identifier (e.g., "user_import", "first_run")
    pub source: String,
    /// Number of words imported from this file
    pub word_count: i64,
    /// Whether this wordlist file is enabled
    pub enabled: bool,
    /// When the wordlist file was added
    pub added_date: String,
}

impl PartialEq for WordlistFileRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.filename == other.filename
            && self.file_path == other.file_path
            && self.source == other.source
            && self.word_count == other.word_count
            && self.enabled == other.enabled
            && self.added_date == other.added_date
    }
}

#[derive(Debug, Clone)]
/// Struct representing a row in the cache table
pub struct CacheRow {
    /// Unique row identifier
    pub id: i64,
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
    /// Length of the input text in bytes
    pub input_length: i64,
    /// Number of decoders in the path
    pub decoder_count: i64,
    /// Name of the checker that confirmed the plaintext
    pub checker_name: Option<String>,
    /// Key used for decryption (if applicable)
    pub key_used: Option<String>,
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
            && self.input_length == other.input_length
            && self.decoder_count == other.decoder_count
            && self.checker_name == other.checker_name
            && self.key_used == other.key_used
            && self.timestamp == other.timestamp
    }
}

#[derive(Debug, Clone)]
/// Struct representing a row in the ai_cache table
pub struct AiCacheRow {
    /// Unique row identifier
    pub id: i64,
    /// The cache key (concatenation of function_type + params)
    pub request_key: String,
    /// The type of AI function ("explain_step", "detect_language", "translate")
    pub function_type: String,
    /// The AI response text
    pub response: String,
    /// The model used to generate this response
    pub model: String,
    /// When the response was cached
    pub created_at: String,
}

impl PartialEq for AiCacheRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.request_key == other.request_key
            && self.function_type == other.function_type
            && self.response == other.response
            && self.model == other.model
            && self.created_at == other.created_at
    }
}

/// Type of branch (how it was created)
#[derive(Debug, Clone, PartialEq)]
pub enum BranchType {
    /// Created automatically during A* search
    Auto,
    /// Created by running all decoders once
    SingleLayer,
    /// Created by manually selecting a specific decoder
    Manual,
}

impl BranchType {
    /// Converts to database string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            BranchType::Auto => "auto",
            BranchType::SingleLayer => "single_layer",
            BranchType::Manual => "manual",
        }
    }

    /// Parses from database string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(BranchType::Auto),
            "single_layer" => Some(BranchType::SingleLayer),
            "manual" => Some(BranchType::Manual),
            _ => None,
        }
    }
}

/// Summary information about a branch for display in the UI
#[derive(Debug, Clone)]
pub struct BranchSummary {
    /// Database row ID for this branch
    pub cache_id: i64,
    /// How the branch was created
    pub branch_type: BranchType,
    /// Name of the first decoder in this branch's path
    pub first_decoder: String,
    /// Preview of the final text (truncated)
    pub final_text_preview: String,
    /// Whether this branch successfully found plaintext
    pub successful: bool,
    /// Number of decoders in the path
    pub path_length: usize,
    /// Number of sub-branches from this branch
    pub sub_branch_count: usize,
}

/// Information about a branch's parent relationship
#[derive(Debug, Clone)]
pub struct ParentBranchInfo {
    /// Parent cache entry ID
    pub parent_cache_id: i64,
    /// Step index in parent's path where branch occurred
    pub branch_step: usize,
}

#[derive(Debug)]
/// Represents an entry into the cache table
pub struct CacheEntry {
    /// Text before it is decoded (primary key)
    pub encoded_text: String,
    /// Text after it is decoded
    pub decoded_text: String,
    /// Ordered list of decoding attempts
    pub path: Vec<CrackResult>,
    /// How long the decoding took in milliseconds
    pub execution_time_ms: i64,
    /// Length of the input text in bytes
    pub input_length: i64,
    /// Number of decoders in the path
    pub decoder_count: i64,
    /// Name of the checker that confirmed the plaintext
    pub checker_name: Option<String>,
    /// Key used for decryption (if applicable)
    pub key_used: Option<String>,
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
///
/// # Panics
///
/// Panics if the RwLock is poisoned.
fn get_db_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
    let db_path = DB_PATH.read().expect("DB_PATH RwLock poisoned");
    match db_path.as_ref() {
        Some(path) => rusqlite::Connection::open(path),
        None => rusqlite::Connection::open_in_memory(),
    }
}

/// Public wrapper for getting a database connection.
///
/// This is primarily used by the TUI tree viewer to load branch data
/// with custom queries not covered by the standard functions.
///
/// # Errors
///
/// Returns rusqlite::Error on connection failure.
pub fn get_db_connection_pub() -> Result<rusqlite::Connection, rusqlite::Error> {
    get_db_connection()
}

/// Public wrapper for setting up database
///
/// # Errors
///
/// On error setting up the database, returns a rusqlite::Error
/// If there's an error while setting the database path, prints warning
/// to console and continues with the default DB_PATH
///
/// # Panics
///
/// Panics if the RwLock is poisoned.
pub fn setup_database() -> Result<(), rusqlite::Error> {
    {
        let db_path = DB_PATH.read().expect("DB_PATH RwLock poisoned");
        if db_path.is_none() {
            drop(db_path); // Release read lock before acquiring write lock
            let path = get_database_path();
            // Create parent directory if it doesn't exist
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }
            set_db_path(Some(path));
        }
    }
    init_database()?;
    Ok(())
}

/// Initializes database with default schema
///
/// This is pub(crate) to allow tests in sibling modules to initialize the database
pub(crate) fn init_database() -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = get_db_connection()?;
    // Initializing cache table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            encoded_text TEXT NOT NULL,
            decoded_text TEXT NOT NULL,
            path JSON NOT NULL,
            successful BOOLEAN NOT NULL DEFAULT false,
            execution_time_ms INTEGER NOT NULL,
            input_length INTEGER NOT NULL,
            decoder_count INTEGER NOT NULL,
            checker_name TEXT,
            key_used TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            parent_cache_id INTEGER REFERENCES cache(id) ON DELETE CASCADE,
            branch_step INTEGER,
            branch_type TEXT
    );",
        (),
    )?;

    // Run migration to add branch columns if they don't exist (for existing databases)
    run_branch_migration(&conn)?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cache_successful
            ON cache(successful);",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cache_timestamp
            ON cache(timestamp DESC);",
        (),
    )?;

    // Initializing human checker table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS human_rejection (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plaintext TEXT NOT NULL,
            encoded_text TEXT,
            checker TEXT NOT NULL,
            checker_description TEXT,
            check_description TEXT,
            decoder_path JSON,
            rejection_count INTEGER NOT NULL DEFAULT 1,
            first_rejected DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_rejected DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(plaintext, checker)
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_rejection_checker ON human_rejection(checker);",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_rejection_count ON human_rejection(rejection_count DESC);",
        (),
    )?;

    // Initializing wordlist_files table for tracking imported wordlist files
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wordlist_files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            file_path TEXT NOT NULL UNIQUE,
            source TEXT NOT NULL,
            word_count INTEGER NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT true,
            added_date DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_wordlist_files_enabled ON wordlist_files(enabled);",
        (),
    )?;

    // Initializing wordlist table for bloom filter-backed word lookups
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wordlist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL UNIQUE,
            source TEXT NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT true,
            added_date DATETIME DEFAULT CURRENT_TIMESTAMP,
            file_id INTEGER REFERENCES wordlist_files(id) ON DELETE CASCADE
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_wordlist_word ON wordlist(word);",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_wordlist_enabled ON wordlist(enabled);",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_wordlist_file_id ON wordlist(file_id);",
        (),
    )?;

    // Initializing AI response cache table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_key TEXT NOT NULL UNIQUE,
            function_type TEXT NOT NULL,
            response TEXT NOT NULL,
            model TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_ai_cache_function_type ON ai_cache(function_type);",
        (),
    )?;

    Ok(conn)
}

/// Runs migration to add branch columns to existing databases
///
/// This migration adds parent_cache_id, branch_step, and branch_type columns
/// to the cache table if they don't already exist. Existing entries will have
/// NULL values for these columns (indicating they are root entries).
fn run_branch_migration(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    // Check if the branch columns already exist
    let mut stmt = conn.prepare("PRAGMA table_info(cache)")?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<usize, String>(1))?
        .filter_map(|r| r.ok())
        .collect();

    // Add parent_cache_id column if it doesn't exist
    if !columns.contains(&"parent_cache_id".to_string()) {
        conn.execute(
            "ALTER TABLE cache ADD COLUMN parent_cache_id INTEGER REFERENCES cache(id) ON DELETE CASCADE",
            (),
        )?;
    }

    // Add branch_step column if it doesn't exist
    if !columns.contains(&"branch_step".to_string()) {
        conn.execute("ALTER TABLE cache ADD COLUMN branch_step INTEGER", ())?;
    }

    // Add branch_type column if it doesn't exist
    if !columns.contains(&"branch_type".to_string()) {
        conn.execute("ALTER TABLE cache ADD COLUMN branch_type TEXT", ())?;
    }

    // Add ai_explanations JSON column if it doesn't exist
    if !columns.contains(&"ai_explanations".to_string()) {
        conn.execute("ALTER TABLE cache ADD COLUMN ai_explanations JSON", ())?;
    }

    // Create index for efficient branch lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cache_parent_id ON cache(parent_cache_id);",
        (),
    )?;

    Ok(())
}

// ============================================================================
// Branch-Related Functions
// ============================================================================

/// Gets all branches from a specific step of a cache entry
///
/// Returns branches where `parent_cache_id = cache_id` and `branch_step = step`.
///
/// # Arguments
///
/// * `cache_id` - The parent cache entry ID
/// * `step` - The step index in the parent's path
///
/// # Returns
///
/// A vector of `BranchSummary` for branches at the specified step.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn get_branches_for_step(
    cache_id: i64,
    step: usize,
) -> Result<Vec<BranchSummary>, rusqlite::Error> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, branch_type, path, decoded_text, successful, decoder_count
         FROM cache 
         WHERE parent_cache_id = ?1 AND branch_step = ?2
         ORDER BY timestamp DESC",
    )?;

    let results = stmt.query_map([cache_id, step as i64], |row| {
        let cache_id: i64 = row.get(0)?;
        let branch_type_str: Option<String> = row.get(1)?;
        let path_json: String = row.get(2)?;
        let decoded_text: String = row.get(3)?;
        let successful: bool = row.get(4)?;
        let decoder_count: i64 = row.get(5)?;

        // Parse the path to get the first decoder
        let path_vec: Vec<String> = serde_json::from_str(&path_json).unwrap_or_default();
        let first_decoder = if let Some(first_json) = path_vec.first() {
            // Try to extract decoder name from JSON
            serde_json::from_str::<serde_json::Value>(first_json)
                .ok()
                .and_then(|v| {
                    v.get("decoder")
                        .and_then(|d| d.as_str().map(|s| s.to_string()))
                })
                .unwrap_or_else(|| "Unknown".to_string())
        } else {
            "Unknown".to_string()
        };

        // Truncate final text preview
        let final_text_preview = if decoded_text.len() > 30 {
            format!("{}...", decoded_text.chars().take(27).collect::<String>())
        } else {
            decoded_text
        };

        Ok(BranchSummary {
            cache_id,
            branch_type: branch_type_str
                .and_then(|s| BranchType::from_str(&s))
                .unwrap_or(BranchType::Auto),
            first_decoder,
            final_text_preview,
            successful,
            path_length: decoder_count as usize,
            sub_branch_count: 0, // Will be populated separately if needed
        })
    })?;

    results.collect()
}

/// Gets all branches from a cache entry (all steps)
///
/// Returns all branches where `parent_cache_id = cache_id`.
///
/// # Arguments
///
/// * `cache_id` - The parent cache entry ID
///
/// # Returns
///
/// A vector of `BranchSummary` for all branches from this cache entry.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn get_branches_for_cache(cache_id: i64) -> Result<Vec<BranchSummary>, rusqlite::Error> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, branch_type, path, decoded_text, successful, decoder_count, branch_step
         FROM cache 
         WHERE parent_cache_id = ?1
         ORDER BY branch_step ASC, timestamp DESC",
    )?;

    let results = stmt.query_map([cache_id], |row| {
        let cache_id: i64 = row.get(0)?;
        let branch_type_str: Option<String> = row.get(1)?;
        let path_json: String = row.get(2)?;
        let decoded_text: String = row.get(3)?;
        let successful: bool = row.get(4)?;
        let decoder_count: i64 = row.get(5)?;

        // Parse the path to get the first decoder
        let path_vec: Vec<String> = serde_json::from_str(&path_json).unwrap_or_default();
        let first_decoder = if let Some(first_json) = path_vec.first() {
            serde_json::from_str::<serde_json::Value>(first_json)
                .ok()
                .and_then(|v| {
                    v.get("decoder")
                        .and_then(|d| d.as_str().map(|s| s.to_string()))
                })
                .unwrap_or_else(|| "Unknown".to_string())
        } else {
            "Unknown".to_string()
        };

        let final_text_preview = if decoded_text.len() > 30 {
            format!("{}...", decoded_text.chars().take(27).collect::<String>())
        } else {
            decoded_text
        };

        Ok(BranchSummary {
            cache_id,
            branch_type: branch_type_str
                .and_then(|s| BranchType::from_str(&s))
                .unwrap_or(BranchType::Auto),
            first_decoder,
            final_text_preview,
            successful,
            path_length: decoder_count as usize,
            sub_branch_count: 0,
        })
    })?;

    results.collect()
}

/// Inserts a cache entry as a branch of another cache entry
///
/// # Arguments
///
/// * `cache_entry` - The cache entry to insert
/// * `parent_cache_id` - The parent cache entry ID
/// * `branch_step` - The step index in the parent's path where branching occurred
/// * `branch_type` - How the branch was created (auto, single_layer, manual)
///
/// # Returns
///
/// The row ID of the newly inserted branch entry.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn insert_branch(
    cache_entry: &CacheEntry,
    parent_cache_id: i64,
    branch_step: usize,
    branch_type: &BranchType,
) -> Result<i64, rusqlite::Error> {
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

    let path_json = serde_json::to_string(&path).unwrap_or_default();
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;

    transaction.execute(
        "INSERT INTO cache (
            encoded_text,
            decoded_text,
            path,
            successful,
            execution_time_ms,
            input_length,
            decoder_count,
            checker_name,
            key_used,
            timestamp,
            parent_cache_id,
            branch_step,
            branch_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        (
            cache_entry.encoded_text.clone(),
            cache_entry.decoded_text.clone(),
            path_json,
            successful,
            cache_entry.execution_time_ms,
            cache_entry.input_length,
            cache_entry.decoder_count,
            cache_entry.checker_name.clone(),
            cache_entry.key_used.clone(),
            get_timestamp(),
            parent_cache_id,
            branch_step as i64,
            branch_type.as_str(),
        ),
    )?;

    let row_id = transaction.last_insert_rowid();
    transaction.commit()?;
    Ok(row_id)
}

/// Links an existing cache row as a branch of a parent cache entry.
///
/// This updates an orphaned root-level cache row to become a branch by setting
/// its `parent_cache_id`, `branch_step`, and `branch_type` fields. This is used
/// when the branch result was inserted by `insert_cache` (which doesn't set
/// branch columns) and needs to be retroactively linked to a parent.
///
/// # Arguments
///
/// * `cache_id` - The cache entry ID to update
/// * `parent_cache_id` - The parent cache entry ID to link to
/// * `branch_step` - The step index in the parent's path where branching occurred
/// * `branch_type` - The type of branch (Auto, SingleLayer, Manual)
///
/// # Returns
///
/// Number of rows updated (should be 1 on success, 0 if cache_id not found).
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn link_as_branch(
    cache_id: i64,
    parent_cache_id: i64,
    branch_step: usize,
    branch_type: &BranchType,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let rows_updated = transaction.execute(
        "UPDATE cache SET parent_cache_id = ?1, branch_step = ?2, branch_type = ?3 WHERE id = ?4",
        (
            parent_cache_id,
            branch_step as i64,
            branch_type.as_str(),
            cache_id,
        ),
    )?;
    transaction.commit()?;
    Ok(rows_updated)
}

/// Gets parent info for a branch
///
/// # Arguments
///
/// * `cache_id` - The cache entry ID to check
///
/// # Returns
///
/// `Some(ParentBranchInfo)` if this is a branch, `None` if it's a root entry.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn get_parent_info(cache_id: i64) -> Result<Option<ParentBranchInfo>, rusqlite::Error> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare("SELECT parent_cache_id, branch_step FROM cache WHERE id = ?1")?;

    let result = stmt.query_row([cache_id], |row| {
        let parent_id: Option<i64> = row.get(0)?;
        let branch_step: Option<i64> = row.get(1)?;

        Ok((parent_id, branch_step))
    });

    match result {
        Ok((Some(parent_id), Some(step))) => Ok(Some(ParentBranchInfo {
            parent_cache_id: parent_id,
            branch_step: step as usize,
        })),
        Ok(_) => Ok(None),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Gets a cache entry by its ID
///
/// # Arguments
///
/// * `cache_id` - The cache entry ID to retrieve
///
/// # Returns
///
/// `Some(CacheRow)` if found, `None` otherwise.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn get_cache_by_id(cache_id: i64) -> Result<Option<CacheRow>, rusqlite::Error> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, encoded_text, decoded_text, path, successful, execution_time_ms,
                input_length, decoder_count, checker_name, key_used, timestamp
         FROM cache WHERE id = ?1",
    )?;

    let result = stmt.query_row([cache_id], |row| {
        let path_str: String = row.get(3)?;
        let crack_json_vec: Vec<String> = serde_json::from_str(&path_str).unwrap_or_default();

        Ok(CacheRow {
            id: row.get(0)?,
            encoded_text: row.get(1)?,
            decoded_text: row.get(2)?,
            path: crack_json_vec,
            successful: row.get(4)?,
            execution_time_ms: row.get(5)?,
            input_length: row.get(6)?,
            decoder_count: row.get(7)?,
            checker_name: row.get(8).ok(),
            key_used: row.get(9).ok(),
            timestamp: row.get(10)?,
        })
    });

    match result {
        Ok(row) => Ok(Some(row)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Counts the number of sub-branches for a given cache entry
///
/// # Arguments
///
/// * `cache_id` - The cache entry ID
///
/// # Returns
///
/// The count of branches that have this entry as their parent.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn count_sub_branches(cache_id: i64) -> Result<i64, rusqlite::Error> {
    let conn = get_db_connection()?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM cache WHERE parent_cache_id = ?1",
        [cache_id],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Adds a new cache record to the cache table
///
/// Returns the row ID of the inserted cache entry on success.
///
/// # Errors
///
/// Returns rusqlite::Error on error
///
/// # Panics
///
/// Panics if the decoding path could not be serialized
pub fn insert_cache(cache_entry: &CacheEntry) -> Result<i64, rusqlite::Error> {
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
    transaction.execute(
        "INSERT INTO cache (
            encoded_text,
            decoded_text,
            path,
            successful,
            execution_time_ms,
            input_length,
            decoder_count,
            checker_name,
            key_used,
            timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        (
            cache_entry.encoded_text.clone(),
            cache_entry.decoded_text.clone(),
            path_json,
            successful,
            cache_entry.execution_time_ms,
            cache_entry.input_length,
            cache_entry.decoder_count,
            cache_entry.checker_name.clone(),
            cache_entry.key_used.clone(),
            get_timestamp(),
        ),
    )?;
    let row_id = transaction.last_insert_rowid();
    transaction.commit()?;
    Ok(row_id)
}

/// Searches the database for a cache table row that matches the given encoded
/// text
///
/// On cache hit, returns a CacheRow
/// On cache miss, returns None
/// Handles both old schema (without id) and new schema (with id) gracefully.
///
/// # Errors
///
/// Returns a ``rusqlite::Error``
pub fn read_cache(encoded_text: &String) -> Result<Option<CacheRow>, rusqlite::Error> {
    let conn = get_db_connection()?;

    // Check if the new schema (with id column) exists
    let has_id_column = {
        let mut stmt = conn.prepare("PRAGMA table_info(cache)")?;
        let columns: Vec<String> = stmt
            .query_map([], |row| row.get::<usize, String>(1))?
            .filter_map(|r| r.ok())
            .collect();
        columns.first().map(|s| s == "id").unwrap_or(false)
    };

    if has_id_column {
        // New schema with id column
        let mut stmt = conn.prepare(
            "SELECT * FROM cache WHERE encoded_text IS $1 ORDER BY timestamp DESC LIMIT 1",
        )?;
        let mut query = stmt.query_map([encoded_text], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();
            let crack_json_vec: Vec<String> =
                serde_json::from_str(&path_str.clone()).unwrap_or_default();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: crack_json_vec,
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                input_length: row.get_unwrap(6),
                decoder_count: row.get_unwrap(7),
                checker_name: row.get(8).ok(),
                key_used: row.get(9).ok(),
                timestamp: row.get_unwrap(10),
            })
        })?;
        let row = query.next();
        match row {
            Some(cache_row) => Ok(Some(cache_row?)),
            None => Ok(None),
        }
    } else {
        // Old schema without id column
        let mut stmt = conn.prepare("SELECT * FROM cache WHERE encoded_text IS $1 LIMIT 1")?;
        let mut query = stmt.query_map([encoded_text], |row| {
            let path_str = row.get_unwrap::<usize, String>(2).to_owned();
            let crack_json_vec: Vec<String> =
                serde_json::from_str(&path_str.clone()).unwrap_or_default();

            Ok(CacheRow {
                id: 0, // No id in old schema
                encoded_text: row.get_unwrap(0),
                decoded_text: row.get_unwrap(1),
                path: crack_json_vec,
                successful: row.get_unwrap(3),
                execution_time_ms: row.get_unwrap(4),
                input_length: row.get_unwrap(5),
                decoder_count: row.get_unwrap(6),
                checker_name: row.get(7).ok(),
                key_used: row.get(8).ok(),
                timestamp: row.get_unwrap(9),
            })
        })?;
        let row = query.next();
        match row {
            Some(cache_row) => Ok(Some(cache_row?)),
            None => Ok(None),
        }
    }
}

/// Reads all cache entries ordered by timestamp (most recent first)
///
/// Returns all history entries for display in the TUI history panel.
/// Handles both old schema (without id) and new schema (with id) gracefully.
///
/// # Errors
///
/// Returns a `rusqlite::Error` on database errors.
pub fn read_cache_history() -> Result<Vec<CacheRow>, rusqlite::Error> {
    let conn = get_db_connection()?;

    // Check if the new schema (with id column) exists
    let has_id_column = {
        let mut stmt = conn.prepare("PRAGMA table_info(cache)")?;
        let columns: Vec<String> = stmt
            .query_map([], |row| row.get::<usize, String>(1))?
            .filter_map(|r| r.ok())
            .collect();
        columns.first().map(|s| s == "id").unwrap_or(false)
    };

    if has_id_column {
        // New schema with id column
        let mut stmt = conn.prepare("SELECT * FROM cache ORDER BY timestamp DESC")?;
        let query = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();
            let crack_json_vec: Vec<String> =
                serde_json::from_str(&path_str.clone()).unwrap_or_default();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: crack_json_vec,
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                input_length: row.get_unwrap(6),
                decoder_count: row.get_unwrap(7),
                checker_name: row.get(8).ok(),
                key_used: row.get(9).ok(),
                timestamp: row.get_unwrap(10),
            })
        })?;

        let mut results = Vec::new();
        for row in query {
            results.push(row?);
        }
        Ok(results)
    } else {
        // Old schema without id column - return empty history
        // User needs to delete ~/.ciphey/database.sqlite to use new schema
        Ok(Vec::new())
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
            decoded_text = $1,
            path = $2,
            successful = $3,
            execution_time_ms = $4,
            input_length = $5,
            decoder_count = $6,
            checker_name = $7,
            key_used = $8,
            timestamp = $9
            WHERE encoded_text = $10;",
        (
            cache_entry.decoded_text.clone(),
            path_json,
            successful,
            cache_entry.execution_time_ms,
            cache_entry.input_length,
            cache_entry.decoder_count,
            cache_entry.checker_name.clone(),
            cache_entry.key_used.clone(),
            get_timestamp(),
            cache_entry.encoded_text.clone(),
        ),
    );
    transaction.commit()?;
    conn_result
}

/// Adds a new decode failure record to the human_rejection table, or increments
/// rejection_count if the (plaintext, checker) combination already exists.
///
/// Optional context can be provided for encoded_text and decoder_path.
///
/// Returns the number of successfully inserted/updated rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn insert_human_rejection(
    plaintext: &str,
    check_result: &CheckResult,
    encoded_text: Option<&str>,
    decoder_path: Option<&str>,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let timestamp = get_timestamp();

    // Use INSERT OR REPLACE with a subquery to handle the upsert logic
    // This increments rejection_count and updates last_rejected if the row exists,
    // otherwise inserts a new row with rejection_count = 1
    let conn_result = transaction.execute(
        "INSERT INTO human_rejection (
            plaintext,
            encoded_text,
            checker,
            checker_description,
            check_description,
            decoder_path,
            rejection_count,
            first_rejected,
            last_rejected)
        VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $7)
        ON CONFLICT(plaintext, checker) DO UPDATE SET
            rejection_count = rejection_count + 1,
            last_rejected = $7,
            encoded_text = COALESCE($2, encoded_text),
            decoder_path = COALESCE($6, decoder_path)",
        (
            plaintext.to_owned(),
            encoded_text.map(|s| s.to_owned()),
            check_result.checker_name,
            Some(check_result.checker_description.to_owned()),
            Some(check_result.description.clone()),
            decoder_path.map(|s| s.to_owned()),
            timestamp,
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
            id: row.get_unwrap(0),
            plaintext: row.get_unwrap(1),
            encoded_text: row.get(2).ok(),
            checker: row.get_unwrap(3),
            checker_description: row.get(4).ok(),
            check_description: row.get(5).ok(),
            decoder_path: row.get(6).ok(),
            rejection_count: row.get_unwrap(7),
            first_rejected: row.get_unwrap(8),
            last_rejected: row.get_unwrap(9),
        })
    })?;
    let row = query.next();
    match row {
        Some(cache_row) => Ok(Some(cache_row?)),
        None => Ok(None),
    }
}

/// Updates a human_rejection row for a given plaintext and checker
///
/// Returns the number of updated rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn update_human_rejection(
    plaintext: &str,
    check_result: &CheckResult,
    encoded_text: Option<&str>,
    decoder_path: Option<&str>,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "UPDATE human_rejection SET 
            encoded_text = COALESCE($1, encoded_text),
            checker_description = $2,
            check_description = $3,
            decoder_path = COALESCE($4, decoder_path),
            last_rejected = $5
            WHERE plaintext = $6 AND checker = $7;",
        (
            encoded_text.map(|s| s.to_owned()),
            Some(check_result.checker_description.to_owned()),
            Some(check_result.description.clone()),
            decoder_path.map(|s| s.to_owned()),
            get_timestamp(),
            plaintext.to_owned(),
            check_result.checker_name,
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

// ============================================================================
// Wordlist Table Functions
// ============================================================================

/// Inserts a single word into the wordlist table
///
/// New words are enabled by default.
///
/// Returns the number of successfully inserted rows on success (0 if word already exists)
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn insert_word(word: &str, source: &str) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "INSERT OR IGNORE INTO wordlist (word, source, enabled, added_date) VALUES ($1, $2, true, $3)",
        (word.to_owned(), source.to_owned(), get_timestamp()),
    );
    transaction.commit()?;
    conn_result
}

/// Bulk inserts words into the wordlist table (efficient batch operation)
///
/// Each tuple in the slice is (word, source).
/// Uses INSERT OR IGNORE to skip duplicates.
/// New words are enabled by default.
///
/// Returns the number of successfully inserted rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn insert_words_batch(words: &[(&str, &str)]) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let timestamp = get_timestamp();

    let mut total_inserted = 0;
    for (word, source) in words {
        let result = transaction.execute(
            "INSERT OR IGNORE INTO wordlist (word, source, enabled, added_date) VALUES ($1, $2, true, $3)",
            (word.to_string(), source.to_string(), timestamp.clone()),
        )?;
        total_inserted += result;
    }

    transaction.commit()?;
    Ok(total_inserted)
}

/// Checks if a word exists and is enabled in the wordlist table
///
/// Returns true if the word exists and is enabled, false otherwise.
/// Disabled words are treated as non-existent for matching purposes.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn word_exists(word: &str) -> Result<bool, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt =
        conn.prepare("SELECT 1 FROM wordlist WHERE word = $1 AND enabled = true LIMIT 1")?;
    let exists = stmt.exists([word])?;
    Ok(exists)
}

/// Returns all enabled words in the wordlist table (for bloom filter building)
///
/// Disabled words are excluded from the result.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn read_all_words() -> Result<Vec<String>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare("SELECT word FROM wordlist WHERE enabled = true")?;
    let words = stmt
        .query_map([], |row| row.get::<usize, String>(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(words)
}

/// Returns the count of enabled words in the wordlist table (for bloom filter sizing)
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn get_word_count() -> Result<i64, rusqlite::Error> {
    let conn = get_db_connection()?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM wordlist WHERE enabled = true",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Deletes a word from the wordlist table
///
/// Returns number of successfully deleted rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn delete_word(word: &str) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result =
        transaction.execute("DELETE FROM wordlist WHERE word = $1", (word.to_owned(),));
    transaction.commit()?;
    conn_result
}

/// Sets the enabled status of a word in the wordlist table
///
/// Use this to disable words that should be excluded from matching without deleting them.
/// Disabled words won't appear in bloom filter builds or word existence checks.
///
/// Returns the number of updated rows on success (0 if word doesn't exist)
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn set_word_enabled(word: &str, enabled: bool) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "UPDATE wordlist SET enabled = $1 WHERE word = $2",
        (enabled, word.to_owned()),
    );
    transaction.commit()?;
    conn_result
}

/// Sets the enabled status for multiple words at once (batch operation)
///
/// Returns the number of updated rows on success
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn set_words_enabled_batch(words: &[&str], enabled: bool) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;

    let mut total_updated = 0;
    for word in words {
        let result = transaction.execute(
            "UPDATE wordlist SET enabled = $1 WHERE word = $2",
            (enabled, word.to_string()),
        )?;
        total_updated += result;
    }

    transaction.commit()?;
    Ok(total_updated)
}

/// Returns all disabled words in the wordlist table
///
/// Useful for displaying which words have been disabled by the user.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn get_disabled_words() -> Result<Vec<WordlistRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, word, source, enabled, added_date, file_id FROM wordlist WHERE enabled = false",
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok(WordlistRow {
                id: row.get_unwrap(0),
                word: row.get_unwrap(1),
                source: row.get_unwrap(2),
                enabled: row.get_unwrap(3),
                added_date: row.get_unwrap(4),
                file_id: row.get(5).ok(),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Returns the count of disabled words in the wordlist table
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn get_disabled_word_count() -> Result<i64, rusqlite::Error> {
    let conn = get_db_connection()?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM wordlist WHERE enabled = false",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Imports a HashSet of words into the database with a given source
///
/// This is useful for migrating existing wordlists from config files
/// to the database. Uses INSERT OR IGNORE to skip duplicates.
/// New words are enabled by default.
///
/// Returns the number of successfully inserted words
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn import_wordlist(
    words: &std::collections::HashSet<String>,
    source: &str,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let timestamp = get_timestamp();

    let mut total_inserted = 0;
    for word in words {
        let result = transaction.execute(
            "INSERT OR IGNORE INTO wordlist (word, source, enabled, added_date) VALUES ($1, $2, true, $3)",
            (word.clone(), source.to_owned(), timestamp.clone()),
        )?;
        total_inserted += result;
    }

    transaction.commit()?;
    Ok(total_inserted)
}

/// Reads a word from the wordlist table by its exact value
///
/// Returns Some(WordlistRow) if found, None otherwise.
/// Note: This returns the word regardless of its enabled status.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn read_word(word: &str) -> Result<Option<WordlistRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, word, source, enabled, added_date, file_id FROM wordlist WHERE word = $1",
    )?;
    let mut query = stmt.query_map([word], |row| {
        Ok(WordlistRow {
            id: row.get_unwrap(0),
            word: row.get_unwrap(1),
            source: row.get_unwrap(2),
            enabled: row.get_unwrap(3),
            added_date: row.get_unwrap(4),
            file_id: row.get(5).ok(),
        })
    })?;
    match query.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

// ============================================================================
// Wordlist Files Table Functions
// ============================================================================

/// Inserts a new wordlist file record into the wordlist_files table
///
/// Returns the ID of the inserted row on success.
///
/// # Errors
///
/// Returns rusqlite::Error on error (e.g., duplicate file_path)
pub fn insert_wordlist_file(
    filename: &str,
    file_path: &str,
    source: &str,
    word_count: i64,
) -> Result<i64, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    transaction.execute(
        "INSERT INTO wordlist_files (filename, file_path, source, word_count, enabled, added_date) 
         VALUES ($1, $2, $3, $4, true, $5)",
        (
            filename.to_owned(),
            file_path.to_owned(),
            source.to_owned(),
            word_count,
            get_timestamp(),
        ),
    )?;
    let id = transaction.last_insert_rowid();
    transaction.commit()?;
    Ok(id)
}

/// Returns all wordlist files from the database (for TUI display)
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn read_all_wordlist_files() -> Result<Vec<WordlistFileRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, filename, file_path, source, word_count, enabled, added_date 
         FROM wordlist_files ORDER BY added_date DESC",
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok(WordlistFileRow {
                id: row.get_unwrap(0),
                filename: row.get_unwrap(1),
                file_path: row.get_unwrap(2),
                source: row.get_unwrap(3),
                word_count: row.get_unwrap(4),
                enabled: row.get_unwrap(5),
                added_date: row.get_unwrap(6),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Returns a single wordlist file by ID
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn read_wordlist_file(id: i64) -> Result<Option<WordlistFileRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, filename, file_path, source, word_count, enabled, added_date 
         FROM wordlist_files WHERE id = $1",
    )?;
    let mut query = stmt.query_map([id], |row| {
        Ok(WordlistFileRow {
            id: row.get_unwrap(0),
            filename: row.get_unwrap(1),
            file_path: row.get_unwrap(2),
            source: row.get_unwrap(3),
            word_count: row.get_unwrap(4),
            enabled: row.get_unwrap(5),
            added_date: row.get_unwrap(6),
        })
    })?;
    match query.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

/// Checks if a wordlist file with the given path already exists
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn wordlist_file_exists(file_path: &str) -> Result<bool, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare("SELECT 1 FROM wordlist_files WHERE file_path = $1 LIMIT 1")?;
    let exists = stmt.exists([file_path])?;
    Ok(exists)
}

/// Sets the enabled status of a wordlist file
///
/// This does NOT cascade to words - use set_words_enabled_by_file_id for that.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn set_wordlist_file_enabled(id: i64, enabled: bool) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "UPDATE wordlist_files SET enabled = $1 WHERE id = $2",
        (enabled, id),
    );
    transaction.commit()?;
    conn_result
}

/// Deletes a wordlist file record
///
/// Due to ON DELETE CASCADE, this will also delete all associated words.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn delete_wordlist_file(id: i64) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    // First delete words (explicit for databases that don't support CASCADE)
    transaction.execute("DELETE FROM wordlist WHERE file_id = $1", (id,))?;
    // Then delete the file record
    let conn_result = transaction.execute("DELETE FROM wordlist_files WHERE id = $1", (id,));
    transaction.commit()?;
    conn_result
}

/// Sets the enabled status for all words associated with a wordlist file
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn set_words_enabled_by_file_id(file_id: i64, enabled: bool) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute(
        "UPDATE wordlist SET enabled = $1 WHERE file_id = $2",
        (enabled, file_id),
    );
    transaction.commit()?;
    conn_result
}

/// Deletes all words associated with a wordlist file
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn delete_words_by_file_id(file_id: i64) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let conn_result = transaction.execute("DELETE FROM wordlist WHERE file_id = $1", (file_id,));
    transaction.commit()?;
    conn_result
}

/// Bulk inserts words with a file_id reference (for importing from files)
///
/// Each word is associated with the given file_id.
/// Uses INSERT OR IGNORE to skip duplicates.
///
/// Returns the number of successfully inserted rows.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn insert_words_with_file_id(
    words: &[String],
    source: &str,
    file_id: i64,
) -> Result<usize, rusqlite::Error> {
    let mut conn = get_db_connection()?;
    let transaction = conn.transaction()?;
    let timestamp = get_timestamp();

    let mut total_inserted = 0;
    for word in words {
        let result = transaction.execute(
            "INSERT OR IGNORE INTO wordlist (word, source, enabled, added_date, file_id) 
             VALUES ($1, $2, true, $3, $4)",
            (word.clone(), source.to_owned(), timestamp.clone(), file_id),
        )?;
        total_inserted += result;
    }

    transaction.commit()?;
    Ok(total_inserted)
}

/// Imports a wordlist from a file path with progress callback
///
/// Reads the file line by line, counts total lines, then inserts words
/// while calling the progress callback periodically.
///
/// Returns the WordlistFileRow for the imported file on success.
///
/// # Arguments
///
/// * `file_path` - Path to the wordlist file
/// * `source` - Source identifier (e.g., "user_import")
/// * `progress_callback` - Callback function called with (current_line, total_lines)
///
/// # Errors
///
/// Returns rusqlite::Error on database error, or std::io::Error wrapped in rusqlite::Error
pub fn import_wordlist_from_file<F>(
    file_path: &str,
    source: &str,
    mut progress_callback: F,
) -> Result<WordlistFileRow, String>
where
    F: FnMut(usize, usize),
{
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    // Check if file exists
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    // Extract filename for display
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Check if already imported
    if wordlist_file_exists(file_path).map_err(|e| e.to_string())? {
        return Err(format!("Wordlist already imported: {}", file_path));
    }

    // Count total lines first
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    let total_lines = reader.lines().count();

    if total_lines == 0 {
        return Err("File is empty".to_string());
    }

    // Insert the file record first
    let file_id = insert_wordlist_file(&filename, file_path, source, total_lines as i64)
        .map_err(|e| format!("Failed to insert wordlist file record: {}", e))?;

    // Re-open file and import words in batches
    let file = File::open(file_path).map_err(|e| format!("Failed to reopen file: {}", e))?;
    let reader = BufReader::new(file);

    let mut conn = get_db_connection().map_err(|e| e.to_string())?;
    let transaction = conn.transaction().map_err(|e| e.to_string())?;
    let timestamp = get_timestamp();

    let mut current_line = 0;
    let mut total_inserted = 0;
    let batch_size = 1000; // Report progress every 1000 lines

    for line_result in reader.lines() {
        current_line += 1;

        if let Ok(word) = line_result {
            let word = word.trim();
            if !word.is_empty() {
                let result = transaction.execute(
                    "INSERT OR IGNORE INTO wordlist (word, source, enabled, added_date, file_id) 
                     VALUES ($1, $2, true, $3, $4)",
                    (
                        word.to_owned(),
                        source.to_owned(),
                        timestamp.clone(),
                        file_id,
                    ),
                );
                if let Ok(n) = result {
                    total_inserted += n;
                }
            }
        }

        // Report progress periodically
        if current_line % batch_size == 0 {
            progress_callback(current_line, total_lines);
        }
    }

    // Final progress update
    progress_callback(total_lines, total_lines);

    transaction.commit().map_err(|e| e.to_string())?;

    // Update word count with actual inserted count (may differ due to duplicates)
    let conn = get_db_connection().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE wordlist_files SET word_count = $1 WHERE id = $2",
        (total_inserted as i64, file_id),
    )
    .map_err(|e| e.to_string())?;

    // Return the file row
    read_wordlist_file(file_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to read inserted file record".to_string())
}

/// Returns the count of enabled wordlist files
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn get_wordlist_file_count() -> Result<i64, rusqlite::Error> {
    let conn = get_db_connection()?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM wordlist_files WHERE enabled = true",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Updates the file_id for words that were imported with a given source
/// but don't yet have a file_id set.
///
/// This is used to link words imported via `import_wordlist()` (which doesn't
/// set file_id) to a `wordlist_files` entry that was created separately.
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn update_words_file_id(source: &str, file_id: i64) -> Result<usize, rusqlite::Error> {
    let conn = get_db_connection()?;
    let updated = conn.execute(
        "UPDATE wordlist SET file_id = $1 WHERE source = $2 AND file_id IS NULL",
        (file_id, source.to_owned()),
    )?;
    Ok(updated)
}

/// Returns the total word count across all enabled wordlist files
///
/// # Errors
///
/// Returns rusqlite::Error on error
pub fn get_total_word_count_from_files() -> Result<i64, rusqlite::Error> {
    let conn = get_db_connection()?;
    let count: i64 = conn.query_row(
        "SELECT COALESCE(SUM(word_count), 0) FROM wordlist_files WHERE enabled = true",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

// ============================================================================
// AI Cache Functions
// ============================================================================

/// Inserts or updates an AI response cache entry.
///
/// Uses SQLite's INSERT OR REPLACE to handle upserts on the unique request_key.
///
/// # Arguments
///
/// * `request_key` - The unique cache key for this request
/// * `function_type` - The AI function type ("explain_step", "detect_language", "translate")
/// * `response` - The AI response text
/// * `model` - The model name used
///
/// # Errors
///
/// Returns rusqlite::Error if the database operation fails.
pub fn insert_ai_cache(
    request_key: &str,
    function_type: &str,
    response: &str,
    model: &str,
) -> Result<(), rusqlite::Error> {
    let conn = get_db_connection()?;
    conn.execute(
        "INSERT OR REPLACE INTO ai_cache (request_key, function_type, response, model, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![request_key, function_type, response, model, get_timestamp()],
    )?;
    Ok(())
}

/// Reads a cached AI response by request key.
///
/// # Arguments
///
/// * `request_key` - The unique cache key to look up
///
/// # Returns
///
/// `Ok(Some(AiCacheRow))` if found, `Ok(None)` if not cached.
///
/// # Errors
///
/// Returns rusqlite::Error if the database operation fails.
pub fn read_ai_cache(request_key: &str) -> Result<Option<AiCacheRow>, rusqlite::Error> {
    let conn = get_db_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, request_key, function_type, response, model, created_at
         FROM ai_cache
         WHERE request_key = ?1",
    )?;
    let mut rows = stmt.query_map([request_key], |row| {
        Ok(AiCacheRow {
            id: row.get(0)?,
            request_key: row.get(1)?,
            function_type: row.get(2)?,
            response: row.get(3)?,
            model: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// Clears all entries from the AI response cache.
///
/// This is useful when the user changes their AI model, which would
/// invalidate all cached responses.
///
/// # Errors
///
/// Returns rusqlite::Error if the database operation fails.
pub fn clear_ai_cache() -> Result<(), rusqlite::Error> {
    let conn = get_db_connection()?;
    conn.execute("DELETE FROM ai_cache", ())?;
    Ok(())
}

// ============================================================================
// Cache AI Explanations Functions
// ============================================================================

/// Updates the AI explanations JSON for a specific cache entry.
///
/// Merges the given step explanation into the existing JSON (or creates a new one).
///
/// # Arguments
///
/// * `cache_id` - The cache entry ID to update
/// * `step_index` - The step index in the path
/// * `explanation` - The AI-generated explanation text
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn update_cache_ai_explanation(
    cache_id: i64,
    step_index: usize,
    explanation: &str,
) -> Result<(), rusqlite::Error> {
    let conn = get_db_connection()?;

    // Read existing explanations
    let existing_json: Option<String> = conn
        .query_row(
            "SELECT ai_explanations FROM cache WHERE id = ?1",
            [cache_id],
            |row| row.get(0),
        )
        .ok();

    let mut explanations: std::collections::HashMap<String, String> = existing_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default();

    explanations.insert(step_index.to_string(), explanation.to_string());

    let json = serde_json::to_string(&explanations).unwrap_or_default();
    conn.execute(
        "UPDATE cache SET ai_explanations = ?1 WHERE id = ?2",
        rusqlite::params![json, cache_id],
    )?;

    Ok(())
}

/// Reads all AI explanations for a cache entry.
///
/// # Arguments
///
/// * `cache_id` - The cache entry ID
///
/// # Returns
///
/// A HashMap mapping step index to explanation text. Empty if no explanations exist.
///
/// # Errors
///
/// Returns rusqlite::Error on database error.
pub fn read_cache_ai_explanations(
    cache_id: i64,
) -> Result<std::collections::HashMap<usize, String>, rusqlite::Error> {
    let conn = get_db_connection()?;

    let json: Option<String> = conn
        .query_row(
            "SELECT ai_explanations FROM cache WHERE id = ?1",
            [cache_id],
            |row| row.get(0),
        )
        .ok()
        .flatten();

    let map: std::collections::HashMap<usize, String> = json
        .as_deref()
        .and_then(|j| {
            let string_map: std::collections::HashMap<String, String> =
                serde_json::from_str(j).ok()?;
            Some(
                string_map
                    .into_iter()
                    .filter_map(|(k, v)| k.parse::<usize>().ok().map(|idx| (idx, v)))
                    .collect(),
            )
        })
        .unwrap_or_default();

    Ok(map)
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
        set_db_path(Some(path));
    }

    /// Helper function for generating a cache row
    fn generate_cache_row(
        encoded_text: &str,
        decoded_text: &str,
    ) -> (CrackResult, CacheRow, CacheEntry) {
        let mock_decoder = Decoder::<MockDecoder>::new();
        let mut mock_crack_result = CrackResult::new(&mock_decoder, encoded_text.to_owned());
        mock_crack_result.success = true;
        mock_crack_result.unencrypted_text = Some(vec![decoded_text.to_owned()]);

        let expected_cache_row = CacheRow {
            id: 1, // Will be auto-assigned
            encoded_text: encoded_text.to_owned(),
            decoded_text: decoded_text.to_owned(),
            path: match serde_json::to_string(&mock_crack_result) {
                Ok(json) => vec![json],
                Err(_) => vec![],
            },
            successful: true,
            execution_time_ms: 100,
            input_length: encoded_text.len() as i64,
            decoder_count: 1,
            checker_name: None,
            key_used: None,
            timestamp: String::new(),
        };

        let cache_entry = CacheEntry {
            encoded_text: encoded_text.to_owned(),
            decoded_text: decoded_text.to_owned(),
            path: vec![mock_crack_result.clone()],
            execution_time_ms: 100,
            input_length: encoded_text.len() as i64,
            decoder_count: 1,
            checker_name: None,
            key_used: None,
        };
        (mock_crack_result, expected_cache_row, cache_entry)
    }

    /// Helper function for generating a new human_rejection row
    fn generate_human_rejection_row<Type>(
        plaintext: &str,
        checker_used: Checker<Type>,
    ) -> (CheckResult, HumanRejectionRow) {
        let check_result = CheckResult {
            is_identified: false,
            text: "".to_string(),
            checker_name: checker_used.name,
            checker_description: checker_used.description,
            description: "test description".to_string(),
            link: checker_used.link,
        };

        let expected_row = HumanRejectionRow {
            id: 1, // Will be auto-assigned
            plaintext: plaintext.to_owned(),
            encoded_text: None,
            checker: String::from(check_result.checker_name),
            checker_description: Some(check_result.checker_description.to_owned()),
            check_description: Some(check_result.description.clone()),
            decoder_path: None,
            rejection_count: 1,
            first_rejected: String::new(),
            last_rejected: String::new(),
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
        assert_eq!(name_list[0], "id");
        assert_eq!(name_list[1], "encoded_text");
        assert_eq!(name_list[2], "decoded_text");
        assert_eq!(name_list[3], "path");
        assert_eq!(name_list[4], "successful");
        assert_eq!(name_list[5], "execution_time_ms");
        assert_eq!(name_list[6], "input_length");
        assert_eq!(name_list[7], "decoder_count");
        assert_eq!(name_list[8], "checker_name");
        assert_eq!(name_list[9], "key_used");
        assert_eq!(name_list[10], "timestamp");

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
        assert_eq!(type_list[6], "INTEGER");
        assert_eq!(type_list[7], "INTEGER");
        assert_eq!(type_list[8], "TEXT");
        assert_eq!(type_list[9], "TEXT");
        assert_eq!(type_list[10], "DATETIME");
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
        assert_eq!(name_list[0], "id");
        assert_eq!(name_list[1], "plaintext");
        assert_eq!(name_list[2], "encoded_text");
        assert_eq!(name_list[3], "checker");
        assert_eq!(name_list[4], "checker_description");
        assert_eq!(name_list[5], "check_description");
        assert_eq!(name_list[6], "decoder_path");
        assert_eq!(name_list[7], "rejection_count");
        assert_eq!(name_list[8], "first_rejected");
        assert_eq!(name_list[9], "last_rejected");

        let type_result = stmt.query_map([], |row| row.get::<usize, String>(2));
        assert!(type_result.is_ok());
        let type_query = type_result.unwrap();
        let type_list: Vec<String> = type_query.map(|row| row.unwrap()).collect();
        assert_eq!(type_list[0], "INTEGER");
        assert_eq!(type_list[1], "TEXT");
        assert_eq!(type_list[2], "TEXT");
        assert_eq!(type_list[3], "TEXT");
        assert_eq!(type_list[4], "TEXT");
        assert_eq!(type_list[5], "TEXT");
        assert_eq!(type_list[6], "JSON");
        assert_eq!(type_list[7], "INTEGER");
        assert_eq!(type_list[8], "DATETIME");
        assert_eq!(type_list[9], "DATETIME");
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
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: serde_json::from_str(&path_str).unwrap_or_default(),
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                input_length: row.get_unwrap(6),
                decoder_count: row.get_unwrap(7),
                checker_name: row.get(8).ok(),
                key_used: row.get(9).ok(),
                timestamp: row.get_unwrap(10),
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

        let (_mock_crack_result, mut expected_cache_row, cache_entry) =
            generate_cache_row(&encoded_text, &decoded_text);
        let row_result = insert_cache(&cache_entry);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: serde_json::from_str(&path_str).unwrap_or_default(),
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                input_length: row.get_unwrap(6),
                decoder_count: row.get_unwrap(7),
                checker_name: row.get(8).ok(),
                key_used: row.get(9).ok(),
                timestamp: row.get_unwrap(10),
            })
        });
        assert!(query_result.is_ok());
        let cache_row: CacheRow = query_result.unwrap().next().unwrap().unwrap();
        expected_cache_row.timestamp = cache_row.timestamp.clone();
        expected_cache_row.id = cache_row.id; // Id is auto-assigned
        assert_eq!(cache_row, expected_cache_row);
    }

    #[test]
    fn cache_insert_2_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);
        let row_result = insert_cache(&cache_entry_1);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(&encoded_text_2, &decoded_text_2);
        let row_result = insert_cache(&cache_entry_2);
        assert!(row_result.is_ok());
        assert_eq!(row_result.unwrap(), 2);

        let stmt_result = conn.prepare("SELECT * FROM cache;");
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            let path_str = row.get_unwrap::<usize, String>(3).to_owned();

            Ok(CacheRow {
                id: row.get_unwrap(0),
                encoded_text: row.get_unwrap(1),
                decoded_text: row.get_unwrap(2),
                path: serde_json::from_str(&path_str).unwrap_or_default(),
                successful: row.get_unwrap(4),
                execution_time_ms: row.get_unwrap(5),
                input_length: row.get_unwrap(6),
                decoder_count: row.get_unwrap(7),
                checker_name: row.get(8).ok(),
                key_used: row.get(9).ok(),
                timestamp: row.get_unwrap(10),
            })
        });
        let mut query = query_result.unwrap();
        let cache_row: CacheRow = query.next().unwrap().unwrap();
        expected_cache_row_1.timestamp = cache_row.timestamp.clone();
        expected_cache_row_1.id = cache_row.id;
        assert_eq!(cache_row, expected_cache_row_1);
        let cache_row: CacheRow = query.next().unwrap().unwrap();
        expected_cache_row_2.timestamp = cache_row.timestamp.clone();
        expected_cache_row_2.id = cache_row.id;
        assert_eq!(cache_row, expected_cache_row_2);
    }

    #[test]
    fn cache_read_hit() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");

        let (_mock_crack_result, mut expected_cache_row, cache_entry) =
            generate_cache_row(&encoded_text, &decoded_text);
        let _row_result = insert_cache(&cache_entry);

        let cache_result = read_cache(&encoded_text);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row.timestamp = cache_row.timestamp.clone();
        expected_cache_row.id = cache_row.id;
        assert_eq!(cache_row, expected_cache_row);
    }

    #[test]
    fn cache_read_2_hit() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);
        let _row_result = insert_cache(&cache_entry_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(&encoded_text_2, &decoded_text_2);
        let _row_result = insert_cache(&cache_entry_2);

        let cache_result = read_cache(&encoded_text_1);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row_1.timestamp = cache_row.timestamp.clone();
        expected_cache_row_1.id = cache_row.id;
        assert_eq!(cache_row, expected_cache_row_1);

        let cache_result = read_cache(&encoded_text_2);
        assert!(cache_result.is_ok());
        let cache_row_result: Option<CacheRow> = cache_result.unwrap();
        assert!(cache_row_result.is_some());
        let cache_row = cache_row_result.unwrap();
        expected_cache_row_2.timestamp = cache_row.timestamp.clone();
        expected_cache_row_2.id = cache_row.id;
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

        let (_mock_crack_result_1, _expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let _decoded_text_2 = String::from("world hello");

        let (_mock_crack_result_2, _expected_cache_row_2, cache_entry_2) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);

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

        let (_mock_crack_result, _expected_cache_row, cache_entry) =
            generate_cache_row(&encoded_text, &decoded_text);
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

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(&encoded_text_2, &decoded_text_2);

        let _row_result = insert_cache(&cache_entry_1);
        let _row_result = insert_cache(&cache_entry_2);

        let read_result = read_cache(&encoded_text_1).unwrap();
        assert!(read_result.is_some());
        let row: CacheRow = read_result.unwrap();
        expected_cache_row_1.timestamp = row.timestamp.clone();
        expected_cache_row_1.id = row.id;
        assert_eq!(row, expected_cache_row_1);

        let read_result = read_cache(&encoded_text_2).unwrap();
        assert!(read_result.is_some());
        let row: CacheRow = read_result.unwrap();
        expected_cache_row_2.timestamp = row.timestamp.clone();
        expected_cache_row_2.id = row.id;
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

        let (_mock_crack_result_1, _expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);
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

        let (_mock_crack_result, mut expected_cache_row, cache_entry) =
            generate_cache_row(&encoded_text, &decoded_text_err);
        let _row_result = insert_cache(&cache_entry);

        let (_mock_crack_result_new, mut expected_cache_row_new, cache_entry_new) =
            generate_cache_row(&encoded_text, &decoded_text);
        let update_result = update_cache(&cache_entry_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_cache(&encoded_text);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_new.timestamp = row.timestamp.clone();
        expected_cache_row_new.id = row.id;
        expected_cache_row.timestamp = row.timestamp.clone();
        expected_cache_row.id = row.id;
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

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_err);
        let _row_result = insert_cache(&cache_entry_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let (_mock_crack_result_2, _expected_cache_row_2, cache_entry_2) =
            generate_cache_row(&encoded_text_2, &decoded_text_2);
        let _row_result = insert_cache(&cache_entry_2);

        let (_mock_crack_result_new, mut expected_cache_row_new, cache_entry_new) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);
        let update_result = update_cache(&cache_entry_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_cache(&encoded_text_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_new.timestamp = row.timestamp.clone();
        expected_cache_row_new.id = row.id;
        expected_cache_row_1.timestamp = row.timestamp.clone();
        expected_cache_row_1.id = row.id;
        assert_eq!(row, expected_cache_row_new);
        assert_ne!(row, expected_cache_row_1);
    }

    #[test]
    fn cache_update_1_change_2_entry_no_match() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
        let decoded_text_1 = String::from("hello world");

        let (_mock_crack_result_1, mut expected_cache_row_1, cache_entry_1) =
            generate_cache_row(&encoded_text_1, &decoded_text_1);
        let _row_result = insert_cache(&cache_entry_1);

        let encoded_text_2 = String::from("d29ybGQgaGVsbG8K");
        let decoded_text_2 = String::from("world hello");

        let (_mock_crack_result_2, mut expected_cache_row_2, cache_entry_2) =
            generate_cache_row(&encoded_text_2, &decoded_text_2);
        let _row_result = insert_cache(&cache_entry_2);

        let encoded_text_new = String::from("c29tZSBuZXcgdGV4dAo=");
        let decoded_text_new = String::from("some new text");

        let (_mock_crack_result_new, mut expected_cache_row_new, cache_entry_new) =
            generate_cache_row(&encoded_text_new, &decoded_text_new);

        let update_result = update_cache(&cache_entry_new);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);

        let row_result = read_cache(&encoded_text_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_new.timestamp = row.timestamp.clone();
        expected_cache_row_new.id = row.id;
        expected_cache_row_1.timestamp = row.timestamp.clone();
        expected_cache_row_1.id = row.id;
        assert_ne!(row, expected_cache_row_new);
        assert_eq!(row, expected_cache_row_1);

        let row_result = read_cache(&encoded_text_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_cache_row_2.timestamp = row.timestamp.clone();
        expected_cache_row_2.id = row.id;
        assert_eq!(row, expected_cache_row_2);
    }

    #[test]
    fn cache_update_empty() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let encoded_text = String::from("aGVsbG8gd29ybGQK");
        let decoded_text = String::from("hello world");

        let (_mock_crack_result, mut _expected_cache_row, cache_entry) =
            generate_cache_row(&encoded_text, &decoded_text);

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
                id: row.get_unwrap(0),
                plaintext: row.get_unwrap(1),
                encoded_text: row.get(2).ok(),
                checker: row.get_unwrap(3),
                checker_description: row.get(4).ok(),
                check_description: row.get(5).ok(),
                decoder_path: row.get(6).ok(),
                rejection_count: row.get_unwrap(7),
                first_rejected: row.get_unwrap(8),
                last_rejected: row.get_unwrap(9),
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

        let checker_used = Checker::<Athena>::new();

        let (check_result, mut expected_row) =
            generate_human_rejection_row(&plaintext, checker_used);

        let result = insert_human_rejection(&plaintext, &check_result, None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM human_rejection;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            Ok(HumanRejectionRow {
                id: row.get_unwrap(0),
                plaintext: row.get_unwrap(1),
                encoded_text: row.get(2).ok(),
                checker: row.get_unwrap(3),
                checker_description: row.get(4).ok(),
                check_description: row.get(5).ok(),
                decoder_path: row.get(6).ok(),
                rejection_count: row.get_unwrap(7),
                first_rejected: row.get_unwrap(8),
                last_rejected: row.get_unwrap(9),
            })
        });
        assert!(query_result.is_ok());
        let mut query = query_result.unwrap();
        let row: HumanRejectionRow = query.next().unwrap().unwrap();
        expected_row.first_rejected = row.first_rejected.clone();
        expected_row.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row);
    }

    #[test]
    fn human_rejection_insert_2_success() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext1");
        let checker_used_1 = Checker::<Athena>::new();

        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);

        let result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();

        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(&plaintext_2, checker_used_2);

        let result = insert_human_rejection(&plaintext_2, &check_result_2, None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let stmt_result = conn.prepare("SELECT * FROM human_rejection;");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();
        let query_result = stmt.query_map([], |row| {
            Ok(HumanRejectionRow {
                id: row.get_unwrap(0),
                plaintext: row.get_unwrap(1),
                encoded_text: row.get(2).ok(),
                checker: row.get_unwrap(3),
                checker_description: row.get(4).ok(),
                check_description: row.get(5).ok(),
                decoder_path: row.get(6).ok(),
                rejection_count: row.get_unwrap(7),
                first_rejected: row.get_unwrap(8),
                last_rejected: row.get_unwrap(9),
            })
        });
        assert!(query_result.is_ok());
        let mut query = query_result.unwrap();
        let row: HumanRejectionRow = query.next().unwrap().unwrap();
        expected_row_1.first_rejected = row.first_rejected.clone();
        expected_row_1.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_1);
        let row: HumanRejectionRow = query.next().unwrap().unwrap();
        expected_row_2.id = row.id;
        expected_row_2.first_rejected = row.first_rejected.clone();
        expected_row_2.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_2);
    }

    #[test]
    fn failed_decode_read_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_used = Checker::<Athena>::new();

        let (check_result, mut expected_row) =
            generate_human_rejection_row(&plaintext, checker_used);

        let _result = insert_human_rejection(&plaintext, &check_result, None, None);

        let row_result = read_human_rejection(&plaintext);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row.first_rejected = row.first_rejected.clone();
        expected_row.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row);
    }

    #[test]
    fn failed_decode_read_2_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext");
        let checker_used_1 = Checker::<Athena>::new();

        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);

        let _result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();

        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(&plaintext_2, checker_used_2);

        let _result = insert_human_rejection(&plaintext_2, &check_result_2, None, None);

        let row_result = read_human_rejection(&plaintext_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_1.first_rejected = row.first_rejected.clone();
        expected_row_1.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_1);

        let row_result = read_human_rejection(&plaintext_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_2.id = row.id;
        expected_row_2.first_rejected = row.first_rejected.clone();
        expected_row_2.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_2);
    }

    #[test]
    fn human_rejection_read_empty_miss() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_used = Checker::<Athena>::new();

        let (check_result, _expected_row) = generate_human_rejection_row(&plaintext, checker_used);

        let _result = insert_human_rejection(&plaintext, &check_result, None, None);
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

        let (check_result_1, _expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);
        let _result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();

        let (check_result_2, _expected_row_2) =
            generate_human_rejection_row(&plaintext_2, checker_used_2);
        let _result = insert_human_rejection(&plaintext_2, &check_result_2, None, None);

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

        let (check_result, _expected_row) = generate_human_rejection_row(&plaintext, checker_used);
        let _result = insert_human_rejection(&plaintext, &check_result, None, None);
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
        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);
        let _result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(&plaintext_2, checker_used_2);
        let _result = insert_human_rejection(&plaintext_2, &check_result_2, None, None);

        let read_result = read_human_rejection(&plaintext_1).unwrap();
        assert!(read_result.is_some());
        let row: HumanRejectionRow = read_result.unwrap();
        expected_row_1.first_rejected = row.first_rejected.clone();
        expected_row_1.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_1);

        let read_result = read_human_rejection(&plaintext_2).unwrap();
        assert!(read_result.is_some());
        let row: HumanRejectionRow = read_result.unwrap();
        expected_row_2.id = row.id;
        expected_row_2.first_rejected = row.first_rejected.clone();
        expected_row_2.last_rejected = row.last_rejected.clone();
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
        let (check_result_1, _expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);
        let row_result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);
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
        let (check_result, mut expected_row) =
            generate_human_rejection_row(&plaintext, checker_used);
        let _row_result = insert_human_rejection(&plaintext, &check_result, None, None);

        // Use the same checker type since update requires matching (plaintext, checker)
        let checker_new = Checker::<Athena>::new();
        let (check_result_new, mut expected_row_new) =
            generate_human_rejection_row(&plaintext, checker_new);
        let update_result = update_human_rejection(&plaintext, &check_result_new, None, None);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_human_rejection(&plaintext);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row.first_rejected = row.first_rejected.clone();
        expected_row.last_rejected = row.last_rejected.clone();
        expected_row_new.first_rejected = row.first_rejected.clone();
        expected_row_new.last_rejected = row.last_rejected.clone();
        // After update, row should match expected_row_new (same checker)
        assert_eq!(row, expected_row_new);
    }

    #[test]
    fn human_rejection_update_1_change_2_entry_success() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext1");
        let checker_used_1 = Checker::<Athena>::new();
        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);
        let _row_result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(&plaintext_2, checker_used_2);
        let _row_result = insert_human_rejection(&plaintext_2, &check_result_2, None, None);

        // Use the same checker type as the original insert since update requires matching (plaintext, checker)
        let checker_new = Checker::<Athena>::new();
        let (check_result_new, mut expected_row_new) =
            generate_human_rejection_row(&plaintext_1, checker_new);

        let update_result = update_human_rejection(&plaintext_1, &check_result_new, None, None);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 1);

        let row_result = read_human_rejection(&plaintext_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_1.first_rejected = row.first_rejected.clone();
        expected_row_1.last_rejected = row.last_rejected.clone();
        expected_row_new.first_rejected = row.first_rejected.clone();
        expected_row_new.last_rejected = row.last_rejected.clone();
        // After update, row should match expected_row_new (same checker as original)
        assert_eq!(row, expected_row_new);

        let row_result = read_human_rejection(&plaintext_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_2.id = row.id;
        expected_row_2.first_rejected = row.first_rejected.clone();
        expected_row_2.last_rejected = row.last_rejected.clone();
        expected_row_new.first_rejected = row.first_rejected.clone();
        expected_row_new.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_2);
        assert_ne!(row, expected_row_new);
    }

    #[test]
    fn human_rejection_update_1_change_2_entry_no_match() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext_1 = String::from("plaintext1");
        let checker_used_1 = Checker::<Athena>::new();
        let (check_result_1, mut expected_row_1) =
            generate_human_rejection_row(&plaintext_1, checker_used_1);
        let _row_result = insert_human_rejection(&plaintext_1, &check_result_1, None, None);

        let plaintext_2 = String::from("plaintext2");
        let checker_used_2 = Checker::<EnglishChecker>::new();
        let (check_result_2, mut expected_row_2) =
            generate_human_rejection_row(&plaintext_2, checker_used_2);
        let _row_result = insert_human_rejection(&plaintext_2, &check_result_2, None, None);

        let plaintext_new = String::from("new plaintext");

        let checker_new = Checker::<EnglishChecker>::new();
        let (check_result_new, mut expected_row_new) =
            generate_human_rejection_row(&plaintext_new, checker_new);

        let update_result = update_human_rejection(&plaintext_new, &check_result_new, None, None);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);

        let row_result = read_human_rejection(&plaintext_1);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_1.first_rejected = row.first_rejected.clone();
        expected_row_1.last_rejected = row.last_rejected.clone();
        expected_row_new.first_rejected = row.first_rejected.clone();
        expected_row_new.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_1);
        assert_ne!(row, expected_row_new);

        let row_result = read_human_rejection(&plaintext_2);
        assert!(row_result.is_ok());
        let row_result = row_result.unwrap();
        assert!(row_result.is_some());
        let row = row_result.unwrap();
        expected_row_2.id = row.id;
        expected_row_2.first_rejected = row.first_rejected.clone();
        expected_row_2.last_rejected = row.last_rejected.clone();
        expected_row_new.first_rejected = row.first_rejected.clone();
        expected_row_new.last_rejected = row.last_rejected.clone();
        assert_eq!(row, expected_row_2);
        assert_ne!(row, expected_row_new);
    }

    #[test]
    fn human_rejection_update_empty() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let plaintext = String::from("plaintext");
        let checker_new = Checker::<EnglishChecker>::new();
        let (check_result_new, _expected_row_new) =
            generate_human_rejection_row(&plaintext, checker_new);
        let update_result = update_human_rejection(&plaintext, &check_result_new, None, None);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), 0);
    }

    // ============================================================================
    // Wordlist Table Tests
    // ============================================================================

    #[test]
    fn wordlist_table_created() {
        set_test_db_path();
        let conn = init_database().unwrap();

        let stmt_result =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='wordlist';");
        assert!(stmt_result.is_ok());
        let mut stmt = stmt_result.unwrap();

        let query_result = stmt.query_map([], |row| row.get::<usize, String>(0));
        assert!(query_result.is_ok());
        assert_eq!(query_result.unwrap().count(), 1);
    }

    #[test]
    fn wordlist_insert_single_word() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let result = insert_word("password123", "test_source");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Verify word exists
        let exists = word_exists("password123").unwrap();
        assert!(exists);
    }

    #[test]
    fn wordlist_insert_duplicate_word() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert first time
        let result = insert_word("duplicate_word", "test_source");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Insert same word again - should be ignored (INSERT OR IGNORE)
        let result = insert_word("duplicate_word", "test_source");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn wordlist_word_exists() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Word doesn't exist initially
        let exists = word_exists("nonexistent_word").unwrap();
        assert!(!exists);

        // Insert word
        insert_word("test_word", "test_source").unwrap();

        // Now it should exist
        let exists = word_exists("test_word").unwrap();
        assert!(exists);
    }

    #[test]
    fn wordlist_read_all_words() {
        set_test_db_path();
        let conn = init_database().unwrap();

        // Clear any leftover words from other tests sharing the in-memory DB
        conn.execute("DELETE FROM wordlist", ()).unwrap();

        // Insert multiple words
        insert_word("word1", "test_source").unwrap();
        insert_word("word2", "test_source").unwrap();
        insert_word("word3", "test_source").unwrap();

        // Read all words
        let words = read_all_words().unwrap();
        assert_eq!(words.len(), 3);
        assert!(words.contains(&"word1".to_string()));
        assert!(words.contains(&"word2".to_string()));
        assert!(words.contains(&"word3".to_string()));
    }

    #[test]
    fn wordlist_get_word_count() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Initially empty
        let count = get_word_count().unwrap();
        assert_eq!(count, 0);

        // Insert words
        insert_word("word1", "test_source").unwrap();
        insert_word("word2", "test_source").unwrap();

        // Count should be 2
        let count = get_word_count().unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn wordlist_delete_word() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert word
        insert_word("to_delete", "test_source").unwrap();
        assert!(word_exists("to_delete").unwrap());

        // Delete word
        let result = delete_word("to_delete");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Verify deleted
        assert!(!word_exists("to_delete").unwrap());
    }

    #[test]
    fn wordlist_delete_nonexistent() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let result = delete_word("nonexistent");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn wordlist_import_hashset() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Create a HashSet of words
        let mut words = std::collections::HashSet::new();
        words.insert("import1".to_string());
        words.insert("import2".to_string());
        words.insert("import3".to_string());

        // Import
        let result = import_wordlist(&words, "import_test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);

        // Verify all words exist
        assert!(word_exists("import1").unwrap());
        assert!(word_exists("import2").unwrap());
        assert!(word_exists("import3").unwrap());

        // Verify count
        assert_eq!(get_word_count().unwrap(), 3);
    }

    #[test]
    fn wordlist_insert_batch() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let words = vec![
            ("batch1", "batch_test"),
            ("batch2", "batch_test"),
            ("batch3", "batch_test"),
        ];

        let result = insert_words_batch(&words);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);

        // Verify all words exist
        assert!(word_exists("batch1").unwrap());
        assert!(word_exists("batch2").unwrap());
        assert!(word_exists("batch3").unwrap());
    }

    #[test]
    fn wordlist_read_word() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert a word
        insert_word("readable_word", "read_test").unwrap();

        // Read it back
        let row = read_word("readable_word").unwrap();
        assert!(row.is_some());
        let row = row.unwrap();
        assert_eq!(row.word, "readable_word");
        assert_eq!(row.source, "read_test");
        assert!(row.enabled); // New words should be enabled by default

        // Non-existent word
        let row = read_word("nonexistent").unwrap();
        assert!(row.is_none());
    }

    #[test]
    fn wordlist_set_word_enabled() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert a word (enabled by default)
        insert_word("toggle_word", "test_source").unwrap();
        assert!(word_exists("toggle_word").unwrap());

        // Disable the word
        let result = set_word_enabled("toggle_word", false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // word_exists should return false for disabled word
        assert!(!word_exists("toggle_word").unwrap());

        // But read_word should still find it
        let row = read_word("toggle_word").unwrap();
        assert!(row.is_some());
        assert!(!row.unwrap().enabled);

        // Re-enable the word
        let result = set_word_enabled("toggle_word", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Now word_exists should return true again
        assert!(word_exists("toggle_word").unwrap());
    }

    #[test]
    fn wordlist_disabled_excluded_from_read_all() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert multiple words
        insert_word("enabled1", "test_source").unwrap();
        insert_word("enabled2", "test_source").unwrap();
        insert_word("to_disable", "test_source").unwrap();

        // Initially all 3 should be in read_all_words
        let words = read_all_words().unwrap();
        assert_eq!(words.len(), 3);

        // Disable one word
        set_word_enabled("to_disable", false).unwrap();

        // Now only 2 should be returned
        let words = read_all_words().unwrap();
        assert_eq!(words.len(), 2);
        assert!(words.contains(&"enabled1".to_string()));
        assert!(words.contains(&"enabled2".to_string()));
        assert!(!words.contains(&"to_disable".to_string()));
    }

    #[test]
    fn wordlist_disabled_excluded_from_count() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert words
        insert_word("count1", "test_source").unwrap();
        insert_word("count2", "test_source").unwrap();
        insert_word("count3", "test_source").unwrap();

        // Initially count is 3
        assert_eq!(get_word_count().unwrap(), 3);

        // Disable one word
        set_word_enabled("count2", false).unwrap();

        // Count should now be 2
        assert_eq!(get_word_count().unwrap(), 2);
    }

    #[test]
    fn wordlist_get_disabled_words() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert words
        insert_word("stay_enabled", "test_source").unwrap();
        insert_word("disable_me1", "test_source").unwrap();
        insert_word("disable_me2", "test_source").unwrap();

        // Initially no disabled words
        let disabled = get_disabled_words().unwrap();
        assert_eq!(disabled.len(), 0);

        // Disable some words
        set_word_enabled("disable_me1", false).unwrap();
        set_word_enabled("disable_me2", false).unwrap();

        // Should have 2 disabled words
        let disabled = get_disabled_words().unwrap();
        assert_eq!(disabled.len(), 2);
        let disabled_words: Vec<String> = disabled.iter().map(|r| r.word.clone()).collect();
        assert!(disabled_words.contains(&"disable_me1".to_string()));
        assert!(disabled_words.contains(&"disable_me2".to_string()));
        assert!(!disabled_words.contains(&"stay_enabled".to_string()));
    }

    #[test]
    fn wordlist_get_disabled_word_count() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert words
        insert_word("word_a", "test_source").unwrap();
        insert_word("word_b", "test_source").unwrap();
        insert_word("word_c", "test_source").unwrap();

        // Initially 0 disabled
        assert_eq!(get_disabled_word_count().unwrap(), 0);

        // Disable 2 words
        set_word_enabled("word_a", false).unwrap();
        set_word_enabled("word_c", false).unwrap();

        // Should have 2 disabled
        assert_eq!(get_disabled_word_count().unwrap(), 2);
    }

    #[test]
    fn wordlist_set_words_enabled_batch() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Insert words
        insert_word("batch_a", "test_source").unwrap();
        insert_word("batch_b", "test_source").unwrap();
        insert_word("batch_c", "test_source").unwrap();
        insert_word("batch_d", "test_source").unwrap();

        // Initially all enabled, count is 4
        assert_eq!(get_word_count().unwrap(), 4);

        // Disable multiple words at once
        let result = set_words_enabled_batch(&["batch_a", "batch_c", "batch_d"], false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);

        // Only batch_b should be enabled
        assert_eq!(get_word_count().unwrap(), 1);
        assert!(word_exists("batch_b").unwrap());
        assert!(!word_exists("batch_a").unwrap());

        // Re-enable batch operation
        let result = set_words_enabled_batch(&["batch_a", "batch_c"], true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);

        // Now 3 enabled
        assert_eq!(get_word_count().unwrap(), 3);
    }

    #[test]
    fn wordlist_set_word_enabled_nonexistent() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Try to disable a word that doesn't exist
        let result = set_word_enabled("nonexistent_word", false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // No rows updated
    }

    #[test]
    fn test_link_as_branch_updates_orphan() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        let mock_decoder = Decoder::<MockDecoder>::new();
        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        let crack_result = mock_decoder.crack("test", &checker);

        // Insert root entry
        let root_entry = CacheEntry {
            encoded_text: "root_encoded".to_string(),
            decoded_text: "root_decoded".to_string(),
            path: vec![crack_result.clone()],
            execution_time_ms: 10,
            input_length: 12,
            decoder_count: 1,
            checker_name: None,
            key_used: None,
        };
        let root_id = insert_cache(&root_entry).unwrap();

        // Insert orphan entry (no branch linkage)
        let orphan_entry = CacheEntry {
            encoded_text: "orphan_encoded".to_string(),
            decoded_text: "orphan_decoded".to_string(),
            path: vec![crack_result],
            execution_time_ms: 5,
            input_length: 14,
            decoder_count: 1,
            checker_name: None,
            key_used: None,
        };
        let orphan_id = insert_cache(&orphan_entry).unwrap();

        // Verify orphan has no parent
        let parent_info = get_parent_info(orphan_id).unwrap();
        assert!(
            parent_info.is_none(),
            "Orphan should have no parent initially"
        );

        // Link the orphan as a branch of root at step 0
        let rows = link_as_branch(orphan_id, root_id, 0, &BranchType::Auto).unwrap();
        assert_eq!(rows, 1, "Should update exactly 1 row");

        // Verify it now has parent linkage
        let parent_info = get_parent_info(orphan_id).unwrap();
        assert!(
            parent_info.is_some(),
            "Orphan should now have parent linkage"
        );
        let info = parent_info.unwrap();
        assert_eq!(info.parent_cache_id, root_id);
        assert_eq!(info.branch_step, 0);
    }

    #[test]
    fn test_link_as_branch_nonexistent_id() {
        set_test_db_path();
        let _conn = init_database().unwrap();

        // Try to link a nonexistent cache ID
        let rows = link_as_branch(99999, 1, 0, &BranchType::Auto).unwrap();
        assert_eq!(rows, 0, "Should update 0 rows for nonexistent ID");
    }
}
