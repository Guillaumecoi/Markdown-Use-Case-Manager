//! # Configuration Types
//!
//! This module defines all the data structures used for MUCM configuration.
//! These types are serialized to/from TOML format for persistence and provide
//! the complete configuration schema for project management.
//!
//! ## Configuration Structure
//!
//! The configuration is organized into several key sections:
//! - `project`: Basic project information and metadata
//! - `directories`: File system paths for use cases, tests, and templates
//! - `templates`: Methodology and language template settings
//! - `base_fields`: Standard fields available to all use cases
//! - `metadata`: Auto-generated metadata settings
//! - `generation`: Code generation preferences and settings
//!
//! ## Configuration File
//!
//! Configurations are stored in `.config/.mucm/mucm.toml` and loaded using
//! the `ConfigFileManager`. The TOML format allows for human-readable configuration
//! while supporting complex nested structures.
//!
//! ## Methodology-Specific Configuration
//!
//! Individual methodologies can have their own configuration files stored in
//! `.config/.mucm/methodologies/{name}.toml`, which extend the base configuration
//! with methodology-specific fields and generation settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration structure for MUCM projects.
///
/// This struct represents the complete configuration state loaded from `.config/.mucm/mucm.toml`.
/// It contains all settings needed to manage use cases, templates, and project structure.
///
/// # Configuration Sections
///
/// - `project`: Basic project information (name, description)
/// - `directories`: Paths for use cases, tests, templates, and TOML files
/// - `templates`: Methodology and language template settings
/// - `base_fields`: Standard fields available to all use cases
/// - `metadata`: Auto-generated metadata settings (creation/update timestamps)
/// - `generation`: Code generation preferences (test language, auto-generation flags)
///
/// # Example Configuration
///
/// ```toml
/// [project]
/// name = "My Project"
/// description = "A project managed with use case manager"
///
/// [directories]
/// use_case_dir = "docs/use-cases"
/// test_dir = "tests/use-cases"
/// toml_dir = "use-cases-data"
///
/// [templates]
/// methodologies = ["developer", "feature", "business", "tester"]
/// default_methodology = "feature"
/// test_language = "python"
///
/// [generation]
/// test_language = "python"
/// auto_generate_tests = false
/// overwrite_test_documentation = false
///
/// [metadata]
/// created = true
/// last_updated = true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Basic project information and metadata
    pub project: ProjectConfig,
    /// Directory paths for use cases, tests, and templates
    pub directories: DirectoryConfig,
    /// Methodology and language template settings
    pub templates: TemplateConfig,
    /// Standard fields available to all use cases
    #[serde(default)]
    pub base_fields: HashMap<String, BaseFieldConfig>,
    /// Auto-generated metadata settings
    pub metadata: MetadataConfig,
    /// Code generation preferences and settings
    #[serde(default)]
    pub generation: GenerationConfig,
}

/// Project-level configuration settings.
///
/// Contains basic information about the project that appears in generated
/// documentation and is used for project identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Human-readable project name displayed in documentation
    pub name: String,
    /// Brief description of the project's purpose and scope
    pub description: String,
}

/// Directory configuration for file organization.
///
/// Defines the directory structure used for storing use case documentation,
/// test files, and template assets. Provides flexible path configuration
/// with sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryConfig {
    /// Directory where generated markdown use case files are stored
    /// Default: "docs/use-cases"
    pub use_case_dir: String,
    /// Directory where generated test files are stored
    /// Default: "tests/use-cases"
    pub test_dir: String,
    /// Optional custom template directory (uses built-in if not specified)
    pub template_dir: Option<String>,
    /// Directory for TOML source files (defaults to same as use_case_dir if not specified)
    /// This is where the raw use case data is stored before markdown generation
    pub toml_dir: Option<String>,
}

impl DirectoryConfig {
    /// Get the effective TOML directory path.
    ///
    /// Returns the configured TOML directory if specified, otherwise falls back
    /// to the use case directory for backward compatibility.
    ///
    /// # Returns
    /// The directory path as a string slice where TOML files should be stored
    pub fn get_toml_dir(&self) -> &str {
        self.toml_dir.as_deref().unwrap_or(&self.use_case_dir)
    }
}

