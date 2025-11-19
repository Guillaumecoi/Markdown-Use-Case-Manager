//! Markdown generator for use case documentation.
//!
//! Handles generation of markdown documentation from use cases using templates.

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::config::Config;
use crate::core::{MethodologyView, TemplateEngine, UseCase};

/// Generator for use case markdown documentation.
pub struct MarkdownGenerator {
    config: Config,
    template_engine: TemplateEngine,
}

impl MarkdownGenerator {
    /// Creates a new markdown generator with the given configuration.
    pub fn new(config: Config) -> Self {
        let template_engine = TemplateEngine::with_config(Some(&config));
        Self {
            config,
            template_engine,
        }
    }

    /// Generates markdown for a use case using the default methodology.
    pub fn generate(&self, use_case: &UseCase) -> Result<String> {
        let default_methodology = &self.config.templates.default_methodology;
        self.generate_with_methodology(use_case, default_methodology)
    }

    /// Generates markdown for a use case with a specific methodology.
    ///
    /// Converts UseCase to JSON and passes it directly to the template engine.
    /// This allows templates to access ANY field from the TOML file without hardcoding.
    pub fn generate_with_methodology(
        &self,
        use_case: &UseCase,
        methodology: &str,
    ) -> Result<String> {
        // Convert UseCase directly to JSON - templates can access any field from TOML
        let use_case_json = serde_json::to_value(use_case)?;

        // Convert to HashMap for template engine compatibility
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json)?;

        // Merge extra fields into top-level HashMap so templates can access them directly
        if let Some(Value::Object(extra_map)) = data.remove("extra") {
            for (key, value) in extra_map {
                data.insert(key, value);
            }
        }

        // Merge methodology_fields for the SPECIFIC methodology into top-level HashMap
        // This flattens methodology_fields.{current_methodology}.{field} -> {field}
        if let Some(Value::Object(methodology_fields_map)) = data.remove("methodology_fields") {
            if let Some(fields) = methodology_fields_map.get(methodology) {
                if let Value::Object(field_map) = fields {
                    for (field_name, field_value) in field_map {
                        // Only insert if not already present (standard fields take priority)
                        data.entry(field_name.clone())
                            .or_insert(field_value.clone());
                    }
                }
            }
        }

