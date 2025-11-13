//! # Template Manager
//!
//! This module handles template file management and processing for the Markdown Use Case Manager.
//! It provides functionality to discover, copy, and manage template assets used for generating
//! use case documentation and test files.
//!
//! ## Template Structure
//!
//! Templates are organized in a hierarchical structure:
//! - `source-templates/` - Source template directory
//!   - `overview.hbs` - Root template files
//!   - `methodologies/` - Methodology-specific templates and configs
//!     - `{methodology}/` - Individual methodology directory
//!       - `config.toml` - Methodology configuration
//!       - Template files (`.hbs`)
//!   - `languages/` - Language-specific test templates
//!     - `{language}/` - Language directory
//!       - `test.hbs` - Test template for the language
//!
//! ## Template Copying Process
//!
//! The template copying process involves:
//! 1. Locating the source templates directory
//! 2. Reading the project configuration
//! 3. Copying root templates (overview.hbs)
//! 4. Copying methodology-specific templates and configs
//! 5. Copying language-specific templates
//!
//! ## Configuration Integration
//!
//! Templates are selected based on the project's configuration:
//! - `config.templates.methodologies` determines which methodologies to copy
//! - `config.templates.test_language` determines which language templates to copy

use crate::config::types::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TemplateManager;

impl TemplateManager {
    /// Create a configuration file from the current config state.
    ///
    /// Serializes the provided configuration to TOML format and writes it to
    /// the standard configuration file path. This ensures that user-selected
    /// languages and methodologies are persisted to disk.
    ///
    /// # Arguments
    /// * `config` - The configuration to serialize and save
    ///
    /// # Errors (This function will return an error if)
    /// * The configuration cannot be serialized to TOML
    /// * The configuration file cannot be written
    pub fn create_config_from_template(config: &Config) -> Result<()> {
        // Serialize the config to TOML instead of copying the template
        // This ensures the user's chosen language and methodology are saved
        let config_content =
            toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;

        // Write the config
        let config_path = Config::config_path();
        fs::write(&config_path, config_content).context("Failed to write config file")?;

        Ok(())
    }

    /// Locate the source templates directory.
    ///
    /// Searches for the source-templates directory in multiple locations:
    /// 1. Current working directory
    /// 2. CARGO_MANIFEST_DIR environment variable (for tests/builds)
    /// 3. Relative to the current executable path
    ///
    /// # Returns
    /// The path to the source-templates directory, or an error if not found.
    ///
    /// # Errors
    /// Returns an error if the source-templates directory cannot be located
    /// in any of the expected locations.
    pub fn find_source_templates_dir() -> Result<PathBuf> {
        // Try current directory first
        let local_templates = Path::new("source-templates");
        if local_templates.exists() {
            return Ok(local_templates.to_path_buf());
        }

        // Try CARGO_MANIFEST_DIR (set during cargo test and build)
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let cargo_templates = Path::new(&manifest_dir).join("source-templates");
            if cargo_templates.exists() {
                return Ok(cargo_templates);
            }
        }

