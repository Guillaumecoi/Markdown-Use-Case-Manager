//! SQLite-based persistence implementation.
//!
//! This module provides SQLite database storage for use cases and actors (personas and system actors).
//! Offers better querying capabilities for larger projects.

pub mod actor_repository;
pub mod migrations;
pub mod repository;
pub mod schema;

pub use actor_repository::SqliteActorRepository;
pub use migrations::Migrator;
pub use repository::SqliteUseCaseRepository;
