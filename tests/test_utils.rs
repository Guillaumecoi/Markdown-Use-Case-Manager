//! Test utilities and helper functions
//!
//! This module contains all test utility functions that are used across
//! multiple test files to provide consistent test setup and operations.

use anyhow::{Context, Result};
use markdown_use_case_manager::config::Config;
use markdown_use_case_manager::core::languages::LanguageRegistry;
use markdown_use_case_manager::core::models::{Scenario, Status, UseCase};
use std::fs;
use std::path::{Path, PathBuf};

/// Initialize a new project in the current directory
pub fn init_project() -> Result<Config> {
    Config::init_project_with_language_in_dir(".", None)
}

/// Initialize a new project in a specific directory
pub fn init_project_in_dir(base_dir: &str) -> Result<Config> {
    Config::init_project_with_language_in_dir(base_dir, None)
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
    Path::new(".config/.mucm").join("templates")
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
