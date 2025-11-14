//! TOML-based persistence implementation.
//!
//! This module provides TOML file-based storage for use cases and personas.
//! Each entity is stored as a separate TOML file, making it
//! git-friendly and human-readable.

mod persona_repository;
mod repository;

pub use persona_repository::TomlPersonaRepository;
pub use repository::TomlUseCaseRepository;
