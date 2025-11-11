// src/config/mod.rs - Configuration module entry point

use std::collections::HashMap;

// Private sub-modules
mod file_manager;
mod template_manager;
mod types;

// Explicit public exports
pub use file_manager::ConfigFileManager;
pub use template_manager::TemplateManager;
pub use types::*;

// Re-export from other modules
pub use crate::core::MethodologyManager;

use crate::core::LanguageRegistry;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

impl Config {
    // Constants
    pub const CONFIG_DIR: &'static str = ".config/.mucm";
    pub const CONFIG_FILE: &'static str = "mucm.toml";
    pub const TEMPLATES_DIR: &'static str = "handlebars";

    /// Create a config for template processing (minimal config used only for template variables)
    pub fn for_template(test_language: String, methodology: Option<String>) -> Self {
        let mut config = Self::default();
        config.templates.test_language = test_language.clone();
        config.generation.test_language = test_language; // Keep generation in sync
        if let Some(method) = methodology {
            config.templates.default_methodology = method;
        }
        config
    }

    /// Get path to config file
    pub fn config_path() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::CONFIG_FILE)
    }

    /// Save config file only (without copying templates or creating directories)
    /// Used in the first step of two-step initialization
    pub fn save_config_only(config: &Config) -> Result<()> {
        let base_path = Path::new(".");
        let config_dir = base_path.join(Self::CONFIG_DIR);

        // Create .config/.mucm directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create .config/.mucm directory")?;
        }

        // Use template file instead of programmatic generation
        TemplateManager::create_config_from_template(config)?;
        Ok(())
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        ConfigFileManager::load()
    }

    /// Save configuration to file in specified directory
    pub fn save_in_dir(&self, base_dir: &str) -> Result<()> {
        ConfigFileManager::save_in_dir(self, base_dir)
    }

    /// Check if templates have already been copied to .config/.mucm/handlebars/
    pub fn check_templates_exist() -> bool {
        ConfigFileManager::check_templates_exist()
    }

    /// Copy templates to .config/.mucm/handlebars/ with language (wrapper for _in_dir version)
    pub fn copy_templates_to_config_with_language(language: Option<String>) -> Result<()> {
        Self::copy_templates_to_config_with_language_in_dir(".", language)
    }

    /// Copy templates to config directory
    fn copy_templates_to_config_with_language_in_dir(
        base_dir: &str,
        _language: Option<String>, // Not currently used - we copy all languages now
    ) -> Result<()> {
        TemplateManager::copy_templates_to_config(base_dir)
    }

    /// Get list of available programming languages from source templates and local config
    pub fn get_available_languages() -> Result<Vec<String>> {
        let registry = LanguageRegistry::new();
        Ok(registry.available_languages())
    }

    /// Get methodology-specific recommendations as a human-readable string
    pub fn methodology_recommendations(methodology: &str) -> String {
        MethodologyManager::get_recommendations(methodology)
    }

    /// Get list of available methodologies (those with config files)
    pub fn list_available_methodologies() -> Result<Vec<String>> {
        MethodologyManager::list_available()
    }
}

impl Default for Config {
    fn default() -> Self {
        // Minimal config used only for tests and template variable processing
        // Production configs are created from source-templates/config.toml
        let mut base_fields = HashMap::new();

        // Add standard base fields
        base_fields.insert(
            "description".to_string(),
            BaseFieldConfig {
                label: "Description".to_string(),
                field_type: "string".to_string(),
                required: true,
                default: None,
            },
        );
        base_fields.insert(
            "status".to_string(),
            BaseFieldConfig {
                label: "Status".to_string(),
                field_type: "string".to_string(),
                required: false,
                default: Some("draft".to_string()),
            },
        );
        base_fields.insert(
            "priority".to_string(),
            BaseFieldConfig {
                label: "Priority".to_string(),
                field_type: "string".to_string(),
                required: false,
                default: Some("medium".to_string()),
            },
        );
        base_fields.insert(
            "author".to_string(),
            BaseFieldConfig {
                label: "Author".to_string(),
                field_type: "string".to_string(),
                required: false,
                default: None,
            },
        );
        base_fields.insert(
            "reviewer".to_string(),
            BaseFieldConfig {
                label: "Reviewer".to_string(),
                field_type: "string".to_string(),
                required: false,
                default: None,
            },
        );

        Config {
            project: ProjectConfig {
                name: "My Project".to_string(),
                description: "A project managed with use case manager".to_string(),
            },
            directories: DirectoryConfig {
                use_case_dir: "docs/use-cases".to_string(),
                test_dir: "tests/use-cases".to_string(),
                template_dir: None,
                toml_dir: Some("use-cases-data".to_string()),
            },
            templates: TemplateConfig {
                methodologies: vec![
                    "developer".to_string(),
                    "feature".to_string(),
                    "business".to_string(),
                    "tester".to_string(),
                ],
                default_methodology: "feature".to_string(),
                test_language: "python".to_string(),
            },
            base_fields,
            metadata: MetadataConfig {
                created: true,
                last_updated: true,
            },
            generation: GenerationConfig {
                test_language: "python".to_string(),
                auto_generate_tests: false,
                overwrite_test_documentation: false,
            },
        }
    }
}
