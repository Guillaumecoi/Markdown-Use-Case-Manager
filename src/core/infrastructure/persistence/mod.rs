//! Persistence layer for use case storage.
//!
//! This module provides different storage backends (TOML, SQLite)
//! with a unified interface through the UseCaseRepository trait.

pub mod file_operations;
pub mod sqlite;
pub mod toml;

// Re-export for convenience
pub use toml::TomlUseCaseRepository;
