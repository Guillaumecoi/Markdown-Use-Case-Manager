//! Field resolution with inheritance support
//!
//! This module handles resolving custom fields for a specific methodology/level combination,
//! taking into account level inheritance (e.g., normal inherits from simple).

use super::{CustomFieldConfig, DocumentationLevel, Methodology, MethodologyDefinition};
use std::collections::{HashMap, HashSet};

/// Resolves custom fields for a specific methodology/level with inheritance
#[allow(dead_code)]
pub struct FieldResolver<'a> {
    methodology: &'a MethodologyDefinition,
}

#[allow(dead_code)]
impl<'a> FieldResolver<'a> {
    /// Create a new field resolver for a methodology
    pub fn new(methodology: &'a MethodologyDefinition) -> Self {
        Self { methodology }
    }

    /// Get all resolved fields for a specific level (including inherited fields)
    ///
    /// Fields are resolved in inheritance order, with child levels overriding parent fields.
    /// Returns a HashMap of field name → field config.
    pub fn resolve_fields_for_level(
        &self,
        level_name: &str,
    ) -> anyhow::Result<HashMap<String, CustomFieldConfig>> {
        let level = self
            .methodology
            .levels()
            .iter()
            .find(|l| l.name == level_name)
            .ok_or_else(|| anyhow::anyhow!("Level '{}' not found", level_name))?;

        // Get inheritance chain (parents first, target level last)
        let chain = self.get_inheritance_chain(level)?;

        // Build field map, later entries override earlier ones
        let mut resolved_fields = HashMap::new();

        for level_in_chain in chain {
            // Get fields defined at this level (use lowercase key to match TOML section names)
            let level_key = level_in_chain.name.to_lowercase();
            if let Some(level_config) = self.methodology.level_configs.get(&level_key) {
                for (field_name, field_config) in &level_config.custom_fields {
                    resolved_fields.insert(field_name.clone(), field_config.clone());
                }
            }
        }

        Ok(resolved_fields)
    }

    /// Get the inheritance chain for a level (parents first, target level last)
    ///
    /// Uses a depth-first traversal to build the chain, ensuring parent fields
    /// are processed before child fields.
    fn get_inheritance_chain(
        &self,
        level: &DocumentationLevel,
    ) -> anyhow::Result<Vec<DocumentationLevel>> {
        let mut chain = Vec::new();
        let mut visited = HashSet::new();
        self.build_inheritance_chain(level, &mut chain, &mut visited)?;
        Ok(chain)
    }

