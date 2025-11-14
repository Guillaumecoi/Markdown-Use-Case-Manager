//! SQLite-based persistence implementation.
//!
//! This module provides SQLite database storage for use cases and personas.
//! Offers better querying capabilities for larger projects.

pub mod migrations;
pub mod persona_repository;
pub mod repository;
pub mod schema;

pub use migrations::Migrator;
pub use persona_repository::SqlitePersonaRepository;
pub use repository::SqliteUseCaseRepository;
pub use schema::Schema;
