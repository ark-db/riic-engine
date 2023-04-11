use crate::base::Save;
use ahash::HashSet;
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use rusqlite::{config::DbConfig, limits::Limit, Connection, Error as SqlError, ErrorCode};
use serde::Serialize;
use std::{borrow::Cow, fs::File, io::BufWriter, ops::Deref};
use tauri::{api::path::download_dir, utils::platform::current_exe, InvokeError, State};
use thiserror::Error;

pub struct Database(Mutex<Connection>);

impl Database {
    /// # Errors
    /// Returns error if:
    /// - A connection to the database cannot be opened
    /// - Database configuration cannot be set
    /// - SQL statements executed during initialization fail
    /// - SQL statements cannot be prepared and cached
    pub fn init() -> Result<Self, SqlError> {
        let db_path = current_exe()
            .expect("Failed to get the currently-running binary path")
            .with_file_name("data.db");
        let conn = Connection::open(db_path)?;

        // Set configuration
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, false)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER, false)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_VIEW, false)?;
        conn.set_limit(Limit::SQLITE_LIMIT_LENGTH, 1_000_000);
        conn.set_limit(Limit::SQLITE_LIMIT_SQL_LENGTH, 1_000_000);

        // Initialize database
        conn.execute_batch(
            "BEGIN;
            CREATE TABLE IF NOT EXISTS save (
                name            TEXT NOT NULL UNIQUE,
                created         TEXT NOT NULL,
                last_modified   TEXT NOT NULL,
                data            BLOB NOT NULL
            ) STRICT;
            CREATE INDEX IF NOT EXISTS idx_name_data ON save (
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
        conn.prepare_cached("UPDATE save SET last_modified = ?2, data = ?3 WHERE name = ?1")?;
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
    Fetching,

    #[error("An error occurred while creating a new save")]
    Creation,

    #[error("An error occurred while renaming the save")]
    Renaming,

    #[error("Another save with the same name already exists")]
    DuplicateName,

    #[error("An error occurred while updating the save")]
    Update,

    #[error("An error occurred while deleting the save")]
    Deletion,

    #[error("The download directory could not be located")]
    NoExportTarget,

    #[error("An error occurred while exporting the save")]
    Export,
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

#[allow(clippy::cast_precision_loss)]
fn get_elapsed_time(earlier: DateTime<Utc>, later: DateTime<Utc>) -> f32 {
    (later - earlier).num_milliseconds() as f32 / 1000.
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

fn get_save_from_name<C>(conn: C, name: &str) -> DbResult<Save>
where
    C: Deref<Target = Connection>,
{
    conn.prepare_cached("SELECT data FROM save WHERE name = ?1")
        .map_err(|_| DbError::Execution)?
        .query_row([name], |row| row.get("data"))
        .map_err(|_| DbError::Fetching)
}

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
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
        .map_err(|_| DbError::Fetching)?
        .collect::<Result<Vec<FileData>, SqlError>>()
        .map_err(|_| DbError::Fetching)?;

    Ok(query)
}

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
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
        .map_err(|_| DbError::Fetching)?
        .collect::<Result<HashSet<String>, SqlError>>()
        .map_err(|_| DbError::Fetching)?;

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

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
/// - Database query failed
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_save(db: State<'_, Database>, name: &str) -> DbResult<Save> {
    let conn = db.0.lock();
    get_save_from_name(conn, name)
}

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
/// - Database update failed
/// - A save with name `new` already exists
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn rename_save(db: State<'_, Database>, old: &str, new: &str) -> DbResult<()> {
    let conn = db.0.lock();

    conn.prepare_cached("UPDATE save SET name = ?2 WHERE name = ?1")
        .map_err(|_| DbError::Execution)?
        .execute([old, new])
        .map_err(|e| {
            if let Some(code) = e.sqlite_error_code() {
                if code == ErrorCode::ConstraintViolation {
                    return DbError::DuplicateName;
                }
            }
            DbError::Renaming
        })?;

    Ok(())
}

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
/// - Database update failed
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn update_save(db: State<'_, Database>, name: &str, save: Save) -> DbResult<()> {
    let conn = db.0.lock();

    conn.prepare_cached("UPDATE save SET last_modified = ?2, data = ?3 WHERE name = ?1")
        .map_err(|_| DbError::Execution)?
        .execute((name, Utc::now(), save))
        .map_err(|_| DbError::Update)?;

    Ok(())
}

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
/// - Database update failed
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn delete_save(db: State<'_, Database>, name: &str) -> DbResult<()> {
    let conn = db.0.lock();

    conn.prepare_cached("DELETE FROM save WHERE name = ?1")
        .map_err(|_| DbError::Execution)?
        .execute([name])
        .map_err(|_| DbError::Deletion)?;

    Ok(())
}

#[derive(Serialize)]
struct NamedSave<'cmd> {
    name: &'cmd str,
    data: Save,
}

/// # Errors
/// Returns error if:
/// - Invalid SQL statement is present
/// - Database update failed
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn export_save(db: State<'_, Database>, name: &str) -> DbResult<()> {
    let target_dir = if let Some(dir) = download_dir() {
        dir
    } else {
        return Err(DbError::NoExportTarget);
    };
    let target_name = get_available_name("export", |new_name| {
        !target_dir.join(new_name).with_extension("json").is_file()
    });
    let target_file = BufWriter::new(
        File::create(target_dir.join(target_name.as_ref()).with_extension("json"))
            .map_err(|_| DbError::Export)?,
    );

    let conn = db.0.lock();
    let save = NamedSave {
        name,
        data: get_save_from_name(conn, name)?,
    };

    serde_json::to_writer(target_file, &save).map_err(|_| DbError::Export)
}
