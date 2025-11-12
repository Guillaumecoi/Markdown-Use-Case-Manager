// Repository trait for use case persistence
use crate::core::domain::UseCase;
use anyhow::Result;

/// Repository abstraction for use case persistence
/// This trait defines the contract for storing and retrieving use cases
/// Implementations can use different storage backends (TOML, database, etc.)
pub trait UseCaseRepository {
    /// Save only the TOML file (source of truth)
    /// Use this when you want to save the use case data without generating markdown yet
    fn save_toml_only(&self, use_case: &UseCase) -> Result<()>;

    /// Save only the markdown file for an existing use case
    /// The use case must already exist in TOML
    fn save_markdown_only(&self, use_case_id: &str, markdown_content: &str) -> Result<()>;

    /// Load all use cases
    fn load_all(&self) -> Result<Vec<UseCase>>;

    /// Load a single use case by ID
    fn load_by_id(&self, id: &str) -> Result<Option<UseCase>>;

    /// Delete a use case
    /// TODO: Implement delete command: mucm delete UC-XXX-001
    #[allow(dead_code)]
    fn delete(&self, id: &str) -> Result<()>;

    /// Check if a use case exists
    /// TODO: Use this for validation before operations (update status, add scenario, etc.)
    #[allow(dead_code)]
    fn exists(&self, id: &str) -> Result<bool>;
}
