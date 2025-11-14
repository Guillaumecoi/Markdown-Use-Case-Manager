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

    /// Delete a use case by ID.
    ///
    /// # Arguments
    /// * `id` - The use case ID to delete
    ///
    /// # Returns
    /// `Ok(())` on success (even if ID doesn't exist), error on failure
    ///
    /// # Side Effects
    /// - TOML: Deletes TOML file and markdown file
    /// - SQLite: Deletes row (CASCADE deletes related data)
    fn delete(&self, id: &str) -> Result<()>;

    /// Check if a use case exists.
    ///
    /// # Arguments
    /// * `id` - The use case ID to check
    ///
    /// # Returns
    /// `true` if exists, `false` if not, error on failure
    fn exists(&self, id: &str) -> Result<bool>;

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

    // === Query Operations ===

    /// Find use cases by category.
    ///
    /// # Arguments
    /// * `category` - The category to filter by (exact match, case-sensitive)
    ///
    /// # Returns
    /// Vector of matching use cases
    ///
    /// # Performance
    /// - TOML: Loads all, filters in memory
    /// - SQLite: Indexed query (fast)
    fn find_by_category(&self, category: &str) -> Result<Vec<UseCase>>;

    /// Find use cases by priority.
    ///
    /// # Arguments
    /// * `priority` - The priority level (e.g., "high", "medium", "low")
    ///
    /// # Returns
    /// Vector of matching use cases (case-insensitive match)
    fn find_by_priority(&self, priority: &str) -> Result<Vec<UseCase>>;

    /// Search use cases by title.
    ///
    /// # Arguments
    /// * `query` - Search string (case-insensitive, partial match)
    ///
    /// # Returns
    /// Vector of use cases whose titles contain the query string
    ///
    /// # Examples
    /// - query "auth" matches "User Authentication", "Authorization Flow"
    fn search_by_title(&self, query: &str) -> Result<Vec<UseCase>>;

    // === Batch Operations ===

    /// Save multiple use cases in one operation.
    ///
    /// Implementations may optimize this for better performance than
    /// calling save() repeatedly.
    ///
    /// # Arguments
    /// * `use_cases` - Slice of use cases to save
    ///
    /// # Returns
    /// `Ok(())` on success, error on first failure
    ///
    /// # Atomicity
    /// - TOML: Not atomic (may partial save)
    /// - SQLite: Atomic transaction (all or nothing)
    fn save_batch(&self, use_cases: &[UseCase]) -> Result<()>;

    /// Delete multiple use cases.
    ///
    /// # Arguments
    /// * `ids` - Slice of use case IDs to delete
    ///
    /// # Returns
    /// `Ok(())` on success, error on first failure
    ///
    /// # Atomicity
    /// - TOML: Not atomic (may partial delete)
    /// - SQLite: Atomic transaction (all or nothing)
    fn delete_batch(&self, ids: &[&str]) -> Result<()>;

    // === Backend Metadata ===

    /// Get the backend type name.
    ///
    /// # Returns
    /// Static string identifying the backend: "toml" or "sqlite"
    fn backend_name(&self) -> &'static str;

    /// Check if the backend is available and healthy.
    ///
    /// # Returns
    /// `Ok(())` if healthy, error with diagnostic message if not
    ///
    /// # Checks
    /// - TOML: Directory exists and is writable
    /// - SQLite: Database connection is alive, schema is valid
    fn health_check(&self) -> Result<()>;
}

/// Transaction support for backends that support atomic operations.
///
/// This trait is optional and only implemented by backends that can
/// guarantee atomicity (e.g., SQLite). The `with_transaction` method ensures
/// that either all operations succeed (commit) or none do (rollback).
pub trait TransactionalRepository: UseCaseRepository {
    /// Execute operations within a transaction.
    ///
    /// # Arguments
    /// * `f` - Closure that performs operations using the repository
    ///
    /// # Returns
    /// Result from the closure, or error if transaction fails
    ///
    /// # Behavior
    /// - If closure returns `Ok`, transaction is committed
    /// - If closure returns `Err`, transaction is rolled back
    /// - Database is left unchanged if transaction fails
    fn with_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&dyn UseCaseRepository) -> Result<T>;
}
