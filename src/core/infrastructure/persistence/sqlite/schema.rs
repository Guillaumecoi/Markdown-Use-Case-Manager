//! SQLite database schema definitions.
//!
//! This module defines the complete database schema for storing use cases
//! in SQLite, including all tables, indexes, and version management.

use anyhow::Result;
use rusqlite::Connection;

/// Current schema version.
///
/// Increment this when making schema changes and add corresponding
/// migration in migrations.rs.
pub const SCHEMA_VERSION: i32 = 1;

/// Schema manager for creating and validating database structure.
pub struct Schema;

impl Schema {
    /// Initialize database with all tables.
    ///
    /// Creates the complete database schema including:
    /// - Metadata table for version tracking
    /// - Use cases table with core fields
    /// - Preconditions, postconditions, and references tables
    /// - All necessary indexes for query performance
    ///
    /// # Arguments
    /// * `conn` - Active database connection
    ///
    /// # Returns
    /// `Ok(())` on success, error if table creation fails
    pub fn initialize(conn: &Connection) -> Result<()> {
        Self::create_metadata_table(conn)?;
        Self::create_use_cases_table(conn)?;
        Self::create_use_case_preconditions_table(conn)?;
        Self::create_use_case_postconditions_table(conn)?;
        Self::create_use_case_references_table(conn)?;
        Self::set_schema_version(conn, SCHEMA_VERSION)?;
        Ok(())
    }

    /// Create metadata table for schema versioning.
    fn create_metadata_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS _metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// Create main use cases table with indexes.
    fn create_use_cases_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS use_cases (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT,
                priority TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                extra_json TEXT NOT NULL DEFAULT '{}'
            )",
            [],
        )?;

        // Indexes for common queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_use_cases_category 
             ON use_cases(category)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_use_cases_priority 
             ON use_cases(priority)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_use_cases_title 
             ON use_cases(title COLLATE NOCASE)",
            [],
        )?;

        Ok(())
    }

    /// Create preconditions table with foreign key.
    fn create_use_case_preconditions_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS use_case_preconditions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                use_case_id TEXT NOT NULL,
                condition_order INTEGER NOT NULL,
                condition_text TEXT NOT NULL,
                FOREIGN KEY (use_case_id) REFERENCES use_cases(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_preconditions_use_case 
             ON use_case_preconditions(use_case_id)",
            [],
        )?;

        Ok(())
    }

    /// Create postconditions table with foreign key.
    fn create_use_case_postconditions_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS use_case_postconditions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                use_case_id TEXT NOT NULL,
                condition_order INTEGER NOT NULL,
                condition_text TEXT NOT NULL,
                FOREIGN KEY (use_case_id) REFERENCES use_cases(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_postconditions_use_case 
             ON use_case_postconditions(use_case_id)",
            [],
        )?;

        Ok(())
    }

    /// Create use case references table with foreign keys.
    fn create_use_case_references_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS use_case_references (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                use_case_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relationship TEXT NOT NULL,
                description TEXT,
                FOREIGN KEY (use_case_id) REFERENCES use_cases(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_references_use_case 
             ON use_case_references(use_case_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_references_target 
             ON use_case_references(target_id)",
            [],
        )?;

        Ok(())
    }

    /// Set the schema version in metadata table.
    fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO _metadata (key, value, updated_at)
             VALUES ('schema_version', ?1, datetime('now'))",
            [version.to_string()],
        )?;
        Ok(())
    }

    /// Get current schema version from database.
    ///
    /// # Returns
    /// Current version number, or error if metadata doesn't exist
    pub fn get_version(conn: &Connection) -> Result<i32> {
        let version: String = conn.query_row(
            "SELECT value FROM _metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )?;

        Ok(version.parse()?)
    }

    /// Check if database needs migration.
    ///
    /// # Returns
    /// `true` if migration needed, `false` if up to date
    pub fn needs_migration(conn: &Connection) -> Result<bool> {
        match Self::get_version(conn) {
            Ok(version) => Ok(version < SCHEMA_VERSION),
            Err(_) => Ok(true), // No version means needs init
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Connection {
        Connection::open_in_memory().unwrap()
    }

    #[test]
    fn test_schema_initialize() {
        let conn = create_test_db();
        Schema::initialize(&conn).unwrap();

        // Verify all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"_metadata".to_string()));
        assert!(tables.contains(&"use_cases".to_string()));
        assert!(tables.contains(&"use_case_preconditions".to_string()));
        assert!(tables.contains(&"use_case_postconditions".to_string()));
        assert!(tables.contains(&"use_case_references".to_string()));
    }

    #[test]
    fn test_schema_version() {
        let conn = create_test_db();
        Schema::initialize(&conn).unwrap();

        let version = Schema::get_version(&conn).unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn test_needs_migration_fresh_db() {
        let conn = create_test_db();
        assert!(Schema::needs_migration(&conn).unwrap());
    }

    #[test]
    fn test_needs_migration_up_to_date() {
        let conn = create_test_db();
        Schema::initialize(&conn).unwrap();
        assert!(!Schema::needs_migration(&conn).unwrap());
    }

    #[test]
    fn test_foreign_keys_enabled() {
        let conn = create_test_db();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        Schema::initialize(&conn).unwrap();

        // Insert a use case
        conn.execute(
            "INSERT INTO use_cases (id, title, category, priority, created_at, updated_at)
             VALUES ('UC-TEST-001', 'Test', 'test', 'medium', datetime('now'), datetime('now'))",
            [],
        )
        .unwrap();

        // Insert a precondition
        conn.execute(
            "INSERT INTO use_case_preconditions (use_case_id, condition_order, condition_text)
             VALUES ('UC-TEST-001', 0, 'Test condition')",
            [],
        )
        .unwrap();

        // Delete the use case
        conn.execute("DELETE FROM use_cases WHERE id = 'UC-TEST-001'", [])
            .unwrap();

        // Precondition should be automatically deleted (CASCADE)
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM use_case_preconditions WHERE use_case_id = 'UC-TEST-001'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(
            count, 0,
            "Foreign key CASCADE should have deleted precondition"
        );
    }

    #[test]
    fn test_indexes_created() {
        let conn = create_test_db();
        Schema::initialize(&conn).unwrap();

        let indexes: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        // Should have indexes for common queries
        assert!(indexes.iter().any(|idx| idx.contains("category")));
        assert!(indexes.iter().any(|idx| idx.contains("priority")));
        assert!(indexes.iter().any(|idx| idx.contains("title")));
    }
}
