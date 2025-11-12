//! # Language Infrastructure
//!
//! This module provides a modular system for defining and managing programming languages
//! supported by the Markdown Use Case Manager. Languages are defined externally in TOML
//! configuration files, making it easy to add new languages without modifying code.
//!
//! ## Architecture
//!
//! - **Language Trait**: Defines the interface that all language implementations must provide
//! - **LanguageDefinition**: The concrete implementation loaded from TOML files
//! - **LanguageRegistry**: Manages a collection of loaded languages and provides lookup
//!
//! ## Language Configuration
//!
//! Each language is defined in `source-templates/languages/{name}/info.toml`:
//!
//! ```toml
//! name = "rust"
//! aliases = ["rs"]
//! file_extension = "rs"
//! template_file = "test.hbs"
//! ```
//!
//! The `template_file` references a Handlebars template in the same directory that
//! contains the test generation template for that language.

use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a programming language supported by the system.
///
/// This trait defines the interface that all language implementations must provide.
/// Languages encapsulate metadata about programming languages and their associated
/// test generation templates.
pub trait Language {
    /// Returns the primary name of the language (e.g., "rust", "python").
    fn name(&self) -> &str;

    /// Returns alternative names/aliases for the language (e.g., ["rs"] for Rust).
    /// These can be used for flexible language lookup.
    fn aliases(&self) -> Vec<&str>;

    /// Returns the file extension typically used for this language (e.g., "rs", "py").
    /// Currently unused but reserved for future test file generation features.
    fn file_extension(&self) -> &str;

    /// Returns the Handlebars template content used for generating test files
    /// for this language.
    fn test_template(&self) -> &str;
}

/// A language definition loaded from external TOML configuration.
///
/// This struct represents a programming language that has been loaded from
/// a `source-templates/languages/{name}/info.toml` file. It contains all
/// the metadata and templates needed to support that language in the system.
///
/// Language definitions are created by reading TOML configuration files
/// and their associated template files from the filesystem.
#[derive(Debug, Clone)]
pub struct LanguageDefinition {
    /// The primary name of the language
    name: String,
    /// Alternative names/aliases for the language
    aliases: Vec<String>,
    /// File extension for this language
    file_extension: String,
    /// The test template content loaded from the template file
    test_template: String,
}

impl LanguageDefinition {
    /// Creates a new language definition by loading from a TOML configuration file.
    ///
    /// This method reads the `info.toml` file at the specified path and loads
    /// the associated template file. The `info_path` should point to an
    /// `info.toml` file in a language directory.
    ///
    /// # Arguments
    /// * `info_path` - Path to the `info.toml` file for the language
    ///
    /// # Returns
    /// A `Result` containing the loaded `LanguageDefinition` or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The TOML file cannot be read or parsed
    /// - The template file specified in the TOML cannot be found or read
    /// - The `info_path` is not a valid path
    pub fn from_toml<P: AsRef<Path>>(info_path: P) -> anyhow::Result<Self> {
        // Deserialize directly into a temporary struct for TOML data
        #[derive(serde::Deserialize)]
        struct TomlData {
            name: String,
            aliases: Vec<String>,
            file_extension: String,
            template_file: String,
        }

        let content = fs::read_to_string(&info_path)?;
        let data: TomlData = toml::from_str(&content)?;

        // Read the template file relative to the info.toml location
        let template_path = info_path
            .as_ref()
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid info.toml path: no parent directory"))?
            .join(&data.template_file);
        let test_template = fs::read_to_string(&template_path).with_context(|| {
            format!("Failed to read template file: {}", template_path.display())
        })?;

        Ok(Self {
            name: data.name,
            aliases: data.aliases,
            file_extension: data.file_extension,
            test_template,
        })
    }
}

/// Implements the Language trait for LanguageDefinition.
impl Language for LanguageDefinition {
    fn name(&self) -> &str {
        &self.name
    }

    fn aliases(&self) -> Vec<&str> {
        self.aliases.iter().map(|s| s.as_str()).collect()
    }

    fn file_extension(&self) -> &str {
        &self.file_extension
    }

    fn test_template(&self) -> &str {
        &self.test_template
    }
}

/// Registry for managing a collection of programming languages.
///
/// The `LanguageRegistry` provides a centralized way to access all supported
/// programming languages. It loads languages from the filesystem and provides
/// efficient lookup by name or alias.
///
/// Languages are stored in a case-insensitive manner, so lookups work
/// regardless of case.
pub struct LanguageRegistry {
    /// Internal storage of languages, keyed by lowercase name/alias
    languages: HashMap<String, Box<dyn Language>>,
}

