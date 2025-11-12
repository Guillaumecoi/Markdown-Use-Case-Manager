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
//! Each methodology is defined in two TOML files in `source-templates/methodologies/{name}/`:
//!
//! - `info.toml`: Contains user-facing information for methodology selection and usage guidance
//! - `config.toml`: Contains technical configuration and template settings
//!
//! The `info.toml` file provides detailed descriptions and usage information to help users
//! choose the appropriate methodology, while `config.toml` contains the operational settings.
//!
//! The methodology directory also contains Handlebars templates for different documentation styles.

use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a methodology supported by the system.
///
/// This trait defines the interface that all methodology implementations must provide.
/// Methodologies encapsulate metadata about documentation methodologies and their associated
/// templates and configuration.
pub trait Methodology {
    /// Returns the primary name of the methodology (e.g., "business", "developer").
    fn name(&self) -> &str;

    /// Returns the title of the methodology for display purposes.
    fn title(&self) -> &str;

    /// Returns a description of the methodology.
    fn description(&self) -> &str;

    /// Returns when to use this methodology.
    fn when_to_use(&self) -> &[String];

    /// Returns the key features of this methodology.
    fn key_features(&self) -> &[String];

    /// Returns the available documentation levels for this methodology.
    fn levels(&self) -> &[DocumentationLevel];

    /// Returns the preferred documentation style for this methodology.
    fn preferred_style(&self) -> &str;
}

/// Represents a documentation level within a methodology.
///
/// Each methodology can have multiple levels (e.g., simple, normal, detailed)
/// with different templates and levels of detail.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DocumentationLevel {
    /// Name of the level (e.g., "simple", "normal", "detailed")
    pub name: String,
    /// Template filename for this level (e.g., "uc_simple.hbs")
    pub filename: String,
    /// Description of what this level provides
    pub description: String,
}

/// A methodology definition loaded from external TOML configuration.
///
/// This struct represents a methodology that has been loaded from
/// `source-templates/methodologies/{name}/info.toml` and `config.toml` files.
/// It contains all the metadata and configuration needed to support that methodology
/// in the system and help users choose the appropriate methodology.
///
/// Methodology definitions are created by reading TOML configuration files
/// from the filesystem.
#[derive(Debug, Clone)]
pub struct MethodologyDefinition {
    /// The primary name of the methodology
    name: String,
    /// Display title of the methodology
    title: String,
    /// Description of the methodology
    description: String,
    /// When to use this methodology
    when_to_use: Vec<String>,
    /// Key features of this methodology
    key_features: Vec<String>,
    /// Available documentation levels
    levels: Vec<DocumentationLevel>,
    /// Preferred documentation style
    preferred_style: String,
}

impl MethodologyDefinition {
    /// Creates a methodology definition by loading from TOML configuration files.
    ///
    /// This method reads the `info.toml` and `config.toml` files from the specified
    /// methodology directory and deserializes them into a MethodologyDefinition.
    /// The `info.toml` provides user-facing information for methodology selection,
    /// while `config.toml` contains the technical configuration.
    ///
    /// # Arguments
    /// * `methodology_dir` - Path to the methodology directory containing info.toml and config.toml
    ///
    /// # Returns
    /// A `Result` containing the loaded `MethodologyDefinition` or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The TOML files cannot be read or parsed
    /// - Required fields are missing from the TOML
    /// - The `methodology_dir` is not a valid path
    pub fn from_toml<P: AsRef<Path>>(methodology_dir: P) -> anyhow::Result<Self> {
        let methodology_dir = methodology_dir.as_ref();

        // Load info.toml for user-facing information
        #[derive(serde::Deserialize)]
        struct InfoData {
            overview: OverviewConfig,
            usage: UsageConfig,
            levels: Vec<DocumentationLevel>,
        }

        #[derive(serde::Deserialize)]
        struct OverviewConfig {
            title: String,
            description: String,
        }

        #[derive(serde::Deserialize)]
        struct UsageConfig {
            when_to_use: Vec<String>,
            key_features: Vec<String>,
        }

        let info_path = methodology_dir.join("info.toml");
        let info_content = fs::read_to_string(&info_path)
            .context("Failed to read methodology info file")?;
        let info_data: InfoData = toml::from_str(&info_content)
            .context("Failed to parse methodology info TOML")?;

        // Load config.toml for technical configuration
        #[derive(serde::Deserialize)]
        struct ConfigData {
            template: TemplateConfig,
        }

        #[derive(serde::Deserialize)]
        struct TemplateConfig {
            name: String,
            preferred_style: String,
        }

        let config_path = methodology_dir.join("config.toml");
        let config_content = fs::read_to_string(&config_path)
            .context("Failed to read methodology config file")?;
        let config_data: ConfigData = toml::from_str(&config_content)
            .context("Failed to parse methodology config TOML")?;

        Ok(Self {
            name: config_data.template.name,
            title: info_data.overview.title,
            description: info_data.overview.description,
            when_to_use: info_data.usage.when_to_use,
            key_features: info_data.usage.key_features,
            levels: info_data.levels,
            preferred_style: config_data.template.preferred_style,
        })
    }
}

