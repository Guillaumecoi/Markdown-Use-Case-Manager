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
pub use domain::{Status, UseCase, UseCaseService};

// Re-export infrastructure types (from infrastructure's public interface)
pub use infrastructure::{
    file_operations, LanguageRegistry, Methodology, MethodologyDefinition, MethodologyRegistry,
    TemplateEngine, TomlUseCaseRepository, TransactionalRepository, UseCaseRepository,
};

// Re-export utility functions
pub use utils::to_snake_case;
