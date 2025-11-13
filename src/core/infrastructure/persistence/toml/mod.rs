//! TOML-based persistence implementation.
//!
//! This module provides TOML file-based storage for use cases.
//! Each use case is stored as a separate TOML file, making it
//! git-friendly and human-readable.

mod repository;

pub use repository::TomlUseCaseRepository;
