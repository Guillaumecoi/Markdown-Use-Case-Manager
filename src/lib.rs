// Internal test interface - NOT FOR PUBLIC USE
// This exists solely to support the extensive test suite
// All public functionality is accessed through the `mucm` CLI binary

#![doc(hidden)]
#![allow(dead_code)]

pub mod config;
pub mod controller;
pub mod core;
pub mod presentation; // Formatters and display logic

// Re-export commonly used types for convenience
pub use core::application::UseCaseApplicationService;
pub use core::domain::entities::{Metadata, Priority, Status, UseCase};
