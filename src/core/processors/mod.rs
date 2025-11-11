// src/core/processors/mod.rs

//! Simple template-driven methodology system
//!
//! This module provides a clean, configuration-driven approach where:
//! 1. Methodology config (custom fields) lives in source-templates/methodologies/{name}/config.toml
//! 2. Templates (.hbs files) live in the same directory
//! 3. Use case data (TOML) gets inserted into templates
//!
//! No complex processors, no transformations - just TOML + Handlebars!

mod methodology_manager;
mod template_driven;

// Explicit public export
pub use methodology_manager::MethodologyManager;
