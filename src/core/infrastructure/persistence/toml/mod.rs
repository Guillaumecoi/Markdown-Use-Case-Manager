//! TOML-based persistence implementation.
//!
//! This module provides TOML file-based storage for use cases and actors (personas and system actors).
//! Each entity is stored as a separate TOML file, making it
//! git-friendly and human-readable.

mod actor_repository;
mod repository;

pub use actor_repository::TomlActorRepository;
pub use repository::TomlUseCaseRepository;
