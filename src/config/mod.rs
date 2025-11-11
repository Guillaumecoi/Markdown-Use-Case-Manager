//! # Configuration Module
//!
//! This module handles all configuration-related functionality for the Markdown Use Case Manager (MUCM).
//! It manages project settings, template configurations, directory structures, and methodology preferences.
//!
//! ## Architecture
//!
//! The configuration system follows a layered approach:
//! - **Types** (`types.rs`): Data structures for configuration
//! - **File Management** (`file_manager.rs`): Loading/saving config files
//! - **Template Management** (`template_manager.rs`): Template copying and processing
//! - **Main Interface** (`mod.rs`): Public API and high-level operations
//!
//! ## Configuration File
//!
//! Configurations are stored in `.config/.mucm/mucm.toml` and contain:
//! - Project metadata (name, description)
//! - Directory paths for use cases, tests, and templates
//! - Methodology settings and defaults
//! - Language preferences for code generation
//! - Custom field definitions
//!
//! ## Two-Phase Initialization
//!
//! Project setup uses a two-phase process:
//! 1. **Configuration Phase**: Create config file with user preferences
//! 2. **Template Phase**: Copy templates based on selected languages/methodologies
//!
//! This allows users to review and modify configuration before templates are copied.

// Private sub-modules
mod file_manager;
mod template_manager;
mod types;

