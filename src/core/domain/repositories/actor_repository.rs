// Repository trait for actor persistence (personas and system actors)
use crate::core::domain::{ActorEntity, Persona};
use anyhow::Result;

/// Repository abstraction for actor persistence
/// This trait defines the contract for storing and retrieving actors (personas and system actors)
/// Implementations can use different storage backends (TOML, database, etc.)
pub trait ActorRepository {
    // === ActorEntity operations (new unified actor system) ===

    /// Save an actor (data file only)
    fn save_actor(&self, actor: &ActorEntity) -> Result<()>;

    /// Load all actors
    fn load_all_actors(&self) -> Result<Vec<ActorEntity>>;

    /// Load a single actor by ID
    fn load_actor_by_id(&self, id: &str) -> Result<Option<ActorEntity>>;

    /// Delete an actor
    fn delete_actor(&self, id: &str) -> Result<()>;

    /// Check if an actor exists
    fn actor_exists(&self, id: &str) -> Result<bool>;

    /// Save actor markdown documentation
    fn save_actor_markdown(&self, actor_id: &str, markdown_content: &str) -> Result<()>;

    // === Persona compatibility operations (backward compatibility) ===

    /// Save a persona (backward compatibility wrapper)
    /// Converts Persona to ActorEntity and delegates to save_actor
    fn save_persona(&self, persona: &Persona) -> Result<()>;

    /// Load all personas (backward compatibility)
    /// Returns only actors with ActorType::Persona
    fn load_all_personas(&self) -> Result<Vec<Persona>>;

    /// Load a single persona by ID (backward compatibility)
    fn load_persona_by_id(&self, id: &str) -> Result<Option<Persona>>;

    /// Delete a persona (backward compatibility wrapper)
    fn delete_persona(&self, id: &str) -> Result<()>;

    /// Check if a persona exists (backward compatibility)
    fn persona_exists(&self, id: &str) -> Result<bool>;

    /// Save persona markdown documentation (backward compatibility)
    fn save_persona_markdown(&self, persona_id: &str, markdown_content: &str) -> Result<()>;
}
