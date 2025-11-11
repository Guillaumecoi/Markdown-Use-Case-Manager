// Core layer - Business logic and domain

// Private with explicit exports:
mod application;     // Exports: UseCaseApplicationService
mod domain;          // Exports: entities, repositories, services
mod infrastructure;  // Exports: languages, persistence, template_engine
mod processors;      // Exports: MethodologyManager
mod utils;           // Internal only

// Explicit public exports from private modules
// Public exports - Explicit API surface
pub use processors::MethodologyManager;
pub use application::UseCaseApplicationService;

// Re-export domain types (from domain's public interface)
pub use domain::{Metadata, Priority, Status, UseCase, UseCaseRepository, UseCaseService};

// Re-export infrastructure types (from infrastructure's public interface)
pub use infrastructure::{file_operations, to_snake_case, LanguageRegistry, TemplateEngine, TomlUseCaseRepository};
