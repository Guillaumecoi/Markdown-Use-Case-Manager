// Internal test interface - NOT FOR PUBLIC USE
// This exists solely to support the extensive test suite
// All public functionality is accessed through the `mucm` CLI binary

#![doc(hidden)]
#![allow(dead_code)]

pub mod config;
pub mod core;

pub use core::use_case_coordinator::UseCaseCoordinator;
pub use core::models::Priority;
