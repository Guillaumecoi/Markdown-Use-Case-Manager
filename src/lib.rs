// Internal test interface - NOT FOR PUBLIC USE
// This exists solely to support the extensive test suite
// All public functionality is accessed through the `mucm` CLI binary

#![doc(hidden)]
#![allow(dead_code)]

// Internal test interface - NOT FOR PUBLIC USE
// This exists solely to support the extensive test suite
// All public functionality is accessed through the `mucm` CLI binary

// Private modules
mod config;
mod controller;
mod core;
mod presentation;

// Explicit public exports for test interface
pub use config::{Config, ConfigFileManager, MethodologyConfig, TemplateManager};
pub use controller::{ProjectController, UseCaseController};
pub use core::application::UseCaseApplicationService;
pub use core::domain::entities::{Metadata, Priority, Status, UseCase};
pub use core::domain::repositories::UseCaseRepository;
pub use core::domain::services::UseCaseService;
pub use core::infrastructure::languages::LanguageRegistry;
pub use core::infrastructure::persistence::TomlUseCaseRepository;
pub use core::processors::MethodologyManager;
pub use presentation::{StatusFormatter, UseCaseFormatter};
