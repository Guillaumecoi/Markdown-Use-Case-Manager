//! SQLite-based persistence implementation.
//!
//! This module provides SQLite database storage for use cases.
//! Offers better querying capabilities for larger projects.

pub mod migrations;
pub mod schema;

pub use migrations::Migrator;
pub use schema::Schema;
