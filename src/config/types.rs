// src/config/types.rs - Configuration struct definitions
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub directories: DirectoryConfig,
    pub templates: TemplateConfig,
    #[serde(default)]
    pub base_fields: HashMap<String, BaseFieldConfig>,
    pub metadata: MetadataConfig,
    /// Internal field for backwards compatibility - derived from methodology config
    #[serde(skip)]
    pub generation: GenerationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryConfig {
    pub use_case_dir: String,
    pub test_dir: String,
    pub persona_dir: String,
    pub template_dir: Option<String>,
    /// Directory for TOML source files (defaults to same as use_case_dir if not specified)
    pub toml_dir: Option<String>,
}

impl DirectoryConfig {
    /// Get the effective TOML directory (falls back to use_case_dir if not specified)
    pub fn get_toml_dir(&self) -> &str {
        self.toml_dir.as_deref().unwrap_or(&self.use_case_dir)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// List of methodologies to import and make available
    pub methodologies: Vec<String>,
    /// Default methodology to use when none specified
    pub default_methodology: String,
    /// Default test language
    pub test_language: String,
}

/// Per-methodology template configuration
/// This is loaded from .config/.mucm/methodologies/{name}.toml
/// Note: Metadata is configured in the main config, not per-methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyConfig {
    pub template: MethodologyTemplateInfo,
    pub generation: GenerationConfig,
    #[serde(default)]
    pub custom_fields: HashMap<String, CustomFieldConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyTemplateInfo {
    pub name: String,
    pub description: String,
    /// Preferred/recommended style for this methodology: "simple", "normal", or "detailed"
    pub preferred_style: String,
}

/// Configuration for base fields that all use cases have (beyond mandatory id/title/category)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseFieldConfig {
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String, // "string", "array", "number", "boolean"
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default: Option<String>,
}

/// Configuration for custom fields specific to a methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldConfig {
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String, // "string", "array", "number", "boolean"
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub test_language: String,
    pub auto_generate_tests: bool,
    pub overwrite_test_documentation: bool,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            test_language: "python".to_string(),
            auto_generate_tests: false,
            overwrite_test_documentation: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    /// Auto-set when use case is created
    pub created: bool,
    /// Auto-updated when use case is modified
    pub last_updated: bool,
}