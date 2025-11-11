// Infrastructure layer - Implementation details

mod languages;
mod persistence;
mod template_engine;

// Re-exports
pub use languages::LanguageRegistry;
pub use persistence::{file_operations, TomlUseCaseRepository};
pub use template_engine::{to_snake_case, TemplateEngine};