        self.template_engine
            .render_use_case_with_methodology(&data, methodology)
    }

    /// Generates markdown for a use case with a specific view (methodology + level).
    ///
    /// This is used for multi-view use cases where each view needs its own markdown file.
    /// The view specifies both the methodology and the level to use.
    ///
    /// # Arguments
    /// * `use_case` - The use case to generate markdown for
    /// * `view` - The methodology view (methodology + level combination)
    ///
    /// # Returns
    /// The generated markdown content for this specific view
    pub fn generate_with_view(&self, use_case: &UseCase, view: &MethodologyView) -> Result<String> {
        // Convert UseCase to JSON and flatten methodology fields
        let use_case_json = serde_json::to_value(use_case)?;
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json)?;

        // Merge extra fields into top-level HashMap
        if let Some(Value::Object(extra_map)) = data.remove("extra") {
            for (key, value) in extra_map {
                data.insert(key, value);
            }
        }

        // Merge methodology_fields for the SPECIFIC methodology into top-level HashMap
        if let Some(Value::Object(methodology_fields_map)) = data.remove("methodology_fields") {
            if let Some(fields) = methodology_fields_map.get(&view.methodology) {
                if let Value::Object(field_map) = fields {
                    for (field_name, field_value) in field_map {
                        data.entry(field_name.clone())
                            .or_insert(field_value.clone());
                    }
                }
            }
        }

        // Render with methodology and level
        self.template_engine
            .render_use_case_with_methodology_and_level(&data, &view.methodology, &view.level)
    }

    /// Generates all markdown outputs for a use case.
    ///
    /// For single-view use cases (no views defined), generates one markdown using default methodology.
    /// For multi-view use cases, generates one markdown per enabled view.
    ///
    /// Returns a vector of (content, view) tuples where view is None for single-view use cases.
    #[allow(dead_code)]
    pub fn generate_all(
        &self,
        use_case: &UseCase,
    ) -> Result<Vec<(String, Option<MethodologyView>)>> {
        if !use_case.is_multi_view() {
            // Single view: use default methodology
            let content = self.generate(use_case)?;
            Ok(vec![(content, None)])
        } else {
            // Multi-view: generate for each enabled view
            use_case
                .enabled_views()
                .map(|view| {
                    let content = self.generate_with_view(use_case, &view)?;
                    Ok((content, Some(view.clone())))
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::UseCase;
    use serde_json::json;

    #[test]
    fn test_methodology_fields_flattening() {
        // Create a use case with methodology_fields
        let mut use_case = UseCase::new(
            "UC-TEST-001".to_string(),
            "Test Use Case".to_string(),
            "Test".to_string(),
            "Test description".to_string(),
            "Medium".to_string(),
        )
        .unwrap();

        // Add methodology fields
        let mut business_fields = std::collections::HashMap::new();
        business_fields.insert("business_value".to_string(), json!("High impact"));
        business_fields.insert(
            "stakeholders".to_string(),
            json!(["Product Manager", "Developer"]),
        );

        let mut feature_fields = std::collections::HashMap::new();
        feature_fields.insert("acceptance_criteria".to_string(), json!(["Must work"]));

        use_case
            .methodology_fields
            .insert("business".to_string(), business_fields);
        use_case
            .methodology_fields
            .insert("feature".to_string(), feature_fields);

        // Convert to JSON and flatten
        let use_case_json = serde_json::to_value(&use_case).unwrap();
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json).unwrap();

        // Apply flattening logic (same as in generate_with_methodology)
        if let Some(Value::Object(methodology_fields_map)) = data.remove("methodology_fields") {
            for (_methodology_name, fields) in methodology_fields_map {
                if let Value::Object(field_map) = fields {
                    for (field_name, field_value) in field_map {
                        data.entry(field_name).or_insert(field_value);
                    }
                }
            }
        }

        // Verify fields are accessible at top level
        assert!(data.contains_key("business_value"));
        assert_eq!(data["business_value"], json!("High impact"));
        assert!(data.contains_key("stakeholders"));
        assert_eq!(
            data["stakeholders"],
            json!(["Product Manager", "Developer"])
        );
        assert!(data.contains_key("acceptance_criteria"));
        assert_eq!(data["acceptance_criteria"], json!(["Must work"]));
    }

    #[test]
    fn test_standard_fields_take_priority_over_methodology_fields() {
        // Create a use case with both extra and methodology_fields
        let mut use_case = UseCase::new(
            "UC-TEST-001".to_string(),
            "Test Use Case".to_string(),
            "Test".to_string(),
            "Test description".to_string(),
            "Medium".to_string(),
        )
        .unwrap();

        // Add standard extra field
        use_case
            .extra
            .insert("author".to_string(), json!("Standard Author"));

        // Add methodology field with same name
        let mut business_fields = std::collections::HashMap::new();
        business_fields.insert("author".to_string(), json!("Methodology Author"));

        use_case
            .methodology_fields
            .insert("business".to_string(), business_fields);

        // Convert to JSON and flatten
        let use_case_json = serde_json::to_value(&use_case).unwrap();
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json).unwrap();

        // Apply flattening logic (extra first, then methodology_fields)
        if let Some(Value::Object(extra_map)) = data.remove("extra") {
            for (key, value) in extra_map {
                data.insert(key, value);
            }
        }

        if let Some(Value::Object(methodology_fields_map)) = data.remove("methodology_fields") {
            for (_methodology_name, fields) in methodology_fields_map {
                if let Value::Object(field_map) = fields {
                    for (field_name, field_value) in field_map {
                        data.entry(field_name).or_insert(field_value);
                    }
                }
            }
        }

        // Verify standard field takes priority
        assert_eq!(data["author"], json!("Standard Author"));
    }
}
