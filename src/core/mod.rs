// Core layer - Business logic and domain

// Private with explicit exports:
mod application;
mod domain;
mod infrastructure;
mod utils; // Internal only

// Explicit public exports from private modules
// Public exports - Explicit API surface
pub use application::UseCaseApplicationService;

// Re-export domain types (from domain's public interface)
pub use domain::{
    Actor, Metadata, Persona, PersonaRepository, Priority, ReferenceType, Scenario,
    ScenarioReference, ScenarioReferenceValidator, ScenarioStep, ScenarioType, Status, UseCase,
    UseCaseReference,
};

// Re-export infrastructure types (from infrastructure's public interface)
pub use infrastructure::{
    file_operations, CustomFieldConfig, LanguageRegistry, Methodology, MethodologyDefinition,
    MethodologyRegistry, RepositoryFactory, SqliteUseCaseRepository, TemplateEngine,
    TomlUseCaseRepository, UseCaseRepository,
};

// Re-export utility functions
pub use utils::to_snake_case;
