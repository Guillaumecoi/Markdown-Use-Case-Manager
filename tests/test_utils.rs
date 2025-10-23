//! Test utilities and helper functions
//!
//! This module contains all test utility functions that are used across
//! multiple test files to provide consistent test setup and operations.

use anyhow::{Context, Result};
use markdown_use_case_manager::config::Config;
use markdown_use_case_manager::core::languages::LanguageRegistry;
use markdown_use_case_manager::core::models::{Priority, Scenario, Status, UseCase};
use std::fs;
use std::path::{Path, PathBuf};

/// Initialize a new project in the current directory with optional language
pub fn init_project_with_language(language: Option<String>) -> Result<Config> {
    init_project_with_language_in_dir(".", language)
}

/// Initialize a new project in a specific directory with optional language
pub fn init_project_with_language_in_dir(
    base_dir: &str,
    language: Option<String>,
) -> Result<Config> {
    let base_path = Path::new(base_dir);
    let config_dir = base_path.join(".config/.mucm");

    // Validate language if provided - check both current directory and built-ins
    if let Some(ref lang) = language {
        let language_registry = LanguageRegistry::new();

        // First check if the language is supported by the built-in registry (including aliases)
        if language_registry.get(lang).is_none() {
            // Check available languages from current working directory as fallback
            let available_languages = Config::get_available_languages()?;
            if !available_languages.contains(lang) {
                anyhow::bail!("Unsupported language '{}'. Supported languages: {}. Add templates to .config/.mucm/handlebars/lang-{}/ to support this language.", 
                            lang, available_languages.join(", "), lang);
            }
        }
    }

    // Create .config/.mucm directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create .config/.mucm directory")?;
    }

    let mut config = Config::default();

    // Set the test language if provided, resolving aliases to primary names
    let resolved_language = if let Some(ref lang) = language {
        let language_registry = LanguageRegistry::new();
        if let Some(lang_def) = language_registry.get(lang) {
            // Use the primary name (not alias)
            let primary_name = lang_def.name().to_string();
            config.generation.test_language = primary_name.clone();
            Some(primary_name)
        } else {
            // Keep original if not found in registry (might be user-defined)
            config.generation.test_language = lang.clone();
            Some(lang.clone())
        }
    } else {
        None
    };

    config.save_in_dir(base_dir)?;

    // Change to the base_dir temporarily to copy templates
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(base_dir)?;
    let result = Config::copy_templates_to_config_with_language(resolved_language);
    std::env::set_current_dir(original_dir)?;
    result?;

    // NOTE: Directories are NOT created during init
    // They will be created automatically when the first use case is created
    // This gives users a chance to configure directory paths in mucm.toml first

    Ok(config)
}

/// Initialize a new project in the current directory
pub fn init_project() -> Result<Config> {
    init_project_with_language(None)
}

/// Initialize a new project in a specific directory
pub fn init_project_in_dir(base_dir: &str) -> Result<Config> {
    init_project_with_language_in_dir(base_dir, None)
}

/// Load configuration from a directory
pub fn load_from_dir(dir: &str) -> Result<Config> {
    let config_path = PathBuf::from(dir).join(".config/.mucm").join("mucm.toml");

    if !config_path.exists() {
        anyhow::bail!(
            "No markdown use case manager project found in directory. Run 'mucm init' first."
        );
    }

    let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    config.save_in_dir(".")
}

/// Get templates directory path
pub fn templates_dir() -> PathBuf {
    Path::new(".config/.mucm").join("handlebars")
}

/// Set status on a scenario
pub fn set_scenario_status(scenario: &mut Scenario, status: Status) {
    scenario.status = status;
    scenario.metadata.touch();
}

/// Find use case by ID
pub fn find_use_case_by_id<'a>(use_cases: &'a [UseCase], id: &str) -> Option<&'a UseCase> {
    use_cases.iter().find(|uc| uc.id == id)
}

/// Get test template for a specific language
pub fn get_test_template_for_language(language_name: &str) -> Option<&'static str> {
    let registry = LanguageRegistry::new();
    registry.get(language_name).map(|lang| lang.test_template())
}

/// Get available test languages
pub fn get_available_test_languages() -> Vec<String> {
    let registry = LanguageRegistry::new();
    registry.available_languages()
}

/// Load methodology-specific configuration (for testing)
pub fn load_methodology_config(methodology: &str) -> Result<markdown_use_case_manager::config::MethodologyConfig> {
    let config_path = Path::new(".config/.mucm")
        .join("methodologies")
        .join(format!("{}.toml", methodology));

    if !config_path.exists() {
        anyhow::bail!(
            "Methodology config not found for '{}' at {:?}. \
             Make sure '{}' is in your methodologies list in mucm.toml.",
            methodology,
            config_path,
            methodology
        );
    }

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read methodology config at {:?}", config_path))?;
    
    let config: markdown_use_case_manager::config::MethodologyConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse methodology config for '{}'", methodology))?;

    Ok(config)
}

/// Test helper constructor for UseCase with default priority (Medium)
/// Creates a UseCase instance for testing purposes
pub fn create_test_use_case(
    id: String,
    title: String,
    category: String,
    description: String,
) -> UseCase {
    UseCase::new(id, title, category, description, Priority::Medium)
}
