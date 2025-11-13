//! Database migration system for schema versioning.
//!
//! This module handles migrating SQLite databases from older schema
//! versions to newer ones, ensuring smooth upgrades as the schema evolves.

use super::schema::{Schema, SCHEMA_VERSION};
use anyhow::Result;
use rusqlite::Connection;

/// Database migrator for handling schema upgrades.
pub struct Migrator;

impl Migrator {
    /// Run all necessary migrations to bring database up to current version.
    ///
    /// This method checks the current schema version and runs any missing
    /// migrations in order. It's safe to call multiple times - already
    /// applied migrations will be skipped.
    ///
    /// # Arguments
    /// * `conn` - Active database connection
    ///
    /// # Returns
    /// `Ok(())` on success, error if migration fails
    ///
    /// # Examples
    /// ```ignore
    /// let conn = Connection::open("use_cases.db")?;
    /// Migrator::migrate(&conn)?; // Brings DB to latest version
    /// ```
    pub fn migrate(conn: &Connection) -> Result<()> {
        let current_version = Self::current_version(conn)?;

        if current_version == 0 {
            // Fresh database - initialize with latest schema
            println!("🔨 Initializing database schema...");
            Schema::initialize(conn)?;
            println!("✅ Database schema initialized (v{})", SCHEMA_VERSION);
            return Ok(());
        }

        if current_version >= SCHEMA_VERSION {
            // Already up to date
            return Ok(());
        }

        println!(
            "🔄 Migrating database from v{} to v{}...",
            current_version, SCHEMA_VERSION
        );

        // Run migrations in order
        for version in (current_version + 1)..=SCHEMA_VERSION {
            Self::run_migration(conn, version)?;
            println!("   ✅ Migrated to v{}", version);
        }

        println!("✅ Database migration complete");
        Ok(())
    }

    /// Get current database schema version.
    ///
    /// Returns 0 if metadata table doesn't exist (fresh database).
    fn current_version(conn: &Connection) -> Result<i32> {
        // Check if metadata table exists
        let table_exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master 
             WHERE type='table' AND name='_metadata'",
            [],
            |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            },
        )?;

        if !table_exists {
            return Ok(0);
        }

        Schema::get_version(conn).or(Ok(0))
    }

    /// Run a specific migration version.
    ///
    /// # Arguments
    /// * `conn` - Active database connection
    /// * `version` - Target version number
    ///
    /// # Returns
    /// `Ok(())` on success, error if unknown version or migration fails
    fn run_migration(conn: &Connection, version: i32) -> Result<()> {
        match version {
            1 => Self::migrate_to_v1(conn),
            _ => anyhow::bail!("Unknown migration version: {}", version),
        }
    }

    /// Migration 1: Initial schema.
    ///
    /// This creates the initial database structure with all tables.
    /// For fresh databases, this is called via Schema::initialize.
    fn migrate_to_v1(conn: &Connection) -> Result<()> {
        // Migration 1 is the same as Schema::initialize
        Schema::initialize(conn)
    }

    // Future migrations will be added here as needed:
    //
    // fn migrate_to_v2(conn: &Connection) -> Result<()> {
    //     // Add new column, table, or index
    //     conn.execute("ALTER TABLE use_cases ADD COLUMN status TEXT DEFAULT 'draft'", [])?;
    //     Schema::set_schema_version(conn, 2)?;
    //     Ok(())
    // }
    //
    // fn migrate_to_v3(conn: &Connection) -> Result<()> {
    //     // Example: Add personas table
    //     conn.execute("CREATE TABLE personas (...)", [])?;
    //     Schema::set_schema_version(conn, 3)?;
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Connection {
        Connection::open_in_memory().unwrap()
    }

    #[test]
    fn test_migrate_fresh_database() {
        let conn = create_test_db();

        // Fresh database should initialize to latest version
        Migrator::migrate(&conn).unwrap();

        let version = Schema::get_version(&conn).unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn test_migrate_already_up_to_date() {
        let conn = create_test_db();

        // Initialize to current version
        Schema::initialize(&conn).unwrap();
        let version_before = Schema::get_version(&conn).unwrap();

        // Running migrate again should be no-op
        Migrator::migrate(&conn).unwrap();

        let version_after = Schema::get_version(&conn).unwrap();
        assert_eq!(version_before, version_after);
    }

    #[test]
    fn test_migrate_idempotent() {
        let conn = create_test_db();

        // Multiple migrations should be safe
        Migrator::migrate(&conn).unwrap();
        Migrator::migrate(&conn).unwrap();
        Migrator::migrate(&conn).unwrap();

        let version = Schema::get_version(&conn).unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn test_current_version_fresh_db() {
        let conn = create_test_db();
        let version = Migrator::current_version(&conn).unwrap();
        assert_eq!(version, 0);
    }

    #[test]
    fn test_current_version_initialized_db() {
        let conn = create_test_db();
        Schema::initialize(&conn).unwrap();
        let version = Migrator::current_version(&conn).unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn test_migration_creates_all_tables() {
        let conn = create_test_db();
        Migrator::migrate(&conn).unwrap();

        // Verify all expected tables exist
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
}
