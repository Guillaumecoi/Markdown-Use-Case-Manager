/// Collects and merges methodology-specific custom fields from multiple methodologies
///
/// This module handles:
/// - Field collection from multiple methodology:level pairs
/// - Field inheritance (simple → normal → detailed)
/// - Collision detection between methodologies
/// - Conflict warnings for standard field overlaps
/// - Validation of duplicate fields within same methodology inheritance chain
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

use crate::config::Config;
use crate::core::{CustomFieldConfig, MethodologyDefinition};

/// Represents a field collected from one or more methodologies
#[derive(Debug, Clone)]
pub struct CollectedField {
    /// Field name
    pub name: String,
    /// Field type (string, array, number, boolean)
    pub field_type: String,
    /// Display label for prompts
    pub label: String,
    /// Whether field is required
    pub required: bool,
    /// Default value if any
    pub default: Option<String>,
    /// Description/help text
    pub description: Option<String>,
    /// Which methodologies define this field
    pub methodologies: Vec<String>,
    /// Which level this field comes from (simple, normal, detailed)
    pub level: String,
}

/// Collection of fields from multiple methodologies
#[derive(Debug, Default)]
pub struct FieldCollection {
    /// All collected fields keyed by field name
    pub fields: HashMap<String, CollectedField>,
    /// Warnings generated during collection (e.g., standard field conflicts)
    pub warnings: Vec<String>,
}

/// Collects and merges methodology-specific custom fields
pub struct MethodologyFieldCollector {
    templates_dir: String,
}

impl MethodologyFieldCollector {
    /// Create a new field collector
    pub fn new() -> Result<Self> {
        let templates_dir = format!(".config/.mucm/{}/methodologies", Config::TEMPLATES_DIR);
        Ok(Self { templates_dir })
    }

    /// Collect all custom fields for given methodology views
    ///
    /// # Arguments
    /// * `views` - List of (methodology, level) pairs (e.g., [("business", "detailed"), ("feature", "simple")])
    ///
    /// # Returns
    /// FieldCollection with merged fields and any warnings
    ///
    /// # Errors
    /// - If methodology not found
    /// - If field collision detected between different methodologies
    /// - If duplicate field within same methodology inheritance chain
    pub fn collect_fields_for_views(&self, views: &[(String, String)]) -> Result<FieldCollection> {
        let mut collection = FieldCollection::default();

        // Standard extra field names (these have priority over methodology fields)
        let standard_fields = vec!["author", "reviewer", "description"];

        // Collect fields from each view
        for (methodology, level) in views {
            let methodology_fields = self.collect_fields_for_methodology(methodology, level)?;

            for (field_name, field_config) in methodology_fields {
                // Check if this conflicts with a standard field
                if standard_fields.contains(&field_name.as_str()) {
                    collection.warnings.push(format!(
                        "⚠️  Methodology '{}' defines field '{}' which conflicts with standard field. Using standard field.",
                        methodology, field_name
                    ));
                    continue; // Skip methodology field, use standard
                }

                // Check if field already exists from another methodology (collision)
                if let Some(existing_field) = collection.fields.get(&field_name) {
                    if !existing_field.methodologies.contains(methodology) {
                        // This is a collision between different methodologies
                        return Err(anyhow::anyhow!(
                            "Field name collision detected!\n\n\
                             Field '{}' is defined by multiple methodologies:\n\
                             - {} ({}): \"{}\"\n\
                             - {} ({}): \"{}\"\n\n\
                             Please either:\n\
                             1. Remove one of these methodologies from your use case views\n\
                             2. Contact the methodology maintainer to rename conflicting fields",
                            field_name,
                            existing_field.methodologies.join(", "),
                            existing_field.field_type,
                            existing_field.label,
                            methodology,
                            field_config.field_type,
                            field_config.label.as_deref().unwrap_or(&field_name)
                        ));
                    }
                } else {
                    // New field - add it
                    collection.fields.insert(
                        field_name.clone(),
                        CollectedField {
                            name: field_name.clone(),
                            field_type: field_config.field_type.clone(),
                            label: field_config
                                .label
                                .clone()
                                .unwrap_or_else(|| field_name.clone()),
                            required: field_config.required,
                            default: field_config.default.clone(),
                            description: field_config.description.clone(),
                            methodologies: vec![methodology.clone()],
                            level: level.clone(),
                        },
                    );
                }
            }
        }

        Ok(collection)
    }

