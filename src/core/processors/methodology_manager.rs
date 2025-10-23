// src/core/processors/methodology_manager.rs - Methodology-specific management and recommendations
use anyhow::Result;
use std::fs;

pub struct MethodologyManager;

impl MethodologyManager {
    /// Get methodology-specific recommendations as a human-readable string
    /// This method reads recommendations from the methodology's config.toml file
    pub fn get_recommendations(methodology: &str) -> String {
        // Try to read from config file
        if let Ok(config_dir) = Self::find_config_dir() {
            let config_path = config_dir
                .join("methodologies")
                .join(methodology)
                .join("config.toml");
            
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(value) = toml::from_str::<toml::Value>(&content) {
                    if let Some(desc) = value.get("description").and_then(|v| v.as_str()) {
                        return format!(
                            "{} Methodology:\n{}",
                            methodology,
                            desc
                        );
                    }
                }
            }
        }
        
        // Fallback if config not found
        format!(
            "{} Methodology\n\
            Configuration-driven methodology. See source-templates/methodologies/{} for details.",
            methodology, methodology
        )
    }

    /// Get list of available methodologies (those with config files)
    pub fn list_available() -> Result<Vec<String>> {
        let methodologies_dir = Self::find_config_dir()?
            .join("methodologies");

        if !methodologies_dir.exists() {
            return Ok(Vec::new());
        }

        let mut methodologies = Vec::new();
        for entry in fs::read_dir(&methodologies_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Files are named {methodology}.toml
                    methodologies.push(name.to_string());
                }
            }
        }

        methodologies.sort();
        Ok(methodologies)
    }

    /// Find the .config/.mucm directory by walking up the directory tree
    fn find_config_dir() -> Result<std::path::PathBuf> {
        let mut current_dir = std::env::current_dir()?;
        
        loop {
            let config_dir = current_dir.join(".config/.mucm");
            if config_dir.exists() && config_dir.is_dir() {
                return Ok(config_dir);
            }
            
            // Try parent directory
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                anyhow::bail!(
                    "No .config/.mucm directory found. Run 'mucm init' first to initialize a project."
                );
            }
        }
    }
}