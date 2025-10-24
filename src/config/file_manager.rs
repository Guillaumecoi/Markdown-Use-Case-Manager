// src/config/file_manager.rs - Configuration file loading and saving
use crate::config::types::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path};

pub struct ConfigFileManager;

impl ConfigFileManager {
    /// Load configuration from file
    pub fn load() -> Result<Config> {
        let config_path = Config::config_path();

        if !config_path.exists() {
            anyhow::bail!("No markdown use case manager project found. Run 'mucm init' first.");
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let mut config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        // Populate the generation field from templates (for backwards compatibility)
        config.generation.test_language = config.templates.test_language.clone();

        Ok(config)
    }

    /// Save configuration to file in specified directory
    pub fn save_in_dir(config: &Config, base_dir: &str) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_path = base_path.join(Config::CONFIG_DIR).join("mucm.toml");
        let content = toml::to_string_pretty(config).context("Failed to serialize config")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Check if templates have already been copied to .config/.mucm/handlebars/
    pub fn check_templates_exist() -> bool {
        let base_path = Path::new(".");
        let templates_dir = base_path.join(Config::CONFIG_DIR).join(Config::TEMPLATES_DIR);
        templates_dir.exists() && templates_dir.is_dir()
    }
}