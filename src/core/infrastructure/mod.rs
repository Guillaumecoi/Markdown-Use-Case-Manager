// Infrastructure layer - Implementation details

mod languages;
mod methodologies;
mod persistence;
mod template_engine;

// Re-exports
pub use languages::LanguageRegistry;
pub use methodologies::{
    CustomFieldConfig, Methodology, MethodologyDefinition, MethodologyRegistry,
};
pub use persistence::sqlite::SqliteUseCaseRepository;
pub use persistence::{
    file_operations, RepositoryFactory, TomlUseCaseRepository, TransactionalRepository,
    UseCaseRepository,
};
pub use template_engine::TemplateEngine;