        // If still not found, try to find templates relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Check ../../source-templates (when running from target/release/)
                let dev_templates = exe_dir
                    .parent()
                    .and_then(|p| p.parent())
                    .map(|p| p.join("source-templates"));
                if let Some(dev_templates) = dev_templates {
                    if dev_templates.exists() {
                        return Ok(dev_templates);
                    }
                }
            }
        }

        anyhow::bail!("Source templates directory not found. Run from project root or ensure source-templates/ exists.")
    }

    /// Copy all templates to the configuration directory.
    ///
    /// This is the main template copying function that sets up the project's
    /// template assets. It copies root templates, methodology-specific templates,
    /// and language-specific templates to the appropriate configuration directories.
    ///
    /// The function reads the project's configuration to determine which methodologies
    /// and languages to copy, then performs a complete template setup.
    ///
    /// # Arguments
    /// * `base_dir` - Base directory for the operation (usually ".")
    ///
    /// # Errors (This function will return an error if)
    /// * The configuration file is missing or invalid
    /// * Source templates directory cannot be found
    /// * Template directories cannot be created
    /// * Individual template files cannot be copied
    /// * Required methodologies are missing from source templates
    pub fn copy_templates_to_config(base_dir: &str) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_templates_dir = base_path
            .join(Config::CONFIG_DIR)
            .join(Config::TEMPLATES_DIR);

        // Create directories
        fs::create_dir_all(&config_templates_dir)
            .context("Failed to create config templates directory")?;

        // Load the config from base_dir to see which methodologies to import
        let config_path = base_path.join(Config::CONFIG_DIR).join("mucm.toml");
        if !config_path.exists() {
            anyhow::bail!(
                "Config file not found at {:?} - run 'mucm init' first",
                config_path
            );
        }
        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        let source_templates_dir = Self::find_source_templates_dir()?;

        // Copy root template files
        Self::copy_root_templates(&source_templates_dir, &config_templates_dir)?;

        // Copy methodologies
        Self::copy_methodologies(&source_templates_dir, &config, &config_templates_dir)?;

        // Copy language templates
        Self::copy_language_templates(&source_templates_dir, &config, &config_templates_dir)?;

        Ok(())
    }

    /// Copy root template files (overview.hbs, etc.).
    ///
    /// Copies template files that are not specific to any methodology or language,
    /// such as overview templates used for generating project documentation.
    ///
    /// # Arguments
    /// * `source_templates_dir` - Path to the source templates directory
    /// * `config_templates_dir` - Path to the destination templates directory
    ///
    /// # Errors
    /// Returns an error if template files cannot be copied.
    fn copy_root_templates(source_templates_dir: &Path, config_templates_dir: &Path) -> Result<()> {
        // Copy overview.hbs
        let overview_src = source_templates_dir.join("overview.hbs");
        if overview_src.exists() {
            let overview_dst = config_templates_dir.join("overview.hbs");
            fs::copy(&overview_src, &overview_dst)?;
            println!("✓ Copied overview template");
        }

        Ok(())
    }

    /// Copy methodology templates and configs.
    ///
    /// For each methodology specified in the configuration, this function
    /// copies all methodology files to the template-assets/{methodology}/ directory
    /// for use by the template engine and methodology registry.
    ///
    /// # Arguments
    /// * `source_templates_dir` - Path to the source templates directory
    /// * `config` - Project configuration containing methodology list
    /// * `config_templates_dir` - Path to the destination templates directory
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The source methodologies directory is missing
    /// - A specified methodology directory is missing
    /// - Template files cannot be copied
    fn copy_methodologies(
        source_templates_dir: &Path,
        config: &Config,
        config_templates_dir: &Path,
    ) -> Result<()> {
        let source_methodologies = source_templates_dir.join("methodologies");
        if !source_methodologies.exists() {
            anyhow::bail!(
                "Source methodologies directory not found at {:?}",
                source_methodologies
            );
        }

        for methodology in &config.templates.methodologies {
            let source_method_dir = source_methodologies.join(methodology);
            if !source_method_dir.exists() {
                anyhow::bail!(
                    "Methodology '{}' not found in source-templates/methodologies/. \
                     Available methodologies should be in source-templates/methodologies/{{name}}/ directories.",
                    methodology
                );
            }

            // Validate that required files exist
            let config_file = source_method_dir.join("config.toml");
            if !config_file.exists() {
                anyhow::bail!(
                    "Methodology '{}' is missing config.toml file in {:?}",
                    methodology,
                    source_method_dir
                );
            }

            let info_file = source_method_dir.join("info.toml");
            if !info_file.exists() {
                anyhow::bail!(
                    "Methodology '{}' is missing info.toml file in {:?}",
                    methodology,
                    source_method_dir
                );
            }

            // Copy methodology templates to template-assets/{methodology}/
            let target_method_templates = config_templates_dir.join(methodology);
            Self::copy_dir_recursive(&source_method_dir, &target_method_templates)?;

            println!("✓ Copied methodology: {}", methodology);
        }

        Ok(())
    }

    /// Copy language templates.
    ///
    /// Copies test templates for the configured test language. Language templates
    /// are optional - if the specified language is not found, the function
    /// completes successfully but logs a warning.
    ///
    /// # Arguments
    /// * `source_templates_dir` - Path to the source templates directory
    /// * `config` - Project configuration containing test language
    /// * `config_templates_dir` - Path to the destination templates directory
    ///
    /// # Errors
    /// Returns an error if template files cannot be copied (but not if language is missing).
    fn copy_language_templates(
        source_templates_dir: &Path,
        config: &Config,
        config_templates_dir: &Path,
    ) -> Result<()> {
        let source_languages = source_templates_dir.join("languages");
        if !source_languages.exists() {
            return Ok(()); // Languages are optional
        }

        let source_lang_dir = source_languages.join(&config.templates.test_language);
        if source_lang_dir.exists() {
            let target_languages = config_templates_dir.join("languages");
            let target_lang_dir = target_languages.join(&config.templates.test_language);
            Self::copy_dir_recursive(&source_lang_dir, &target_lang_dir)?;
            println!(
                "✓ Copied language templates: {}",
                config.templates.test_language
            );
        } else {
            println!(
                "⚠ Language '{}' not found in source-templates/languages/, skipping",
                config.templates.test_language
            );
        }

        Ok(())
    }

    /// Recursively copy a directory and all its contents.
    ///
    /// Creates the destination directory if it doesn't exist, then recursively
    /// copies all files and subdirectories from source to destination.
    ///
    /// # Arguments
    /// * `src` - Source directory path
    /// * `dst` - Destination directory path
    ///
    /// # Errors
    /// Returns an error if directories cannot be created or files cannot be copied.
    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_create_config_from_template() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Create config directory
        fs::create_dir_all(Config::CONFIG_DIR)?;

        let config = Config::default();
        TemplateManager::create_config_from_template(&config)?;

        // Verify config file was created
        assert!(Config::config_path().exists());

        // Verify content
        let content = fs::read_to_string(Config::config_path())?;
        let loaded_config: Config = toml::from_str(&content)?;
        assert_eq!(loaded_config.project.name, config.project.name);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_find_source_templates_dir_current_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Create source-templates directory
        fs::create_dir("source-templates")?;

        let result = TemplateManager::find_source_templates_dir()?;
        assert_eq!(result, Path::new("source-templates"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_find_source_templates_dir_manifest_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Set CARGO_MANIFEST_DIR to a different location
        let manifest_dir = temp_dir.path().join("manifest");
        let expected_path = manifest_dir.join("source-templates");
        fs::create_dir(&manifest_dir)?;
        fs::create_dir(&expected_path)?;
        std::env::set_var("CARGO_MANIFEST_DIR", manifest_dir);

        let result = TemplateManager::find_source_templates_dir()?;
        assert_eq!(result, expected_path);

        // Clean up
        std::env::remove_var("CARGO_MANIFEST_DIR");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_find_source_templates_dir_not_found() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Remove CARGO_MANIFEST_DIR if it exists
        std::env::remove_var("CARGO_MANIFEST_DIR");

        let result = TemplateManager::find_source_templates_dir();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Source templates directory not found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_templates_to_config_missing_config() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        let result = TemplateManager::copy_templates_to_config(".");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Config file not found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_templates_to_config_invalid_config() -> Result<()> {
        let temp_dir = TempDir::new()?;
        std::env::set_current_dir(&temp_dir)?;

        // Create config directory and invalid config file
        fs::create_dir_all(Config::CONFIG_DIR)?;
        fs::write(Config::config_path(), "invalid toml content")?;

        let result = TemplateManager::copy_templates_to_config(".");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse config file"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_root_templates_with_overview() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");

        fs::create_dir(&source_dir)?;
        fs::create_dir(&dest_dir)?;

        // Create overview.hbs file
        fs::write(source_dir.join("overview.hbs"), "test content")?;

        TemplateManager::copy_root_templates(&source_dir, &dest_dir)?;

        // Verify file was copied
        assert!(dest_dir.join("overview.hbs").exists());
        assert_eq!(
            fs::read_to_string(dest_dir.join("overview.hbs"))?,
            "test content"
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_root_templates_without_overview() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");

        fs::create_dir(&source_dir)?;
        fs::create_dir(&dest_dir)?;

        // No overview.hbs file
        TemplateManager::copy_root_templates(&source_dir, &dest_dir)?;

        // Should complete successfully without copying anything
        assert!(!dest_dir.join("overview.hbs").exists());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_methodologies_missing_source_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let dest_templates = temp_dir.path().join("templates");

        fs::create_dir(&source_dir)?;
        fs::create_dir(&dest_templates)?;

        let config = Config::default();

        let result = TemplateManager::copy_methodologies(
            &source_dir,
            &config,
            &dest_templates,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Source methodologies directory not found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_methodologies_missing_methodology() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let methodologies_dir = source_dir.join("methodologies");
        let dest_templates = temp_dir.path().join("templates");

        fs::create_dir_all(&methodologies_dir)?;
        fs::create_dir(&dest_templates)?;

        let mut config = Config::default();
        config.templates.methodologies = vec!["nonexistent".to_string()];

        let result = TemplateManager::copy_methodologies(
            &source_dir,
            &config,
            &dest_templates,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Methodology 'nonexistent' not found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_methodologies_missing_config() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let methodologies_dir = source_dir.join("methodologies");
        let method_dir = methodologies_dir.join("test_method");
        let dest_templates = temp_dir.path().join("templates");

        fs::create_dir_all(&method_dir)?;
        fs::create_dir(&dest_templates)?;

        // Create a template file but no config.toml
        fs::write(method_dir.join("template.hbs"), "template content")?;

        let mut config = Config::default();
        config.templates.methodologies = vec!["test_method".to_string()];

        let result = TemplateManager::copy_methodologies(
            &source_dir,
            &config,
            &dest_templates,
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing config.toml file"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_language_templates_missing_languages_dir() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let dest_templates = temp_dir.path().join("templates");

        fs::create_dir(&source_dir)?;
        fs::create_dir(&dest_templates)?;

        let config = Config::default();

        // Should succeed even without languages directory (it's optional)
        let result =
            TemplateManager::copy_language_templates(&source_dir, &config, &dest_templates);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_language_templates_missing_language() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let languages_dir = source_dir.join("languages");
        let dest_templates = temp_dir.path().join("templates");

        fs::create_dir_all(&languages_dir)?;
        fs::create_dir(&dest_templates)?;

        let mut config = Config::default();
        config.templates.test_language = "nonexistent_lang".to_string();

        // Should succeed but log a warning (language templates are optional)
        let result =
            TemplateManager::copy_language_templates(&source_dir, &config, &dest_templates);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_copy_dir_recursive() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");

        // Create source structure
        fs::create_dir_all(source_dir.join("subdir"))?;
        fs::write(source_dir.join("file1.txt"), "content1")?;
        fs::write(source_dir.join("subdir").join("file2.txt"), "content2")?;

        TemplateManager::copy_dir_recursive(&source_dir, &dest_dir)?;

        // Verify copy
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("subdir").join("file2.txt").exists());
        assert_eq!(fs::read_to_string(dest_dir.join("file1.txt"))?, "content1");
        assert_eq!(
            fs::read_to_string(dest_dir.join("subdir").join("file2.txt"))?,
            "content2"
        );

        Ok(())
    }
}
