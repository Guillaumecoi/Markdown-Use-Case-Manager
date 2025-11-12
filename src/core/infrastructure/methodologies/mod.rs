//! # Methodology Infrastructure
//!
//! This module provides a modular system for defining and managing methodologies
//! supported by the Markdown Use Case Manager. Methodologies are defined externally in TOML
//! configuration files, making it easy to add new methodologies without modifying code.
//!
//! ## Architecture
//!
//! - **Methodology Trait**: Defines the interface that all methodology implementations must provide
//! - **MethodologyDefinition**: The concrete implementation loaded from TOML files
//! - **MethodologyRegistry**: Manages a collection of loaded methodologies and provides lookup
//!
//! ## Methodology Configuration
//!
//! Each methodology is defined in `source-templates/methodologies/{name}/config.toml`:
//!
//! ```toml
//! [template]
//! name = "business"
//! description = "Business-focused documentation for stakeholders"
//! preferred_style = "detailed"
//!
//! [generation]
//! auto_generate_tests = false
//! overwrite_test_documentation = false
//! ```
//!
//! The methodology directory also contains Handlebars templates for different documentation styles.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Context;

/// Represents a methodology supported by the system.
///
/// This trait defines the interface that all methodology implementations must provide.
/// Methodologies encapsulate metadata about documentation methodologies and their associated
/// templates and configuration.
pub trait Methodology {
    /// Returns the primary name of the methodology (e.g., "business", "developer").
    fn name(&self) -> &str;

    /// Returns a description of the methodology.
    fn description(&self) -> &str;

    /// Returns the preferred documentation style for this methodology.
    fn preferred_style(&self) -> &str;
}

/// A methodology definition loaded from external TOML configuration.
///
/// This struct represents a methodology that has been loaded from
/// a `source-templates/methodologies/{name}/config.toml` file. It contains all
/// the metadata and configuration needed to support that methodology in the system.
///
/// Methodology definitions are created by reading TOML configuration files
/// from the filesystem.
#[derive(Debug, Clone)]
pub struct MethodologyDefinition {
    /// The primary name of the methodology
    name: String,
    /// Description of the methodology
    description: String,
    /// Preferred documentation style
    preferred_style: String,
}

impl MethodologyDefinition {
    /// Creates a methodology definition by loading from a TOML configuration file.
    ///
    /// This method reads the `config.toml` file at the specified path and
    /// deserializes it into a MethodologyDefinition.
    ///
    /// # Arguments
    /// * `config_path` - Path to the `config.toml` file for the methodology
    ///
    /// # Returns
    /// A `Result` containing the loaded `MethodologyDefinition` or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The TOML file cannot be read or parsed
    /// - Required fields are missing from the TOML
    /// - The `config_path` is not a valid path
    pub fn from_toml<P: AsRef<Path>>(config_path: P) -> anyhow::Result<Self> {
        // Deserialize directly into a temporary struct for TOML data
        #[derive(serde::Deserialize)]
        struct TomlData {
            template: TemplateConfig,
        }

        #[derive(serde::Deserialize)]
        struct TemplateConfig {
            name: String,
            description: String,
            preferred_style: String,
        }

        let content = fs::read_to_string(&config_path)
            .context("Failed to read methodology config file")?;

        let data: TomlData = toml::from_str(&content)
            .context("Failed to parse methodology config TOML")?;

        Ok(Self {
            name: data.template.name,
            description: data.template.description,
            preferred_style: data.template.preferred_style,
        })
    }
}

impl Methodology for MethodologyDefinition {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn preferred_style(&self) -> &str {
        &self.preferred_style
    }
}

/// Registry for managing methodology definitions.
///
/// Provides lookup and discovery of methodologies loaded from the filesystem.
/// Acts as a central repository for methodology metadata and configuration.
#[derive(Debug)]
pub struct MethodologyRegistry {
    /// Map of methodology name to methodology definition
    methodologies: HashMap<String, MethodologyDefinition>,
}

