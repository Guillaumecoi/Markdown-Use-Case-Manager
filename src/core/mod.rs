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
    Persona, ReferenceType, ScenarioReference, ScenarioReferenceValidator, ScenarioType, Status,
    UseCase,
};

// Exported for integration tests (appear unused to lib but required by tests/)
#[allow(unused_imports)]
pub use domain::Scenario;

// Re-export infrastructure types (from infrastructure's public interface)
pub use infrastructure::{
    file_operations, CustomFieldConfig, LanguageRegistry, Methodology, MethodologyDefinition,
    MethodologyRegistry, RepositoryFactory, TemplateEngine, UseCaseRepository,
};

// Exported for integration tests (appear unused to lib but required by tests/)
#[allow(unused_imports)]
pub use infrastructure::{SqliteUseCaseRepository, TomlUseCaseRepository};

// Re-export utility functions
pub use utils::to_snake_case;
