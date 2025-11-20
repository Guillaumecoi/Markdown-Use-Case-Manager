//! Methodology trait and related types.
//!
//! Defines the interface that all methodology implementations must provide.

use std::collections::HashMap;

use super::types::CustomFieldConfig;

/// Represents a methodology supported by the system.
///
/// This trait defines the interface that all methodology implementations must provide.
/// Methodologies encapsulate metadata about documentation methodologies and their associated
/// templates and configuration.
pub trait Methodology {
    /// Returns the primary name of the methodology (e.g., "business", "developer").
    fn name(&self) -> &str;

    /// Returns the title of the methodology for display purposes.
    fn title(&self) -> &str;

    /// Returns a description of the methodology.
    fn description(&self) -> &str;

    /// Returns when to use this methodology.
    fn when_to_use(&self) -> &[String];

    /// Returns the key features of this methodology.
    fn key_features(&self) -> &[String];

    /// Returns the available documentation levels for this methodology.
    fn levels(&self) -> &[DocumentationLevel];

    /// Returns the preferred documentation style for this methodology.
    fn preferred_style(&self) -> &str;

    /// Returns the custom fields specific to this methodology.
    fn custom_fields(&self) -> &HashMap<String, CustomFieldConfig>;
}

/// Represents a documentation level within a methodology.
///
/// Each methodology can have multiple levels (e.g., simple, normal, detailed)
/// with different templates and levels of detail.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DocumentationLevel {
    /// Name of the level (e.g., "simple", "normal", "detailed")
    pub name: String,
    /// Short abbreviation for file suffixes (e.g., "s", "n", "d")
    pub abbreviation: String,
    /// Template filename for this level (e.g., "uc_simple.hbs")
    pub filename: String,
    /// Description of what this level provides
    pub description: String,
    /// List of level names this level inherits from (for field resolution)
    #[serde(default)]
    pub inherits: Vec<String>,
}