// Explicit public exports
pub use file_manager::ConfigFileManager;
pub use template_manager::TemplateManager;
pub use types::{Config, GenerationConfig};

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
    pub const TEMPLATES_DIR: &'static str = "template-assets";

    /// Create a minimal config for template processing.
    ///
    /// This creates a basic configuration used only for template variable substitution.
    /// It's not a complete project configuration and should not be saved directly.
    ///
    /// # Arguments
    /// * `test_language` - The programming language for test templates
    /// * `methodology` - Optional default methodology override
    ///
    /// # Returns
    /// A minimal Config instance suitable for template processing
    pub fn for_template(test_language: Option<String>, methodology: Option<String>) -> Self {
        let mut config = Self::default();
        if let Some(lang) = test_language {
            config.generation.test_language = lang.clone();
            config.templates.test_language = lang;
        }
        if let Some(method) = methodology {
            config.templates.default_methodology = method;
        }
        config
    }

    /// Get the path to the configuration file.
    ///
    /// Returns the full path to `.config/.mucm/mucm.toml` relative to the current directory.
    pub fn config_path() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::CONFIG_FILE)
    }

    /// Save configuration file only (without copying templates or creating directories).
    ///
    /// This is the first step of two-step initialization. It creates the config file
    /// but doesn't copy templates yet, allowing users to review and modify the config.
    ///
    /// Use `finalize_init()` after this to complete the setup.
    ///
    /// # Arguments
    /// * `config` - The configuration to save
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if saving fails
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

    /// Load configuration from the standard config file.
    ///
    /// Reads `.config/.mucm/mucm.toml` and deserializes it into a Config struct.
    ///
    /// # Returns
    /// The loaded configuration, or an error if the file doesn't exist or is invalid
    pub fn load() -> Result<Self> {
        ConfigFileManager::load()
    }

    /// Save configuration to file in specified directory.
    ///
    /// # Arguments
    /// * `self` - The configuration to save
    /// * `base_dir` - Base directory where the config should be saved
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if saving fails
    pub fn save_in_dir(&self, base_dir: &str) -> Result<()> {
        ConfigFileManager::save_in_dir(self, base_dir)
    }

    /// Check if templates have already been copied to .config/.mucm/handlebars/
    ///
    /// This is used to determine if the project has completed the template setup phase.
    ///
    /// # Returns
    /// `true` if templates exist, `false` otherwise
    pub fn check_templates_exist() -> bool {
        ConfigFileManager::check_templates_exist()
    }

    /// Copy templates to .config/.mucm/handlebars/ with language (wrapper for _in_dir version)
    ///
    /// This is the second phase of initialization - copying templates after config review.
    ///
    /// # Arguments
    /// * `language` - Optional language override for template selection
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if copying fails
    pub fn copy_templates_to_config_with_language(language: Option<String>) -> Result<()> {
        Self::copy_templates_to_config_with_language_in_dir(".", language)
    }

    /// Copy templates to config directory (internal implementation)
    ///
    /// # Arguments
    /// * `base_dir` - Base directory for the operation
    /// * `language` - Optional language override (currently unused - copies all languages)
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if copying fails
    fn copy_templates_to_config_with_language_in_dir(
        base_dir: &str,
        _language: Option<String>, // Not currently used - we copy all languages now
    ) -> Result<()> {
        TemplateManager::copy_templates_to_config(base_dir)
    }

    /// Get list of available programming languages from source templates and local config
    ///
    /// Returns languages that have templates available in the source-templates directory.
    ///
    /// # Returns
    /// A vector of language names, or an error if discovery fails
    pub fn get_available_languages() -> Result<Vec<String>> {
        use crate::config::template_manager::TemplateManager;

        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let registry = LanguageRegistry::new_dynamic(&templates_dir)?;
        Ok(registry.available_languages())
    }

    /// Get methodology-specific recommendations as a human-readable string
    ///
    /// # Arguments
    /// * `methodology` - The methodology name to get recommendations for
    ///
    /// # Returns
    /// A formatted string with usage recommendations for the methodology
    pub fn methodology_recommendations(methodology: &str) -> String {
        MethodologyManager::get_recommendations(methodology)
    }

    /// Get list of available methodologies (those with config files)
    ///
    /// Returns methodologies that have been configured in source-templates/methodologies/.
    ///
    /// # Returns
    /// A vector of methodology names, or an error if discovery fails
    pub fn list_available_methodologies() -> Result<Vec<String>> {
        MethodologyManager::list_available()
    }

    /// Load default configuration from source-templates/config.toml
    fn load_default_from_template() -> Result<Self> {
        use crate::config::template_manager::TemplateManager;
        use std::fs;

        let source_templates_dir = TemplateManager::find_source_templates_dir()?;
        let config_path = source_templates_dir.join("config.toml");

        let content = fs::read_to_string(&config_path)
            .context("Failed to read source-templates/config.toml")?;
        let mut config: Config =
            toml::from_str(&content).context("Failed to parse source-templates/config.toml")?;

        // Set default generation config that matches the template defaults
        config.generation = GenerationConfig {
            test_language: config.templates.test_language.clone(),
            auto_generate_tests: false,
            overwrite_test_documentation: false,
        };

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        // Load default configuration from source-templates/config.toml
        // This ensures consistency between the template and the default config
        match Self::load_default_from_template() {
            Ok(config) => config,
            Err(e) => {
                panic!(
                    "Failed to load default configuration from source-templates/config.toml: {}\n\
                     The source-templates directory and config.toml file are required for the application to function.\n\
                     Please ensure you are running from the project root directory.",
                    e
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::TempDir;

    /// Helper to initialize a project in a temporary directory with optional language
    fn init_project_with_language(language: Option<String>) -> Result<Config> {
        use crate::config::template_manager::TemplateManager;

        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let language_registry = LanguageRegistry::new_dynamic(&templates_dir)?;

        // Validate language if provided
        if let Some(ref lang) = language {
            if language_registry.get(lang).is_none() {
                let available = language_registry.available_languages();
                anyhow::bail!(
                    "Unsupported language '{}'. Supported languages: {}",
                    lang,
                    available.join(", ")
                );
            }
        }

        let config_dir = Path::new(".config/.mucm");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create .config/.mucm directory")?;
        }

        let mut config = Config::default();

        // Set the test language if provided, resolving aliases to primary names
        if let Some(ref lang) = language {
            if let Some(lang_def) = language_registry.get(lang) {
                let primary_name = lang_def.name().to_string();
                config.generation.test_language = primary_name.clone();
                config.templates.test_language = primary_name.clone();
            } else {
                config.generation.test_language = lang.clone();
                config.templates.test_language = lang.clone();
            }
        }

        config.save_in_dir(".")?;
        Config::copy_templates_to_config_with_language(language)?;

        Ok(config)
    }

    #[test]
    #[serial]
    fn test_auto_init_detection() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let result = Config::load();
        assert!(
            result.is_err(),
            "Should fail when project is not initialized"
        );

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("No markdown use case manager project found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_auto_init_process() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        assert!(Config::load().is_err());

        let config = init_project_with_language(Some("rust".to_string()))?;

        assert!(Config::config_path().exists(), "Config file should exist");
        assert_eq!(config.generation.test_language, "rust");

        let use_case_dir = Path::new(&config.directories.use_case_dir);
        let test_dir = Path::new(&config.directories.test_dir);
        assert!(
            !use_case_dir.exists(),
            "Use case directory should NOT exist yet"
        );
        assert!(!test_dir.exists(), "Test directory should NOT exist yet");

        let templates_dir = Path::new(".config/.mucm").join(Config::TEMPLATES_DIR);
        assert!(templates_dir.exists(), "Templates directory should exist");
        assert!(templates_dir.join("developer/uc_simple.hbs").exists());
        assert!(templates_dir.join("developer/uc_detailed.hbs").exists());
        assert!(templates_dir.join("languages/rust/test.hbs").exists());

        let reloaded_config = Config::load()?;
        assert_eq!(reloaded_config.generation.test_language, "rust");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_auto_init_language_options() -> Result<()> {
        // Test with Python
        {
            let temp_dir = TempDir::new()?;
            std::env::set_current_dir(&temp_dir)?;

            let config = init_project_with_language(Some("python".to_string()))?;
            assert_eq!(config.generation.test_language, "python");

            let python_template = Path::new(".config/.mucm")
                .join(Config::TEMPLATES_DIR)
                .join("languages/python/test.hbs");
            assert!(python_template.exists(), "Python template should exist");
        }

        // Test with None (default language)
        {
            let temp_dir = TempDir::new()?;
            std::env::set_current_dir(&temp_dir)?;

            let config = init_project_with_language(None)?;
            assert_eq!(config.generation.test_language, "python"); // Default from Config::default()
        }

        Ok(())
    }

    #[test]
    #[serial]
    fn test_config_management() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let mut config = init_project_with_language(Some("rust".to_string()))?;

        config.project.name = "Modified Project".to_string();
        config.project.description = "Modified description".to_string();
        config.directories.use_case_dir = "custom/use-cases".to_string();
        config.directories.test_dir = "custom/tests".to_string();
        config.generation.test_language = "python".to_string();
        config.generation.auto_generate_tests = true;
        config.metadata.created = false;
        config.metadata.last_updated = false;

        config.save_in_dir(".")?;

        let reloaded_config = Config::load()?;
        assert_eq!(reloaded_config.project.name, "Modified Project");
        assert_eq!(reloaded_config.project.description, "Modified description");
        assert_eq!(reloaded_config.directories.use_case_dir, "custom/use-cases");
        assert_eq!(reloaded_config.directories.test_dir, "custom/tests");
        assert_eq!(reloaded_config.generation.test_language, "python");
        assert!(reloaded_config.generation.auto_generate_tests);
        assert!(!reloaded_config.metadata.created);
        assert!(!reloaded_config.metadata.last_updated);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_available_languages_for_settings() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let languages = Config::get_available_languages();
        match languages {
            Ok(langs) => {
                assert!(!langs.is_empty(), "Should have built-in languages");
                assert!(
                    langs.contains(&"rust".to_string()) || langs.contains(&"python".to_string())
                );
            }
            Err(_) => {
                // It's okay if this fails in some test environments
            }
        }

        init_project_with_language(Some("rust".to_string()))?;

        let languages = Config::get_available_languages()?;
        assert!(!languages.is_empty(), "Should have languages after init");
        assert!(languages.contains(&"rust".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_config_validation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let config = init_project_with_language(Some("rust".to_string()))?;

        let toml_content = toml::to_string_pretty(&config)?;
        let parsed_config: Config = toml::from_str(&toml_content)?;

        assert_eq!(parsed_config.project.name, config.project.name);
        assert_eq!(
            parsed_config.generation.test_language,
            config.generation.test_language
        );
        assert_eq!(parsed_config.metadata.created, config.metadata.created);
        assert_eq!(
            parsed_config.metadata.last_updated,
            config.metadata.last_updated
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn test_config_error_handling() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let result = Config::load();
        assert!(result.is_err());

        let config = Config::default();
        let _ = config.save_in_dir(".");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_auto_init_settings_integration() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let mut config = init_project_with_language(Some("rust".to_string()))?;

        config.project.name = "Integration Test Project".to_string();
        config.directories.use_case_dir = "docs/custom-use-cases".to_string();
        config.generation.auto_generate_tests = true;
        config.metadata.created = true;
        config.metadata.last_updated = true;

        config.save_in_dir(".")?;

        let saved_config = Config::load()?;
        assert_eq!(saved_config.project.name, "Integration Test Project");

        fs::create_dir_all(&config.directories.use_case_dir)?;
        fs::create_dir_all(&config.directories.test_dir)?;

        use crate::core::UseCaseApplicationService;
        let mut coordinator = UseCaseApplicationService::load()?;

        let _uc_id = coordinator.create_use_case_with_methodology(
            "Integration Test Use Case".to_string(),
            "integration".to_string(),
            Some("Testing integration between auto-init and settings".to_string()),
            "feature",
        )?;

        let custom_use_case_file = Path::new("docs/custom-use-cases/integration/UC-INT-001.md");
        assert!(
            custom_use_case_file.exists(),
            "Use case should be created in custom directory"
        );

        let final_config = Config::load()?;
        assert_eq!(final_config.project.name, "Integration Test Project");
        assert_eq!(
            final_config.directories.use_case_dir,
            "docs/custom-use-cases"
        );
        assert!(final_config.generation.auto_generate_tests);
        assert!(final_config.metadata.created);
        assert!(final_config.metadata.last_updated);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_for_template() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Test with both language and methodology set
        let config = Config::for_template(Some("python".to_string()), Some("feature".to_string()));
        assert_eq!(config.generation.test_language, "python");
        assert_eq!(config.templates.test_language, "python");
        assert_eq!(config.templates.default_methodology, "feature");

        // Test with language set but methodology None (should keep default methodology)
        let config_lang_only = Config::for_template(Some("rust".to_string()), None);
        assert_eq!(config_lang_only.generation.test_language, "rust");
        assert_eq!(config_lang_only.templates.test_language, "rust");
        // Should use default methodology from Config::default()
        assert!(!config_lang_only.templates.default_methodology.is_empty());

        // Test with language None but methodology set (should keep default language)
        let default_config = Config::default();
        let config_methodology_only = Config::for_template(None, Some("business".to_string()));
        assert_eq!(
            config_methodology_only.generation.test_language,
            default_config.generation.test_language
        );
        assert_eq!(
            config_methodology_only.templates.test_language,
            default_config.templates.test_language
        );
        assert_eq!(
            config_methodology_only.templates.default_methodology,
            "business"
        );

        // Test with both None (should keep all defaults)
        let config_both_none = Config::for_template(None, None);
        assert_eq!(
            config_both_none.generation.test_language,
            default_config.generation.test_language
        );
        assert_eq!(
            config_both_none.templates.test_language,
            default_config.templates.test_language
        );
        assert_eq!(
            config_both_none.templates.default_methodology,
            default_config.templates.default_methodology
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn test_save_config_only() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let mut config = Config::default();
        config.project.name = "Test Project".to_string();
        config.generation.test_language = "javascript".to_string();

        // Save config only (without templates)
        Config::save_config_only(&config)?;

        // Verify config file exists
        assert!(Config::config_path().exists());

        // Verify templates directory does NOT exist (since we only saved config)
        let templates_dir = Path::new(".config/.mucm").join(Config::TEMPLATES_DIR);
        assert!(!templates_dir.exists());

        // Verify we can load the config back
        let loaded_config = Config::load()?;
        assert_eq!(loaded_config.project.name, "Test Project");
        assert_eq!(loaded_config.generation.test_language, "javascript");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_check_templates_exist() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Initially, templates should not exist
        assert!(!Config::check_templates_exist());

        // Initialize project (which copies templates)
        let _config = init_project_with_language(Some("rust".to_string()))?;

        // Now templates should exist
        assert!(Config::check_templates_exist());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_methodology_recommendations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Initialize project to set up config directory
        init_project_with_language(Some("rust".to_string()))?;

        // Test methodology recommendations for known methodologies
        let feature_recommendations = Config::methodology_recommendations("feature");
        assert!(!feature_recommendations.is_empty());
        assert!(
            feature_recommendations.contains("feature")
                || feature_recommendations.contains("Feature")
        );

        let developer_recommendations = Config::methodology_recommendations("developer");
        assert!(!developer_recommendations.is_empty());
        assert!(
            developer_recommendations.contains("developer")
                || developer_recommendations.contains("Developer")
        );

        // Test with unknown methodology
        let unknown_recommendations = Config::methodology_recommendations("unknown_methodology");
        assert!(!unknown_recommendations.is_empty()); // Should return some default response

        Ok(())
    }

    #[test]
    #[serial]
    fn test_list_available_methodologies() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Initialize project to set up config directory
        init_project_with_language(Some("rust".to_string()))?;

        let methodologies = Config::list_available_methodologies()?;

        // Should have some methodologies
        assert!(!methodologies.is_empty());

        // Should include common methodologies
        assert!(
            methodologies.contains(&"feature".to_string())
                || methodologies.contains(&"developer".to_string())
                || methodologies.contains(&"business".to_string())
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn test_config_path() -> Result<()> {
        let expected_path = Path::new(".config/.mucm/mucm.toml");
        assert_eq!(Config::config_path(), expected_path);

        Ok(())
    }
}
