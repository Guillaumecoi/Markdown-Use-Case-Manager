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
pub use core::UseCaseApplicationService;
pub use core::{Metadata, Priority, Status, UseCase};
pub use core::{UseCaseRepository, UseCaseService};
pub use core::{LanguageRegistry, TomlUseCaseRepository};
pub use core::MethodologyManager;
pub use presentation::{StatusFormatter, UseCaseFormatter};
