// Infrastructure layer - Implementation details

mod languages;
mod methodologies;
mod persistence;
mod template_engine;

// Re-exports
pub use languages::LanguageRegistry;
pub use methodologies::{Methodology, MethodologyDefinition, MethodologyRegistry};
pub use persistence::{
    file_operations, RepositoryFactory, TomlUseCaseRepository, TransactionalRepository, UseCaseRepository,
};
pub use persistence::sqlite::SqliteUseCaseRepository;
pub use template_engine::TemplateEngine;