    /// Recursive helper to build inheritance chain
    fn build_inheritance_chain(
        &self,
        level: &DocumentationLevel,
        chain: &mut Vec<DocumentationLevel>,
        visited: &mut HashSet<String>,
    ) -> anyhow::Result<()> {
        // Detect cycles
        if visited.contains(&level.name) {
            return Err(anyhow::anyhow!(
                "Circular inheritance detected at level '{}'",
                level.name
            ));
        }
        visited.insert(level.name.clone());

        // Process parent levels first (depth-first)
        for parent_name in &level.inherits {
            let parent = self
                .methodology
                .levels()
                .iter()
                .find(|l| l.name == *parent_name)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Parent level '{}' not found for level '{}'",
                        parent_name,
                        level.name
                    )
                })?;

            self.build_inheritance_chain(parent, chain, visited)?;
        }

        // Add current level (if not already in chain)
        if !chain.iter().any(|l| l.name == level.name) {
            chain.push(level.clone());
        }

        Ok(())
    }

    /// Get list of all available levels for this methodology
    pub fn available_levels(&self) -> &[DocumentationLevel] {
        self.methodology.levels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::methodologies::Methodology;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_methodology() -> (MethodologyDefinition, TempDir) {
        // Create a test methodology with inheritance: detailed -> normal -> simple
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("test");
        fs::create_dir(&methodology_dir).unwrap();

        let methodology_toml = r#"
[methodology]
name = "test"
abbreviation = "tst"
description = "Test methodology"

[template]
preferred_style = "structured"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false

[usage]
when_to_use = ["Testing"]
key_features = ["Test feature"]

[levels.normal]
name = "Normal"
abbreviation = "n"
filename = "uc_normal.hbs"
description = "Normal level"
inherits = []

[levels.normal.custom_fields]
basic_field = { type = "string", required = true }
normal_field = { type = "string", required = false }

[levels.advanced]
name = "Advanced"
abbreviation = "a"
filename = "uc_advanced.hbs"
description = "Advanced level"
inherits = ["Normal"]

[levels.advanced.custom_fields]
basic_field = { type = "text", required = true, description = "Override from advanced" }
advanced_field = { type = "array", required = true }
detailed_field = { type = "string", required = true }
        "#;

        fs::write(methodology_dir.join("methodology.toml"), methodology_toml).unwrap();

        let methodology = MethodologyDefinition::from_toml(&methodology_dir).unwrap();
        (methodology, temp_dir)
    }

    #[test]
    fn test_resolve_simple_level() {
        let (methodology, _temp) = create_test_methodology();
        let resolver = FieldResolver::new(&methodology);

        let fields = resolver.resolve_fields_for_level("Normal").unwrap();

        assert_eq!(fields.len(), 2);
        assert!(fields.contains_key("basic_field"));
        assert!(fields.contains_key("normal_field"));
        assert_eq!(fields["basic_field"].field_type, "string");
    }

    #[test]
    fn test_resolve_normal_level_with_inheritance() {
        let (methodology, _temp) = create_test_methodology();
        let resolver = FieldResolver::new(&methodology);

        let fields = resolver.resolve_fields_for_level("Advanced").unwrap();

        assert_eq!(fields.len(), 4);
        assert!(fields.contains_key("basic_field"));
        assert!(fields.contains_key("normal_field"));
        assert!(fields.contains_key("advanced_field"));
        assert!(fields.contains_key("detailed_field"));

        // basic_field should be overridden by advanced level
        assert_eq!(fields["basic_field"].field_type, "text");
        assert_eq!(
            fields["basic_field"].description,
            Some("Override from advanced".to_string())
        );
    }

    #[test]
    fn test_resolve_detailed_level_full_chain() {
        let (methodology, _temp) = create_test_methodology();
        let resolver = FieldResolver::new(&methodology);

        let fields = resolver.resolve_fields_for_level("Advanced").unwrap();

        assert_eq!(fields.len(), 4);
        assert!(fields.contains_key("basic_field")); // From normal, overridden by advanced
        assert!(fields.contains_key("normal_field")); // From normal
        assert!(fields.contains_key("advanced_field")); // From advanced
        assert!(fields.contains_key("detailed_field")); // From advanced

        // Verify override chain worked
        assert_eq!(fields["basic_field"].field_type, "text");
    }

    #[test]
    fn test_inheritance_chain_order() {
        let (methodology, _temp) = create_test_methodology();
        let resolver = FieldResolver::new(&methodology);

        let advanced_level = methodology
            .levels()
            .iter()
            .find(|l| l.name == "Advanced")
            .unwrap();

        let chain = resolver.get_inheritance_chain(advanced_level).unwrap();

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].name, "Normal");
        assert_eq!(chain[1].name, "Advanced");
    }

    #[test]
    fn test_resolve_nonexistent_level() {
        let (methodology, _temp) = create_test_methodology();
        let resolver = FieldResolver::new(&methodology);

        let result = resolver.resolve_fields_for_level("NonExistent");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Level 'NonExistent' not found"));
    }

    #[test]
    fn test_circular_inheritance_detection() {
        // Create a methodology with circular inheritance
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("test");
        fs::create_dir(&methodology_dir).unwrap();

        let methodology_toml = r#"
[methodology]
name = "test"
abbreviation = "tst"
description = "Test"

[template]
preferred_style = "structured"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false

[usage]
when_to_use = ["Test"]
key_features = ["Test"]

[levels.a]
name = "A"
abbreviation = "a"
filename = "a.hbs"
description = "Level A"
inherits = ["B"]

[levels.a.custom_fields]
field_a = { type = "string", required = true }

[levels.b]
name = "B"
abbreviation = "b"
filename = "b.hbs"
description = "Level B"
inherits = ["A"]

[levels.b.custom_fields]
field_b = { type = "string", required = true }
        "#;

        fs::write(methodology_dir.join("methodology.toml"), methodology_toml).unwrap();

        let methodology = MethodologyDefinition::from_toml(&methodology_dir).unwrap();
        let resolver = FieldResolver::new(&methodology);
        let result = resolver.resolve_fields_for_level("A");

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Circular inheritance detected"));
    }

    #[test]
    fn test_available_levels() {
        let (methodology, _temp) = create_test_methodology();
        let resolver = FieldResolver::new(&methodology);

        let levels = resolver.available_levels();
        assert_eq!(levels.len(), 2);

        let level_names: HashSet<&str> = levels.iter().map(|l| l.name.as_str()).collect();
        assert!(level_names.contains("Normal"));
        assert!(level_names.contains("Advanced"));
    }

    #[test]
    fn test_missing_parent_level() {
        let temp_dir = TempDir::new().unwrap();
        let methodology_dir = temp_dir.path().join("test");
        fs::create_dir(&methodology_dir).unwrap();

        let methodology_toml = r#"
[methodology]
name = "test"
abbreviation = "tst"
description = "Test"

[template]
preferred_style = "structured"

[generation]
auto_generate_tests = false
overwrite_test_documentation = false

[usage]
when_to_use = ["Test"]
key_features = ["Test"]

[levels.child]
name = "Child"
abbreviation = "c"
filename = "child.hbs"
description = "Child level"
inherits = ["MissingParent"]

[levels.child.custom_fields]
field = { type = "string", required = true }
        "#;

        fs::write(methodology_dir.join("methodology.toml"), methodology_toml).unwrap();

        let methodology = MethodologyDefinition::from_toml(&methodology_dir).unwrap();
        let resolver = FieldResolver::new(&methodology);
        let result = resolver.resolve_fields_for_level("Child");

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Parent level 'MissingParent' not found"));
    }
}