impl MethodologyRegistry {
    /// Creates a new methodology registry by discovering and loading all available methodologies.
    ///
    /// This method scans the `templates_dir/methodologies/` directory for methodology
    /// configuration files and loads them into the registry.
    ///
    /// # Arguments
    /// * `templates_dir` - Base directory containing the methodologies subdirectory
    ///
    /// # Returns
    /// A `Result` containing the loaded `MethodologyRegistry` or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The methodologies directory cannot be found or read
    /// - Methodology loading fails for any methodology
    pub fn new_dynamic<P: AsRef<Path>>(templates_dir: P) -> anyhow::Result<Self> {
        let methodologies_dir = templates_dir.as_ref().join("methodologies");

        if !methodologies_dir.exists() {
            return Ok(Self {
                methodologies: HashMap::new(),
            });
        }

        let mut methodologies = HashMap::new();

        for entry in fs::read_dir(&methodologies_dir)
            .context("Failed to read methodologies directory")? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(methodology_name) = path.file_name().and_then(|n| n.to_str()) {
                    let config_path = path.join("config.toml");

                    if config_path.exists() {
                        match MethodologyDefinition::from_toml(&config_path) {
                            Ok(methodology) => {
                                methodologies.insert(methodology_name.to_string(), methodology);
                            }
                            Err(e) => {
                                // Log the error but continue loading other methodologies
                                eprintln!("Warning: Failed to load methodology '{}': {}", methodology_name, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(Self { methodologies })
    }

    /// Gets a methodology by name.
    ///
    /// Performs case-insensitive lookup of methodologies by name.
    ///
    /// # Arguments
    /// * `name` - The name of the methodology to retrieve
    ///
    /// # Returns
    /// An `Option` containing the methodology if found, or `None` if not found
    pub fn get(&self, name: &str) -> Option<&MethodologyDefinition> {
        // Try exact match first
        if let Some(methodology) = self.methodologies.get(name) {
            return Some(methodology);
        }

        // Try case-insensitive match
        self.methodologies.values().find(|m| m.name().eq_ignore_ascii_case(name))
    }

    /// Returns a list of all available methodology names.
    ///
    /// Returns the names of all successfully loaded methodologies,
    /// sorted alphabetically.
    ///
    /// # Returns
    /// A vector of methodology names
    pub fn available_methodologies(&self) -> Vec<String> {
        let mut names: Vec<String> = self.methodologies.keys().cloned().collect();
        names.sort();
        names
    }

    /// Discovers available methodologies from the templates directory.
    ///
    /// This is a convenience method that creates a registry and returns
    /// the list of available methodology names.
    ///
    /// # Arguments
    /// * `templates_dir` - Base directory containing the methodologies subdirectory
    ///
    /// # Returns
    /// A `Result` containing a vector of methodology names or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The templates directory cannot be found or read
    /// - Methodology loading fails
    pub fn discover_available<P: AsRef<Path>>(templates_dir: P) -> anyhow::Result<Vec<String>> {
        let registry = Self::new_dynamic(templates_dir)?;
        Ok(registry.available_methodologies())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to create a temporary methodology directory with config.toml
    fn create_test_methodology(dir: &std::path::Path, name: &str, description: &str, preferred_style: &str) -> std::path::PathBuf {
        let methodology_dir = dir.join(name);
        fs::create_dir(&methodology_dir).unwrap();

        let config_content = format!(
            r#"[template]
name = "{}"
description = "{}"
preferred_style = "{}"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false"#,
            name, description, preferred_style
        );
        fs::write(methodology_dir.join("config.toml"), config_content).unwrap();

        methodology_dir
    }

    #[test]
    fn test_methodology_definition_from_toml() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = create_test_methodology(&temp_dir.path(), "testmethod", "Test description", "detailed");

        let result = MethodologyDefinition::from_toml(methodology_dir.join("config.toml"));
        assert!(result.is_ok());

        let methodology = result.unwrap();
        assert_eq!(methodology.name(), "testmethod");
        assert_eq!(methodology.description(), "Test description");
        assert_eq!(methodology.preferred_style(), "detailed");
    }

    #[test]
    fn test_methodology_definition_from_toml_missing_file() {
        let temp_dir = TempDir::new().unwrap();

        let result = MethodologyDefinition::from_toml(temp_dir.path().join("nonexistent.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_methodology_definition_from_toml_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("testmethod");
        fs::create_dir(&methodology_dir).unwrap();

        // Create invalid TOML
        fs::write(methodology_dir.join("config.toml"), "invalid toml content").unwrap();

        let result = MethodologyDefinition::from_toml(methodology_dir.join("config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_methodology_registry_new_dynamic() {
        let temp_dir = TempDir::new().unwrap();
        let methodologies_dir = temp_dir.path().join("methodologies");
        fs::create_dir(&methodologies_dir).unwrap();

        // Create test methodologies
        create_test_methodology(&methodologies_dir, "method1", "Description 1", "simple");
        create_test_methodology(&methodologies_dir, "method2", "Description 2", "detailed");

        let result = MethodologyRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok());

        let registry = result.unwrap();

        // Check that methodologies were loaded
        assert!(registry.get("method1").is_some());
        assert!(registry.get("method2").is_some());

        // Check that they have correct data
        let method1 = registry.get("method1").unwrap();
        assert_eq!(method1.name(), "method1");
        assert_eq!(method1.description(), "Description 1");
        assert_eq!(method1.preferred_style(), "simple");
    }

    #[test]
    fn test_methodology_registry_new_dynamic_no_methodologies_dir() {
        let temp_dir = TempDir::new().unwrap();

        let result = MethodologyRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok());

        let registry = result.unwrap();
        assert!(registry.available_methodologies().is_empty());
    }

    #[test]
    fn test_methodology_registry_get() {
        let temp_dir = TempDir::new().unwrap();
        let methodologies_dir = temp_dir.path().join("methodologies");
        fs::create_dir(&methodologies_dir).unwrap();

        create_test_methodology(&methodologies_dir, "business", "Business-focused", "detailed");

        let registry = MethodologyRegistry::new_dynamic(&temp_dir.path()).unwrap();

        // Test primary name
        let methodology = registry.get("business").unwrap();
        assert_eq!(methodology.name(), "business");

        // Test case insensitive
        let methodology_lower = registry.get("BUSINESS").unwrap();
        assert_eq!(methodology_lower.name(), "business");

        // Test nonexistent
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_methodology_registry_available_methodologies() {
        let temp_dir = TempDir::new().unwrap();
        let methodologies_dir = temp_dir.path().join("methodologies");
        fs::create_dir(&methodologies_dir).unwrap();

        create_test_methodology(&methodologies_dir, "method1", "Desc 1", "simple");
        create_test_methodology(&methodologies_dir, "method2", "Desc 2", "detailed");

        let registry = MethodologyRegistry::new_dynamic(&temp_dir.path()).unwrap();
        let available = registry.available_methodologies();

        assert!(available.contains(&"method1".to_string()));
        assert!(available.contains(&"method2".to_string()));
        assert_eq!(available.len(), 2);
    }

    #[test]
    fn test_methodology_registry_with_malformed_methodology() {
        let temp_dir = TempDir::new().unwrap();
        let methodologies_dir = temp_dir.path().join("methodologies");
        fs::create_dir(&methodologies_dir).unwrap();

        // Create a valid methodology
        create_test_methodology(&methodologies_dir, "valid", "Valid description", "simple");

        // Create a malformed methodology directory (invalid TOML)
        let bad_methodology_dir = methodologies_dir.join("bad");
        fs::create_dir(&bad_methodology_dir).unwrap();
        fs::write(bad_methodology_dir.join("config.toml"), "invalid toml").unwrap();

        let result = MethodologyRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok()); // Should succeed despite one bad methodology

        let registry = result.unwrap();

        // Valid methodology should still be loaded
        assert!(registry.get("valid").is_some());

        // Bad methodology should not be loaded
        assert!(registry.get("bad").is_none());
    }

    #[test]
    fn test_methodology_registry_discover_available() {
        let temp_dir = TempDir::new().unwrap();
        let methodologies_dir = temp_dir.path().join("methodologies");
        fs::create_dir(&methodologies_dir).unwrap();

        create_test_methodology(&methodologies_dir, "method1", "Desc 1", "simple");
        create_test_methodology(&methodologies_dir, "method2", "Desc 2", "detailed");

        let result = MethodologyRegistry::discover_available(&temp_dir.path());
        assert!(result.is_ok());

        let methodologies = result.unwrap();
        assert!(methodologies.contains(&"method1".to_string()));
        assert!(methodologies.contains(&"method2".to_string()));
        assert_eq!(methodologies.len(), 2);
    }
}