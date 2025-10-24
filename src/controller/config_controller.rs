use anyhow::Result;

use crate::config::Config;
use super::dto::{ConfigSummary, DisplayResult};

/// Controller for configuration management
pub struct ConfigController;

impl ConfigController {
    /// Get current configuration summary
    pub fn get_config_summary() -> Result<ConfigSummary> {
        let config = Config::load()?;
        
        Ok(ConfigSummary {
            project_name: config.project.name,
            project_description: config.project.description,
            use_case_dir: config.directories.use_case_dir,
            test_dir: config.directories.test_dir,
            test_language: config.generation.test_language,
            default_methodology: config.templates.default_methodology,
            auto_generate_tests: config.generation.auto_generate_tests,
        })
    }

    /// Save configuration
    pub fn save_config(config: Config) -> Result<DisplayResult> {
        config.save_in_dir(".")?;
        Ok(DisplayResult::success("Configuration saved successfully!".to_string()))
    }

    /// Get the full config for editing
    pub fn load_config() -> Result<Config> {
        Config::load()
    }
}
