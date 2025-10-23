// src/config/language.rs - Language discovery and management
use crate::core::languages::LanguageRegistry;
use crate::config::types::Config;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct LanguageManager;

impl LanguageManager {
    /// Get list of available programming languages from source templates and local config
    pub fn get_available_languages() -> Result<Vec<String>> {
        let mut languages = Vec::new();

        // Start with built-in language registry
        let language_registry = LanguageRegistry::new();
        languages.extend(language_registry.available_languages());

        // Look for user-defined languages in current directory
        let config_dir = Path::new(Config::CONFIG_DIR);
        let templates_dir = config_dir.join(Config::TEMPLATES_DIR);

        if templates_dir.exists() {
            for entry in fs::read_dir(&templates_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();

                    // Check for "lang-{language}" pattern (preferred)
                    if let Some(lang) = dir_name.strip_prefix("lang-") {
                        if !languages.contains(&lang.to_string()) {
                            languages.push(lang.to_string());
                        }
                    }
                }
            }
        }

        languages.sort();
        Ok(languages)
    }
}