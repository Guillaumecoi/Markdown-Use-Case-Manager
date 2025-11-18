//! Common traits for persistence layer.
//!
//! This module defines the unified interface that all storage backends
//! (TOML, SQLite, etc.) must implement to ensure consistency and
//! interchangeability.

use crate::core::domain::UseCase;
use anyhow::Result;

/// Repository abstraction for use case persistence.
///
/// Both TOML and SQLite implementations must support all methods
/// to ensure feature parity across backends.
///
/// # Design Philosophy
///
/// - **Backend Agnostic**: Application code should work with any backend
/// - **Feature Complete**: All backends support the same operations
/// - **Query Support**: Both backends support filtering and searching
/// - **Batch Operations**: Efficient bulk operations when possible
///
/// # Implementations
///
/// - `TomlUseCaseRepository`: File-based, git-friendly storage
/// - `SqliteUseCaseRepository`: Database storage with advanced querying
pub trait UseCaseRepository {
    // === Basic CRUD Operations ===

    /// Save a use case (insert if new, update if exists).
    ///
    /// # Arguments
    /// * `use_case` - The use case to save
    ///
    /// # Returns
    /// `Ok(())` on success, error if save fails
    fn save(&self, use_case: &UseCase) -> Result<()>;

    /// Load all use cases from storage.
    ///
    /// # Returns
    /// Vector of all use cases, or error if loading fails
    ///
    /// # Performance
    /// - TOML: Reads all files from disk
    /// - SQLite: Single query with JOINs
    fn load_all(&self) -> Result<Vec<UseCase>>;

    /// Load a single use case by ID.
    ///
    /// # Arguments
    /// * `id` - The use case ID (e.g., "UC-AUTH-001")
    ///
    /// # Returns
    /// `Some(UseCase)` if found, `None` if not found, error on failure
    fn load_by_id(&self, id: &str) -> Result<Option<UseCase>>;

    // === Markdown Generation ===

    /// Save generated markdown for a use case.
    ///
    /// This is separate from source data (TOML/SQLite). Markdown is always
    /// saved to files for documentation purposes, regardless of backend.
    ///
    /// # Arguments
    /// * `use_case_id` - The use case ID
    /// * `content` - The markdown content to save
    ///
    /// # Returns
    /// `Ok(())` on success, error if write fails
    fn save_markdown(&self, use_case_id: &str, content: &str) -> Result<()>;

    /// Save markdown file with a specific filename.
    ///
    /// Used for multi-view use cases where each view needs its own file
    /// (e.g., UC-001-feat-s.md, UC-001-bus-n.md).
    ///
    /// # Arguments
    /// * `use_case` - The use case (for category/path resolution)
    /// * `filename` - The full filename (e.g., "UC-001-feat-s.md")
    /// * `content` - The markdown content to save
    ///
    /// # Returns
    /// `Ok(())` on success, error if write fails
    fn save_markdown_with_filename(
        &self,
        use_case: &UseCase,
        filename: &str,
        content: &str,
    ) -> Result<()>;
}
