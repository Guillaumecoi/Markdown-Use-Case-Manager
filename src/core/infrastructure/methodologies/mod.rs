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
//! Each methodology is defined in two TOML files in `source-templates/methodologies/{name}/`:
//!
//! - `info.toml`: Contains user-facing information for methodology selection and usage guidance
//! - `config.toml`: Contains technical configuration and template settings
//!
//! The `info.toml` file provides detailed descriptions and usage information to help users
//! choose the appropriate methodology, while `config.toml` contains the operational settings.
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