impl Methodology for MethodologyDefinition {
    fn name(&self) -> &str {
        &self.name
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn when_to_use(&self) -> &[String] {
        &self.when_to_use
    }

    fn key_features(&self) -> &[String] {
        &self.key_features
    }

    fn levels(&self) -> &[DocumentationLevel] {
        &self.levels
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

        for entry in
            fs::read_dir(&methodologies_dir).context("Failed to read methodologies directory")?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(methodology_name) = path.file_name().and_then(|n| n.to_str()) {
                    let config_path = path.join("config.toml");

                    if config_path.exists() {
                        match MethodologyDefinition::from_toml(&path) {
                            Ok(methodology) => {
                                methodologies.insert(methodology_name.to_string(), methodology);
                            }
                            Err(e) => {
                                // Log the error but continue loading other methodologies
                                eprintln!(
                                    "Warning: Failed to load methodology '{}': {}",
                                    methodology_name, e
                                );
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
        self.methodologies
            .values()
            .find(|m| m.name().eq_ignore_ascii_case(name))
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

    /// Helper function to create a temporary methodology directory with config.toml and info.toml
    fn create_test_methodology(
        dir: &std::path::Path,
        name: &str,
        title: &str,
        description: &str,
        preferred_style: &str,
    ) -> std::path::PathBuf {
        let methodology_dir = dir.join(name);
        fs::create_dir(&methodology_dir).unwrap();

        let info_content = format!(
            r#"[overview]
title = "{}"
description = "{}"

[usage]
when_to_use = [
    "Use case 1",
    "Use case 2"
]
key_features = [
    "Feature 1",
    "Feature 2"
]

[[levels]]
name = "simple"
filename = "uc_simple.hbs"
description = "Basic level"

[[levels]]
name = "detailed"
filename = "uc_detailed.hbs"
description = "Detailed level"
"#,
            title, description
        );
        fs::write(methodology_dir.join("info.toml"), info_content).unwrap();

        let config_content = format!(
            r#"[template]
name = "{}"
preferred_style = "{}"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false"#,
            name, preferred_style
        );
        fs::write(methodology_dir.join("config.toml"), config_content).unwrap();

        methodology_dir
    }

    #[test]
    fn test_methodology_definition_from_toml() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = create_test_methodology(
            &temp_dir.path(),
            "testmethod",
            "Test Methodology",
            "Test description",
            "detailed",
        );

        let result = MethodologyDefinition::from_toml(&methodology_dir);
        assert!(result.is_ok());

        let methodology = result.unwrap();
        assert_eq!(methodology.name(), "testmethod");
        assert_eq!(methodology.title(), "Test Methodology");
        assert_eq!(methodology.description(), "Test description");
        assert_eq!(methodology.when_to_use(), &["Use case 1", "Use case 2"]);
        assert_eq!(methodology.key_features(), &["Feature 1", "Feature 2"]);
        assert_eq!(methodology.levels().len(), 2);
        assert_eq!(methodology.levels()[0].name, "simple");
        assert_eq!(methodology.levels()[0].filename, "uc_simple.hbs");
        assert_eq!(methodology.levels()[1].name, "detailed");
        assert_eq!(methodology.levels()[1].filename, "uc_detailed.hbs");
        assert_eq!(methodology.preferred_style(), "detailed");
    }

    #[test]
    fn test_methodology_definition_from_toml_missing_file() {
        let temp_dir = TempDir::new().unwrap();

        let result = MethodologyDefinition::from_toml(temp_dir.path().join("nonexistent"));
        assert!(result.is_err());
    }

    #[test]
    fn test_methodology_definition_from_toml_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("testmethod");
        fs::create_dir(&methodology_dir).unwrap();

        // Create invalid info.toml
        fs::write(methodology_dir.join("info.toml"), "invalid toml content").unwrap();

        // Create valid config.toml
        fs::write(methodology_dir.join("config.toml"), r#"
[template]
name = "testmethod"
preferred_style = "detailed"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false
"#).unwrap();

        let result = MethodologyDefinition::from_toml(&methodology_dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_methodology_registry_new_dynamic() {
        let temp_dir = TempDir::new().unwrap();
        let methodologies_dir = temp_dir.path().join("methodologies");
        fs::create_dir(&methodologies_dir).unwrap();

        // Create test methodologies
        create_test_methodology(&methodologies_dir, "method1", "Method 1", "Description 1", "simple");
        create_test_methodology(&methodologies_dir, "method2", "Method 2", "Description 2", "detailed");

        let result = MethodologyRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok());

        let registry = result.unwrap();

        // Check that methodologies were loaded
        assert!(registry.get("method1").is_some());
        assert!(registry.get("method2").is_some());

        // Check that they have correct data
        let method1 = registry.get("method1").unwrap();
        assert_eq!(method1.name(), "method1");
        assert_eq!(method1.title(), "Method 1");
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

        create_test_methodology(
            &methodologies_dir,
            "business",
            "Business Methodology",
            "Business-focused",
            "detailed",
        );

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

        create_test_methodology(&methodologies_dir, "method1", "Method 1", "Desc 1", "simple");
        create_test_methodology(&methodologies_dir, "method2", "Method 2", "Desc 2", "detailed");

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
        create_test_methodology(&methodologies_dir, "valid", "Valid Methodology", "Valid description", "simple");

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

        create_test_methodology(&methodologies_dir, "method1", "Method 1", "Desc 1", "simple");
        create_test_methodology(&methodologies_dir, "method2", "Method 2", "Desc 2", "detailed");

        let result = MethodologyRegistry::discover_available(&temp_dir.path());
        assert!(result.is_ok());

        let methodologies = result.unwrap();
        assert!(methodologies.contains(&"method1".to_string()));
        assert!(methodologies.contains(&"method2".to_string()));
        assert_eq!(methodologies.len(), 2);
    }
}
