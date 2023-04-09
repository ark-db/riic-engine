use parking_lot::Mutex;
use rusqlite::{config::DbConfig, limits::Limit, Connection, Error};

pub struct Database(Mutex<Connection>);

impl Database {
    /// # Errors
    /// Returns error if:
    /// - A connection to the database cannot be opened
    /// - Database settings and limits cannot be set
    /// - SQL statements executed during initialization fail
    /// - SQL statements cannot be prepared and cached
    pub fn init() -> Result<Self, Error> {
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
