//! # Methodology Infrastructure
//!
//! This module provides a modular system for defining and managing methodologies
//! supported by the Markdown Use Case Manager. Methodologies are defined externally in TOML
//! configuration files, making it easy to add new methodologies without modifying code.
//!
//! ## Architecture
//!
//! - **Methodology Trait**: Defines the interface that all methodology implementations must provide
//! - **MethodologyDefinition**: The concrete implementation loaded from TOML files
//! - **MethodologyRegistry**: Manages a collection of loaded methodologies and provides lookup
//!
//! ## Methodology Configuration
//!
//! Each methodology is defined in a single `methodology.toml` file in `source-templates/methodologies/{name}/`.
//! This file contains all methodology information:
//!
//! - Methodology metadata (name, abbreviation, description)
//! - Template configuration (preferred style, generation options)
//! - Documentation levels with custom fields
//! - Usage guidance (when to use, key features, best practices)
//!
//! The methodology directory also contains Handlebars templates for different documentation styles.

mod definition;
mod field_resolver;
mod registry;
mod r#trait;
mod types;

// Re-export public types
pub use definition::MethodologyDefinition;
// FieldResolver will be exported when used by application service
// pub use field_resolver::FieldResolver;
pub use r#trait::{DocumentationLevel, Methodology};
pub use registry::MethodologyRegistry;
pub use types::CustomFieldConfig;