impl std::fmt::Debug for LanguageRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageRegistry")
            .field("language_count", &self.languages.len())
            .field("language_names", &self.languages.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl LanguageRegistry {
    /// Creates a new registry by loading all languages from the specified templates directory.
    ///
    /// This method scans the `templates_dir/languages/` directory for language
    /// definitions. Each subdirectory containing an `info.toml` file is treated
    /// as a language definition.
    ///
    /// # Arguments
    /// * `templates_dir` - Path to the templates directory (e.g., "source-templates")
    ///
    /// # Returns
    /// A `Result` containing the loaded `LanguageRegistry` or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The languages directory cannot be read
    /// - Any language's `info.toml` file cannot be loaded
    pub fn new_dynamic<P: AsRef<Path>>(templates_dir: P) -> anyhow::Result<Self> {
        let mut languages: HashMap<String, Box<dyn Language>> = HashMap::new();
        let languages_dir = templates_dir.as_ref().join("languages");

        if languages_dir.exists() {
            for entry in fs::read_dir(&languages_dir).with_context(|| {
                format!(
                    "Failed to read languages directory: {}",
                    languages_dir.display()
                )
            })? {
                let entry = entry?;
                let info_path = entry.path().join("info.toml");

                if info_path.exists() {
                    match LanguageDefinition::from_toml(&info_path) {
                        Ok(lang) => {
                            let lang_box = Box::new(lang);
                            // Insert the primary name
                            languages.insert(lang_box.name().to_string(), lang_box.clone());
                            // Insert all aliases
                            for alias in lang_box.aliases() {
                                languages.insert(alias.to_string(), lang_box.clone());
                            }
                        }
                        Err(e) => {
                            // Log the error but continue loading other languages
                            eprintln!(
                                "Warning: Failed to load language from {}: {}",
                                info_path.display(),
                                e
                            );
                        }
                    }
                }
            }
        } else {
            anyhow::bail!("Languages directory not found: {}", languages_dir.display());
        }

        Ok(Self { languages })
    }

    /// Retrieves a language by name or alias.
    ///
    /// Performs a case-insensitive lookup for the specified language name.
    /// Both primary names and aliases are supported.
    ///
    /// # Arguments
    /// * `name` - The name or alias of the language to retrieve
    ///
    /// # Returns
    /// An `Option` containing a reference to the language if found, or `None`
    pub fn get(&self, name: &str) -> Option<&dyn Language> {
        self.languages.get(&name.to_lowercase()).map(|l| l.as_ref())
    }

    /// Returns a list of all available language names (excluding aliases).
    ///
    /// This returns only the primary names of languages, not their aliases.
    /// The list is deduplicated and represents the distinct languages available.
    ///
    /// # Returns
    /// A `Vec<String>` containing the primary names of all loaded languages
    pub fn available_languages(&self) -> Vec<String> {
        self.languages
            .keys()
            .filter(|k| {
                if let Some(lang) = self.languages.get(*k) {
                    !lang.aliases().contains(&k.as_str())
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    /// Discover available languages from a templates directory.
    ///
    /// This is a convenience method that creates a registry from the specified
    /// templates directory and returns the available language names.
    ///
    /// # Arguments
    /// * `templates_dir` - Path to the templates directory (e.g., "source-templates")
    ///
    /// # Returns
    /// A `Result` containing a vector of available language names, or an error
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The templates directory cannot be found or read
    /// - Language loading fails
    pub fn discover_available<P: AsRef<Path>>(templates_dir: P) -> anyhow::Result<Vec<String>> {
        let registry = Self::new_dynamic(templates_dir)?;
        Ok(registry.available_languages())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to create a temporary language directory with info.toml and template
    fn create_test_language(
        dir: &std::path::Path,
        name: &str,
        aliases: &[&str],
        extension: &str,
        template_content: &str,
    ) -> std::path::PathBuf {
        let lang_dir = dir.join(name);
        fs::create_dir(&lang_dir).unwrap();

        // Create info.toml with proper TOML array syntax
        let aliases_str = if aliases.is_empty() {
            "[]".to_string()
        } else {
            format!(
                "[{}]",
                aliases
                    .iter()
                    .map(|a| format!("\"{}\"", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let info_content = format!(
            r#"name = "{}"
aliases = {}
file_extension = "{}"
template_file = "test.hbs""#,
            name, aliases_str, extension
        );
        fs::write(lang_dir.join("info.toml"), info_content).unwrap();

        // Create template file
        fs::write(lang_dir.join("test.hbs"), template_content).unwrap();

        lang_dir
    }

    #[test]
    fn test_language_definition_from_toml() {
        let temp_dir = TempDir::new().unwrap();
        let lang_dir = create_test_language(
            &temp_dir.path(),
            "testlang",
            &["tl"],
            "tl",
            "template content",
        );

        let result = LanguageDefinition::from_toml(lang_dir.join("info.toml"));
        assert!(result.is_ok());

        let lang = result.unwrap();
        assert_eq!(lang.name(), "testlang");
        assert_eq!(lang.aliases(), vec!["tl"]);
        assert_eq!(lang.file_extension(), "tl");
        assert_eq!(lang.test_template(), "template content");
    }

    #[test]
    fn test_language_definition_from_toml_missing_template() {
        let temp_dir = TempDir::new().unwrap();
        let lang_dir = temp_dir.path().join("testlang");
        fs::create_dir(&lang_dir).unwrap();

        // Create info.toml but no template file
        let info_content = r#"name = "testlang"
aliases = ["tl"]
file_extension = "tl"
template_file = "test.hbs""#;
        fs::write(lang_dir.join("info.toml"), info_content).unwrap();

        let result = LanguageDefinition::from_toml(lang_dir.join("info.toml"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read template file"));
    }

    #[test]
    fn test_language_definition_from_toml_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let lang_dir = temp_dir.path().join("testlang");
        fs::create_dir(&lang_dir).unwrap();

        // Create invalid TOML
        fs::write(lang_dir.join("info.toml"), "invalid toml content").unwrap();

        let result = LanguageDefinition::from_toml(lang_dir.join("info.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_language_definition_from_toml_missing_info_file() {
        let temp_dir = TempDir::new().unwrap();

        let result = LanguageDefinition::from_toml(temp_dir.path().join("nonexistent.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_language_registry_new_dynamic() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir(&languages_dir).unwrap();

        // Create test languages
        create_test_language(&languages_dir, "lang1", &["l1"], "l1", "template1");
        create_test_language(&languages_dir, "lang2", &["l2", "alt"], "l2", "template2");

        let result = LanguageRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok());

        let registry = result.unwrap();

        // Check that languages were loaded
        assert!(registry.get("lang1").is_some());
        assert!(registry.get("lang2").is_some());

        // Check aliases work
        assert!(registry.get("l1").is_some());
        assert!(registry.get("l2").is_some());
        assert!(registry.get("alt").is_some());

        // Check that aliases point to same language
        let lang1_primary = registry.get("lang1").unwrap();
        let lang1_alias = registry.get("l1").unwrap();
        assert_eq!(lang1_primary.name(), lang1_alias.name());
    }

    #[test]
    fn test_language_registry_new_dynamic_no_languages_dir() {
        let temp_dir = TempDir::new().unwrap();

        let result = LanguageRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Languages directory not found"));
    }

    #[test]
    fn test_language_registry_new_dynamic_empty_languages_dir() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir(&languages_dir).unwrap();

        let result = LanguageRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok());

        let registry = result.unwrap();
        assert!(registry.available_languages().is_empty());
    }

    #[test]
    fn test_language_registry_get() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir(&languages_dir).unwrap();

        create_test_language(&languages_dir, "rust", &["rs"], "rs", "fn main() {}");

        let registry = LanguageRegistry::new_dynamic(&temp_dir.path()).unwrap();

        // Test primary name
        let lang = registry.get("rust").unwrap();
        assert_eq!(lang.name(), "rust");

        // Test alias
        let lang_alias = registry.get("rs").unwrap();
        assert_eq!(lang_alias.name(), "rust");

        // Test case insensitive
        let lang_lower = registry.get("RUST").unwrap();
        assert_eq!(lang_lower.name(), "rust");

        // Test nonexistent
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_language_registry_available_languages() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir(&languages_dir).unwrap();

        create_test_language(&languages_dir, "lang1", &["alias1"], "ext1", "template1");
        create_test_language(
            &languages_dir,
            "lang2",
            &["alias2", "alias2b"],
            "ext2",
            "template2",
        );

        let registry = LanguageRegistry::new_dynamic(&temp_dir.path()).unwrap();
        let available = registry.available_languages();

        // Should contain primary names only, not aliases
        assert!(available.contains(&"lang1".to_string()));
        assert!(available.contains(&"lang2".to_string()));
        assert!(!available.contains(&"alias1".to_string()));
        assert!(!available.contains(&"alias2".to_string()));
        assert!(!available.contains(&"alias2b".to_string()));
    }

    #[test]
    fn test_language_registry_with_malformed_language() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir(&languages_dir).unwrap();

        // Create a valid language
        create_test_language(&languages_dir, "valid", &["v"], "v", "valid template");

        // Create a malformed language directory (missing template file)
        let bad_lang_dir = languages_dir.join("bad");
        fs::create_dir(&bad_lang_dir).unwrap();
        fs::write(
            bad_lang_dir.join("info.toml"),
            r#"name = "bad"
aliases = ["b"]
file_extension = "b"
template_file = "test.hbs""#,
        )
        .unwrap();
        // Don't create the template file

        let result = LanguageRegistry::new_dynamic(&temp_dir.path());
        assert!(result.is_ok()); // Should succeed despite one bad language

        let registry = result.unwrap();

        // Valid language should still be loaded
        assert!(registry.get("valid").is_some());
        assert!(registry.get("v").is_some());

        // Bad language should not be loaded
        assert!(registry.get("bad").is_none());
    }

    #[test]
    fn test_language_registry_discover_available() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir(&languages_dir).unwrap();

        create_test_language(&languages_dir, "lang1", &["alias1"], "ext1", "template1");
        create_test_language(&languages_dir, "lang2", &["alias2"], "ext2", "template2");

        let result = LanguageRegistry::discover_available(&temp_dir.path());
        assert!(result.is_ok());

        let languages = result.unwrap();
        assert!(languages.contains(&"lang1".to_string()));
        assert!(languages.contains(&"lang2".to_string()));
        assert!(!languages.contains(&"alias1".to_string()));
        assert!(!languages.contains(&"alias2".to_string()));
    }
}
