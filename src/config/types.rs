// src/config/types.rs - Configuration struct definitions
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub directories: DirectoryConfig,
    pub templates: TemplateConfig,
    pub generation: GenerationConfig,
    pub metadata: MetadataConfig,
    #[serde(default)]
    pub custom_fields: Vec<String>,
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
    pub use_case_template: Option<String>,
    pub test_template: Option<String>,
    /// List of methodologies to import and make available
    pub methodologies: Vec<String>,
    /// Default methodology to use when none specified
    pub default_methodology: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    /// Enable or disable metadata generation entirely
    pub enabled: bool,

    // Auto-populated fields (true/false to include or not)
    // These fields are automatically filled by the system when creating use cases:
    /// Auto-generated unique identifier
    pub include_id: bool,
    /// Use case title from command line argument
    pub include_title: bool,
    /// Category derived from directory structure
    pub include_category: bool,
    /// Current status (automatically set to "draft")
    pub include_status: bool,
    /// Priority level (automatically set to "medium")
    pub include_priority: bool,
    /// Creation timestamp (automatically set to current time)
    pub include_created: bool,
    /// Last updated timestamp (automatically set to current time)  
    pub include_last_updated: bool,

    // Extended metadata fields (true/false to enable/disable each field)
    /// Prerequisites and dependencies for the use case
    pub include_prerequisites: bool,
    /// Target users and stakeholders
    pub include_personas: bool,
    /// Author of the use case
    pub include_author: bool,
    /// Reviewer of the use case
    pub include_reviewer: bool,
    /// Business value and justification
    pub include_business_value: bool,
    /// Implementation complexity assessment
    pub include_complexity: bool,
    /// Associated epic or project
    pub include_epic: bool,
    /// Acceptance criteria for completion
    pub include_acceptance_criteria: bool,
    /// Assumptions made in the use case
    pub include_assumptions: bool,
    /// Constraints and limitations
    pub include_constraints: bool,
}