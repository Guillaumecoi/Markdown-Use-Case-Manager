//! # Language Infrastructure
//!
//! This module provides a modular system for defining and managing programming languages
//! supported by the Markdown Use Case Manager. Languages are defined externally in TOML
//! configuration files, making it easy to add new languages without modifying code.
//!
//! ## Architecture
//!
//! - **Language Trait**: Defines the interface that all language implementations must provide
//! - **LanguageDefinition**: The concrete implementation loaded from TOML files
//! - **LanguageRegistry**: Manages a collection of loaded languages and provides lookup
//!
//! ## Language Configuration
//!
//! Each language is defined in `source-templates/languages/{name}/info.toml`:
//!
//! ```toml
//! name = "rust"
//! aliases = ["rs"]
//! file_extension = "rs"
//! template_file = "test.hbs"
//! ```
//!
//! The `template_file` references a Handlebars template in the same directory that
//! contains the test generation template for that language.

mod r#trait;
mod definition;
mod registry;

// Re-export public types
pub use r#trait::Language;
pub use definition::LanguageDefinition;
pub use registry::LanguageRegistry;
