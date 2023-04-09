use crate::base::Save;
use ahash::HashSet;
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use rusqlite::{config::DbConfig, limits::Limit, Connection, Error as SqlError};
use serde::Serialize;
use std::borrow::Cow;
use tauri::{InvokeError, State};
use thiserror::Error;

pub struct Database(Mutex<Connection>);

impl Database {
    /// # Errors
    /// Returns error if:
    /// - A connection to the database cannot be opened
    /// - Database settings and limits cannot be set
    /// - SQL statements executed during initialization fail
    /// - SQL statements cannot be prepared and cached
    pub fn init() -> Result<Self, SqlError> {
        let conn = Connection::open("./data.db")?;

        // Set configuration
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, false)?;
        conn.set_limit(Limit::SQLITE_LIMIT_LENGTH, 1_000_000);
        conn.set_limit(Limit::SQLITE_LIMIT_SQL_LENGTH, 1_000_000);

        // Initialize database
        conn.execute_batch(
            "BEGIN;
            CREATE TABLE IF NOT EXISTS save (
                name            TEXT NOT NULL UNIQUE,
                created         REAL NOT NULL,
                last_modified   REAL NOT NULL,
                data            BLOB NOT NULL
            ) STRICT;
            CREATE INDEX idx_name_data ON save (
                name, data
            );
            COMMIT;",
        )?;

        // Check taken names
        conn.prepare_cached("SELECT name FROM save")?;
        // Fetch all
        conn.prepare_cached("SELECT name, created, last_modified FROM save")?;
        // Create
        conn.prepare_cached(
            "INSERT INTO save (
                name, created, last_modified, data
            ) VALUES (
                ?1, ?2, ?3, ?4
            )",
        )?;
        // Get
        conn.prepare_cached("SELECT data FROM save WHERE name = ?1")?;
        // Rename
        conn.prepare_cached("UPDATE save SET name = ?2 WHERE name = ?1")?;
        // Update
        conn.prepare_cached("UPDATE save SET data = ?2, last_modified = ?3 WHERE name = ?1")?;
        // Delete
        conn.prepare_cached("DELETE FROM save WHERE name = ?1")?;

        Ok(Self(Mutex::from(conn)))
    }
}

#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum DbError {
    #[error("Failed to execute query statement")]
    Execution,

    #[error("An error occurred while fetching saves")]
    Fetch,

    #[error("An error occurred while creating a new save")]
    Creation,
}

type DbResult<T> = Result<T, DbError>;

impl From<DbError> for InvokeError {
    fn from(val: DbError) -> Self {
        InvokeError::from(val.to_string())
    }
}

#[derive(Serialize)]
pub struct FileData {
    name: String,
    created: f32,
    modified: f32,
}

fn get_elapsed_time(earlier: DateTime<Utc>, later: DateTime<Utc>) -> f32 {
    later
        .signed_duration_since(earlier)
        .to_std()
        .expect("`earlier` datetime should be earlier than the `later` datetime")
        .as_secs_f32()
}

/// # Errors
/// Returns error if:
/// - Database query failed
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_saves(db: State<'_, Database>) -> DbResult<Vec<FileData>> {
    let conn = db.0.lock();

    let now = Utc::now();

    let query = conn
        .prepare_cached("SELECT name, created, last_modified FROM save")
        .map_err(|_| DbError::Execution)?
        .query_and_then([], |row| {
            Ok(FileData {
                name: row.get("name")?,
                created: get_elapsed_time(row.get("created")?, now),
                modified: get_elapsed_time(row.get("last_modified")?, now),
            })
        })
        .map_err(|_| DbError::Fetch)?
        .collect::<Result<Vec<FileData>, SqlError>>()
        .map_err(|_| DbError::Fetch)?;

    Ok(query)
}

fn get_available_name<F>(name: &str, is_available: F) -> Cow<'_, str>
where
    F: Fn(&str) -> bool,
{
    if is_available(name) {
        return Cow::Borrowed(name);
    }

    let mut i = 1u32;
    let mut new_name: String;
    while {
        new_name = format!("{name}-{i}");
        !is_available(&new_name)
    } {
        i += 1;
    }
    Cow::Owned(new_name)
}

/// # Errors
/// Returns error if:
/// - Database query failed
/// - Database insertion failed
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn create_save(db: State<'_, Database>) -> DbResult<()> {
    let conn = db.0.lock();

    let names = conn
        .prepare_cached("SELECT name FROM save")
        .map_err(|_| DbError::Execution)?
        .query_and_then([], |row| row.get("name"))
        .map_err(|_| DbError::Fetch)?
        .collect::<Result<HashSet<String>, SqlError>>()
        .map_err(|_| DbError::Fetch)?;

    let save_name = get_available_name("Untitled", |new_name| !names.contains(new_name));

    let now = Utc::now();

    conn.prepare_cached(
        "INSERT INTO save (
            name, created, last_modified, data
        ) VALUES (
            ?1, ?2, ?3, ?4
        )",
    )
    .map_err(|_| DbError::Execution)?
    .execute((save_name, now, now, Save::default()))
    .map_err(|_| DbError::Creation)?;

    Ok(())
}