/// Template configuration settings.
///
/// Defines which methodologies and languages are available for use case
/// generation, along with default selections for new projects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// List of methodologies to import and make available for use case creation
    /// These correspond to subdirectories in source-templates/methodologies/
    pub methodologies: Vec<String>,
    /// Default methodology to use when none is specified during use case creation
    /// Must be one of the values in the methodologies array
    pub default_methodology: String,
    /// Default programming language for test template generation
    /// Must be one of the supported languages (rust, python, javascript)
    pub test_language: String,
}

/// Per-methodology template configuration.
///
/// This configuration is loaded from `.config/.mucm/methodologies/{name}.toml`
/// and provides methodology-specific settings that extend the base configuration.
/// Note: Metadata configuration is handled in the main config, not per-methodology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyConfig {
    /// Template information and preferences for this methodology
    pub template: MethodologyTemplateInfo,
    /// Code generation settings specific to this methodology
    pub generation: GenerationConfig,
    /// Custom fields specific to this methodology (beyond base fields)
    #[serde(default)]
    pub custom_fields: HashMap<String, CustomFieldConfig>,
}

/// Template information for a specific methodology.
///
/// Contains metadata about a methodology's templates and preferred usage patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyTemplateInfo {
    /// Unique identifier for this methodology (used in file paths and commands)
    pub name: String,
    /// Human-readable description of when and how to use this methodology
    pub description: String,
    /// Preferred/recommended style for this methodology: "simple", "normal", or "detailed"
    /// This determines which template variant (.hbs file) is used by default
    pub preferred_style: String,
}

/// Configuration for base fields that all use cases have.
///
/// These fields are available in all methodologies and are defined in the main config.
/// Base fields extend beyond the mandatory id/title/category fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseFieldConfig {
    /// Human-readable label displayed in prompts and documentation
    pub label: String,
    /// Data type of the field: "string", "array", "number", "boolean"
    #[serde(rename = "type")]
    pub field_type: String,
    /// Whether this field must be provided when creating a use case
    #[serde(default)]
    pub required: bool,
    /// Default value if none provided (None means no default)
    #[serde(default)]
    pub default: Option<String>,
}

/// Configuration for custom fields specific to a methodology.
///
/// These fields extend the base fields and are only available in specific methodologies.
/// Custom fields allow methodologies to have specialized data collection requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldConfig {
    /// Human-readable label displayed in prompts and documentation
    pub label: String,
    /// Data type of the field: "string", "array", "number", "boolean"
    #[serde(rename = "type")]
    pub field_type: String,
    /// Whether this field must be provided when creating a use case with this methodology
    #[serde(default)]
    pub required: bool,
    /// Default value if none provided (None means no default)
    #[serde(default)]
    pub default_value: Option<String>,
}

/// Configuration for code generation and test creation settings.
///
/// Controls how code and tests are automatically generated when creating use cases,
/// including language selection and overwrite behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Programming language to use for generated test files
    /// Must be one of the supported languages (rust, python, javascript)
    pub test_language: String,
    /// Whether to automatically generate test files when creating use cases
    pub auto_generate_tests: bool,
    /// Whether to overwrite existing test documentation files during regeneration
    pub overwrite_test_documentation: bool,
}

impl Default for GenerationConfig {
    /// Create a default generation configuration.
    ///
    /// Returns a configuration with sensible defaults:
    /// - Python as the default test language
    /// - Auto-generation disabled
    /// - Overwrite protection enabled
    fn default() -> Self {
        Self {
            test_language: "python".to_string(),
            auto_generate_tests: false,
            overwrite_test_documentation: false,
        }
    }
}

/// Configuration for automatically generated metadata fields.
///
/// Controls which metadata fields are automatically populated when use cases
/// are created or modified, providing audit trails and timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    /// Whether to automatically set creation timestamp when use case is created
    /// Adds a "created" field with the current date/time
    pub created: bool,
    /// Whether to automatically update timestamp when use case is modified
    /// Updates a "last_updated" field with the current date/time
    pub last_updated: bool,
}
