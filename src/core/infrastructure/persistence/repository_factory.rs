//! Repository Factory
//!
//! This module provides a factory for creating use case repositories based on
//! configuration settings. It supports both TOML and SQLite backends with
//! identical interfaces through the UseCaseRepository trait.

use crate::config::{Config, StorageBackend};
use crate::core::infrastructure::persistence::traits::UseCaseRepository;
use crate::core::infrastructure::persistence::toml::TomlUseCaseRepository;
use crate::core::infrastructure::persistence::sqlite::SqliteUseCaseRepository;
use anyhow::{Context, Result};

/// Repository factory for creating use case repositories based on configuration
pub struct RepositoryFactory;

impl RepositoryFactory {
    /// Create a repository based on the provided configuration
    ///
    /// # Arguments
    /// * `config` - The application configuration containing storage backend settings
    ///
    /// # Returns
    /// A boxed trait object implementing UseCaseRepository, or an error if creation fails
    ///
    /// # Examples
    /// ```ignore
    /// let config = Config::load()?;
    /// let repository = RepositoryFactory::create(&config)?;
    /// assert_eq!(repository.backend_name(), "toml"); // or "sqlite"
    /// ```
    pub fn create(config: &Config) -> Result<Box<dyn UseCaseRepository>> {
        match config.storage.backend {
            StorageBackend::Toml => {
                let repo = TomlUseCaseRepository::new(config.clone());
                Ok(Box::new(repo))
            }
            StorageBackend::Sqlite => {
                // For SQLite, we need to determine the database path
                // Use a default path in the project directory
                let db_path = std::path::Path::new(".config").join("mucm").join("usecases.db");

                // Create parent directories if they don't exist
                if let Some(parent) = db_path.parent() {
                    std::fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create database directory {:?}", parent))?;
                }

                let repo = SqliteUseCaseRepository::new(&db_path)?;
                Ok(Box::new(repo))
            }
        }
    }

    /// Create a repository with a custom database path (SQLite only)
    ///
    /// This is useful for testing or when you want to specify a custom database location.
    ///
    /// # Arguments
    /// * `config` - The application configuration
    /// * `db_path` - Custom path for the SQLite database (ignored for TOML backend)
    ///
    /// # Returns
    /// A boxed trait object implementing UseCaseRepository
    pub fn create_with_db_path<P: AsRef<std::path::Path>>(
        config: &Config,
        db_path: P,
    ) -> Result<Box<dyn UseCaseRepository>> {
        match config.storage.backend {
            StorageBackend::Toml => {
                let repo = TomlUseCaseRepository::new(config.clone());
                Ok(Box::new(repo))
            }
            StorageBackend::Sqlite => {
                let repo = SqliteUseCaseRepository::new(db_path)?;
                Ok(Box::new(repo))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StorageBackend;
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_create_toml_repository() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        // Create a minimal config for testing
        let mut config = Config::default();
        config.storage.backend = StorageBackend::Toml;

        let repository = RepositoryFactory::create(&config)?;
        assert_eq!(repository.backend_name(), "toml");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_sqlite_repository() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        // Create a minimal config for testing
        let mut config = Config::default();
        config.storage.backend = StorageBackend::Sqlite;

        let repository = RepositoryFactory::create(&config)?;
        assert_eq!(repository.backend_name(), "sqlite");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_with_custom_db_path() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        // Create a minimal config for testing
        let mut config = Config::default();
        config.storage.backend = StorageBackend::Sqlite;

        let custom_db_path = temp_dir.path().join("custom.db");
        let repository = RepositoryFactory::create_with_db_path(&config, &custom_db_path)?;
        assert_eq!(repository.backend_name(), "sqlite");

        // Verify the database file was created
        assert!(custom_db_path.exists());

        Ok(())
    }
}