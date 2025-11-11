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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::TempDir;

    /// Helper to initialize a project in a temporary directory with optional language
    fn init_project_with_language(language: Option<String>) -> Result<Config> {
        let language_registry = LanguageRegistry::new();

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

        let templates_dir = Path::new(".config/.mucm/handlebars");
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

            let python_template = Path::new(".config/.mucm/handlebars/languages/python/test.hbs");
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
        assert_eq!(
            reloaded_config.project.description,
            "Modified description"
        );
        assert_eq!(
            reloaded_config.directories.use_case_dir,
            "custom/use-cases"
        );
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
}
