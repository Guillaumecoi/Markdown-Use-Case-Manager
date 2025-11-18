//! Methodology definition implementation.
//!
//! Provides concrete implementation of the Methodology trait loaded from TOML files.

use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::r#trait::{DocumentationLevel, Methodology};
use super::types::CustomFieldConfig;

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
    /// Custom fields specific to this methodology
    custom_fields: HashMap<String, CustomFieldConfig>,
}

impl MethodologyDefinition {
    /// Creates a methodology definition by loading from TOML configuration files.
    ///
    /// This method reads the `info.toml` and `config.toml` files from the specified
    /// methodology directory and deserializes them into a MethodologyDefinition.
    /// The `info.toml` provides user-facing information for methodology selection,
    /// while `config.toml` contains the technical configuration with nested level definitions.
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
            #[serde(default)]
            methodology: Option<MethodologyMeta>,
            overview: OverviewConfig,
            usage: UsageConfig,
            #[serde(default)]
            levels: HashMap<String, DocumentationLevel>, // New: nested levels format
        }

        #[derive(serde::Deserialize)]
        struct MethodologyMeta {
            name: String,
            abbreviation: String,
            description: String,
        }

        #[derive(serde::Deserialize)]
        struct OverviewConfig {
            title: String,
            #[serde(default)]
            description: Option<String>, // Optional - can come from [methodology] section
        }

        #[derive(serde::Deserialize)]
        struct UsageConfig {
            when_to_use: Vec<String>,
            key_features: Vec<String>,
        }

        let info_path = methodology_dir.join("info.toml");
        let info_content =
            fs::read_to_string(&info_path).context("Failed to read methodology info file")?;
        let info_data: InfoData =
            toml::from_str(&info_content).context("Failed to parse methodology info TOML")?;

        // Convert HashMap<String, DocumentationLevel> to Vec<DocumentationLevel>
        let levels: Vec<DocumentationLevel> = info_data.levels.into_values().collect();

        // Load config.toml for technical configuration
        #[derive(serde::Deserialize)]
        struct ConfigData {
            template: TemplateConfig,
            #[serde(default)]
            custom_fields: HashMap<String, CustomFieldConfig>, // Legacy flat format (deprecated)
            #[serde(default)]
            levels: HashMap<String, LevelConfig>, // New: nested levels with custom_fields
        }

        #[derive(serde::Deserialize)]
        struct LevelConfig {
            #[serde(default)]
            custom_fields: HashMap<String, CustomFieldConfig>,
        }

        #[derive(serde::Deserialize)]
        struct TemplateConfig {
            name: String,
            preferred_style: String,
        }

        let config_path = methodology_dir.join("config.toml");
        let config_content =
            fs::read_to_string(&config_path).context("Failed to read methodology config file")?;
        let config_data: ConfigData =
            toml::from_str(&config_content).context("Failed to parse methodology config TOML")?;

        // For now, flatten custom_fields from all levels for backward compatibility
        // TODO: In Sprint 2, implement proper level-based field resolution
        let mut all_custom_fields = config_data.custom_fields; // Start with legacy flat fields
        for (_level_name, level_config) in config_data.levels {
            all_custom_fields.extend(level_config.custom_fields);
        }

        let methodology_name = config_data.template.name;

        // Use description from [methodology] section if available, otherwise fall back to [overview]
        let description = if let Some(ref meta) = info_data.methodology {
            meta.description.clone()
        } else {
            info_data.overview.description.unwrap_or_default()
        };

        Ok(Self {
            name: methodology_name,
            title: info_data.overview.title,
            description,
            when_to_use: info_data.usage.when_to_use,
            key_features: info_data.usage.key_features,
            levels,
            preferred_style: config_data.template.preferred_style,
            custom_fields: all_custom_fields,
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

    fn custom_fields(&self) -> &HashMap<String, CustomFieldConfig> {
        &self.custom_fields
    }
}

#[cfg(test)]
mod tests {
    use super::super::r#trait::Methodology;
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

        // New format with nested levels
        let info_content = format!(
            r#"[methodology]
name = "{}"
abbreviation = "test"
description = "{}"

[overview]
title = "{}"

[usage]
when_to_use = [
    "Use case 1",
    "Use case 2"
]
key_features = [
    "Feature 1",
    "Feature 2"
]

[levels.simple]
name = "Simple"
abbreviation = "s"
filename = "uc_simple.hbs"
description = "Basic level"
inherits = []

[levels.detailed]
name = "Detailed"
abbreviation = "d"
filename = "uc_detailed.hbs"
description = "Detailed level"
inherits = ["simple"]
"#,
            name, description, title
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
        
        // Find levels by name (order not guaranteed from HashMap)
        let simple_level = methodology.levels().iter().find(|l| l.name == "Simple").expect("simple level");
        let detailed_level = methodology.levels().iter().find(|l| l.name == "Detailed").expect("detailed level");
        
        assert_eq!(simple_level.name, "Simple");
        assert_eq!(simple_level.abbreviation, "s");
        assert_eq!(simple_level.filename, "uc_simple.hbs");
        assert_eq!(simple_level.inherits, Vec::<String>::new());
        
        assert_eq!(detailed_level.name, "Detailed");
        assert_eq!(detailed_level.abbreviation, "d");
        assert_eq!(detailed_level.filename, "uc_detailed.hbs");
        assert_eq!(detailed_level.inherits, vec!["simple"]);
        
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
        fs::write(
            methodology_dir.join("config.toml"),
            r#"
[template]
name = "testmethod"
preferred_style = "detailed"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false
"#,
        )
        .unwrap();

        let result = MethodologyDefinition::from_toml(&methodology_dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_methodology_with_custom_fields() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("feature");
        fs::create_dir(&methodology_dir).unwrap();

        // Create info.toml with new format
        fs::write(
            methodology_dir.join("info.toml"),
            r#"
[methodology]
name = "feature"
abbreviation = "feat"
description = "Feature-focused development methodology"

[overview]
title = "Feature Methodology"

[usage]
when_to_use = ["Feature development", "User story tracking"]
key_features = ["User stories", "Acceptance criteria", "Story points"]

[levels.simple]
name = "Simple"
abbreviation = "s"
filename = "uc_simple.hbs"
description = "Simple feature specification"
inherits = []
"#,
        )
        .unwrap();

        // Create config.toml with custom fields
        fs::write(
            methodology_dir.join("config.toml"),
            r#"
[template]
name = "feature"
preferred_style = "detailed"

[generation]
auto_generate_tests = true
overwrite_test_documentation = false

# Custom fields for feature methodology
[custom_fields.user_story]
label = "User Story"
type = "string"
required = true

[custom_fields.acceptance_criteria]
label = "Acceptance Criteria"
type = "text"
required = true

[custom_fields.story_points]
label = "Story Points"
type = "number"
required = false
default = "3"
"#,
        )
        .unwrap();

        let result = MethodologyDefinition::from_toml(&methodology_dir);
        if let Err(e) = &result {
            panic!("Failed to load methodology: {:?}", e);
        }

        let methodology = result.unwrap();
        assert_eq!(methodology.name(), "feature");

        // Test custom fields
        let custom_fields = methodology.custom_fields();
        assert_eq!(custom_fields.len(), 3);

        // Check user_story field
        let user_story = custom_fields.get("user_story").unwrap();
        assert_eq!(user_story.label, Some("User Story".to_string()));
        assert_eq!(user_story.field_type, "string");
        assert_eq!(user_story.required, true);
        assert_eq!(user_story.default, None);

        // Check acceptance_criteria field
        let acceptance_criteria = custom_fields.get("acceptance_criteria").unwrap();
        assert_eq!(
            acceptance_criteria.label,
            Some("Acceptance Criteria".to_string())
        );
        assert_eq!(acceptance_criteria.field_type, "text");
        assert_eq!(acceptance_criteria.required, true);

        // Check story_points field (with default)
        let story_points = custom_fields.get("story_points").unwrap();
        assert_eq!(story_points.label, Some("Story Points".to_string()));
        assert_eq!(story_points.field_type, "number");
        assert_eq!(story_points.required, false);
        assert_eq!(story_points.default, Some("3".to_string()));
    }

    #[test]
    fn test_methodology_without_custom_fields() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("simple");
        fs::create_dir(&methodology_dir).unwrap();

        // Create info.toml with new format
        fs::write(
            methodology_dir.join("info.toml"),
            r#"
[methodology]
name = "simple"
abbreviation = "simp"
description = "Simple methodology without custom fields"

[overview]
title = "Simple Methodology"

[usage]
when_to_use = ["Simple use cases"]
key_features = ["Basic documentation"]

[levels.simple]
name = "Simple"
abbreviation = "s"
filename = "uc_simple.hbs"
description = "Simple use case"
inherits = []
"#,
        )
        .unwrap();

        // Create config.toml without custom_fields section
        fs::write(
            methodology_dir.join("config.toml"),
            r#"
[template]
name = "simple"
preferred_style = "simple"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false
"#,
        )
        .unwrap();

        let result = MethodologyDefinition::from_toml(&methodology_dir);
        assert!(result.is_ok());

        let methodology = result.unwrap();
        assert_eq!(methodology.name(), "simple");

        // Custom fields should be empty (thanks to #[serde(default)])
        let custom_fields = methodology.custom_fields();
        assert_eq!(custom_fields.len(), 0);
    }
}