    /// Collect fields for a single methodology:level pair with inheritance
    ///
    /// # Arguments
    /// * `methodology` - Methodology name (e.g., "business")
    /// * `level` - Documentation level (e.g., "detailed")
    ///
    /// # Returns
    /// HashMap of field_name -> CustomFieldConfig
    ///
    /// Includes inherited fields (simple → normal → detailed)
    fn collect_fields_for_methodology(
        &self,
        methodology: &str,
        level: &str,
    ) -> Result<HashMap<String, CustomFieldConfig>> {
        let methodology_dir = Path::new(&self.templates_dir).join(methodology);

        if !methodology_dir.exists() || !methodology_dir.join("methodology.toml").exists() {
            return Err(anyhow::anyhow!(
                "Methodology '{}' not found in {}",
                methodology,
                self.templates_dir
            ));
        }

        let methodology_def = MethodologyDefinition::from_toml(&methodology_dir)
            .context(format!("Failed to load methodology '{}'", methodology))?;

        // Get fields with inheritance
        let mut all_fields = HashMap::new();

        // Determine which levels to include based on inheritance
        // normal: just normal
        // advanced: normal + advanced
        // Backward compatibility: simple -> normal, detailed -> advanced
        let levels_to_include = match level.to_lowercase().as_str() {
            "simple" | "s" => vec!["normal"], // Backward compatibility
            "normal" | "n" => vec!["normal"],
            "detailed" | "d" => vec!["normal", "advanced"], // Backward compatibility
            "advanced" | "a" => vec!["normal", "advanced"],
            _ => vec![level], // Fallback: just use the provided level
        };

        // Collect fields from each level
        for level_name in levels_to_include {
            // Access level_configs directly from methodology_def
            if let Some(level_config) = methodology_def.level_configs.get(level_name) {
                for (field_name, field_config) in &level_config.custom_fields {
                    // Check for duplicate field within same methodology (error)
                    if all_fields.contains_key(field_name) {
                        return Err(anyhow::anyhow!(
                            "Duplicate field '{}' found in methodology '{}' inheritance chain. \
                             Field is defined in multiple levels (simple/normal/detailed). \
                             Each field should only be defined once per methodology.",
                            field_name,
                            methodology
                        ));
                    }
                    all_fields.insert(field_name.clone(), field_config.clone());
                }
            }
        }

        Ok(all_fields)
    }

    /// Apply user-provided values to field collection, converting types appropriately
    ///
    /// # Arguments
    /// * `field_collection` - The collected fields with metadata
    /// * `user_values` - User-provided values as strings (from CLI prompts)
    ///
    /// # Returns
    /// HashMap ready to be inserted into UseCase.methodology_fields
    pub fn apply_user_values(
        &self,
        field_collection: &FieldCollection,
        user_values: HashMap<String, String>,
    ) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();

        for (field_name, user_value) in user_values {
            if let Some(field) = field_collection.fields.get(&field_name) {
                // Convert string value to appropriate JSON type
                let json_value = self.convert_to_json_type(&user_value, &field.field_type);
                result.insert(field_name, json_value);
            }
        }

        // Add default values for required fields that weren't provided
        for (field_name, field) in &field_collection.fields {
            if field.required && !result.contains_key(field_name) {
                if let Some(default) = &field.default {
                    let json_value = self.convert_to_json_type(default, &field.field_type);
                    result.insert(field_name.clone(), json_value);
                } else {
                    // Required field with no default and no user value - use empty/zero
                    let json_value = match field.field_type.as_str() {
                        "array" => serde_json::Value::Array(vec![]),
                        "number" => serde_json::Value::Number(serde_json::Number::from(0)),
                        "boolean" => serde_json::Value::Bool(false),
                        _ => serde_json::Value::String(String::new()),
                    };
                    result.insert(field_name.clone(), json_value);
                }
            }
        }

        result
    }

    /// Convert string value to appropriate JSON type
    fn convert_to_json_type(&self, value: &str, field_type: &str) -> serde_json::Value {
        match field_type {
            "array" => {
                // Parse comma-separated values
                let items: Vec<String> = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                serde_json::Value::Array(items.into_iter().map(serde_json::Value::String).collect())
            }
            "number" => {
                // Try to parse as number
                if let Ok(num) = value.parse::<i64>() {
                    serde_json::Value::Number(serde_json::Number::from(num))
                } else if let Ok(num) = value.parse::<f64>() {
                    serde_json::json!(num)
                } else {
                    serde_json::Value::String(value.to_string())
                }
            }
            "boolean" => {
                let val = matches!(value.to_lowercase().as_str(), "true" | "yes" | "1");
                serde_json::Value::Bool(val)
            }
            _ => serde_json::Value::String(value.to_string()),
        }
    }
}

impl Default for MethodologyFieldCollector {
    fn default() -> Self {
        Self::new().expect("Failed to create MethodologyFieldCollector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require actual methodology config files to exist
    // They will be integration tests rather than pure unit tests

    #[test]
    fn test_convert_to_json_type_string() {
        let collector = MethodologyFieldCollector::default();
        let value = collector.convert_to_json_type("hello", "string");
        assert_eq!(value, serde_json::Value::String("hello".to_string()));
    }

    #[test]
    fn test_convert_to_json_type_array() {
        let collector = MethodologyFieldCollector::default();
        let value = collector.convert_to_json_type("item1, item2, item3", "array");
        assert_eq!(value, serde_json::json!(["item1", "item2", "item3"]));
    }

    #[test]
    fn test_convert_to_json_type_number() {
        let collector = MethodologyFieldCollector::default();
        let value = collector.convert_to_json_type("42", "number");
        assert_eq!(
            value,
            serde_json::Value::Number(serde_json::Number::from(42))
        );
    }

    #[test]
    fn test_convert_to_json_type_boolean() {
        let collector = MethodologyFieldCollector::default();
        let value = collector.convert_to_json_type("true", "boolean");
        assert_eq!(value, serde_json::Value::Bool(true));

        let value = collector.convert_to_json_type("false", "boolean");
        assert_eq!(value, serde_json::Value::Bool(false));
    }
}
