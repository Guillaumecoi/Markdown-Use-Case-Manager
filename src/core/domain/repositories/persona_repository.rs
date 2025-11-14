// Repository trait for persona persistence
use crate::core::domain::Persona;
use anyhow::Result;

/// Repository abstraction for persona persistence
/// This trait defines the contract for storing and retrieving personas
/// Implementations can use different storage backends (TOML, database, etc.)
pub trait PersonaRepository {
    /// Save a persona (data file only)
    fn save(&self, persona: &Persona) -> Result<()>;

    /// Load all personas
    fn load_all(&self) -> Result<Vec<Persona>>;

    /// Load a single persona by ID
    fn load_by_id(&self, id: &str) -> Result<Option<Persona>>;

    /// Delete a persona
    fn delete(&self, id: &str) -> Result<()>;

    /// Check if a persona exists
    fn exists(&self, id: &str) -> Result<bool>;

    /// Save persona markdown documentation
    fn save_markdown(&self, persona_id: &str, markdown_content: &str) -> Result<()>;
}
