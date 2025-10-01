use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::Config;

#[derive(Debug)]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    test_templates: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self::with_config(None)
    }
    
    pub fn with_config(config: Option<&Config>) -> Self {
        let mut handlebars = Handlebars::new();
        
        // Disable HTML escaping since we're generating Markdown, not HTML
        handlebars.set_strict_mode(false);
        handlebars.register_escape_fn(handlebars::no_escape);
        
        // Determine which use case template to use based on config
        let use_case_style = config
            .and_then(|c| c.templates.use_case_style.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("simple");
            
        // Use config templates directory
        let templates_dir = crate::config::Config::templates_dir();
        let use_case_template_file = match use_case_style {
            "detailed" => templates_dir.join("use_case_detailed.hbs"),
            _ => templates_dir.join("use_case_simple.hbs"), // default to simple
        };
        
        // Register core templates
        Self::register_template(&mut handlebars, "use_case", &use_case_template_file.to_string_lossy(), Self::use_case_template);
        Self::register_template(&mut handlebars, "overview", &templates_dir.join("overview.hbs").to_string_lossy(), Self::overview_template);
        
        // Initialize test templates map
        let mut test_templates = HashMap::new();
        
        // Load language-specific test templates if test generation is enabled
        if let Some(config) = config {
            if config.generation.auto_generate_tests {
                Self::load_test_templates_for_language(&mut handlebars, &mut test_templates, &config.generation.test_language, &templates_dir);
            }
        } else {
            // Default case: load all test templates
            Self::load_all_test_templates(&mut handlebars, &mut test_templates, &templates_dir);
        }
        
        Self { handlebars, test_templates }
    }
    
    /// Load test templates for a specific language
    fn load_test_templates_for_language(
        handlebars: &mut Handlebars, 
        test_templates: &mut HashMap<String, String>,
        language: &str,
        templates_dir: &Path
    ) {
        match language {
            "rust" => {
                let rust_dir = templates_dir.join("rust");
                Self::register_template(handlebars, "rust_test", &rust_dir.join("test.hbs").to_string_lossy(), Self::rust_test_template);
                test_templates.insert("rust".to_string(), "rust_test".to_string());
            },
            "python" => {
                let python_dir = templates_dir.join("python");
                Self::register_template(handlebars, "python_test", &python_dir.join("test.hbs").to_string_lossy(), Self::python_test_template);
                test_templates.insert("python".to_string(), "python_test".to_string());
            },
            _ => {
                println!("Warning: Unsupported test language '{}', skipping test template loading", language);
            }
        }
    }
    
    /// Load all available test templates (for initialization)
    fn load_all_test_templates(
        handlebars: &mut Handlebars, 
        test_templates: &mut HashMap<String, String>,
        templates_dir: &Path
    ) {
        // Load Rust templates
        let rust_dir = templates_dir.join("rust");
        Self::register_template(handlebars, "rust_test", &rust_dir.join("test.hbs").to_string_lossy(), Self::rust_test_template);
        test_templates.insert("rust".to_string(), "rust_test".to_string());
        
        // Load Python templates
        let python_dir = templates_dir.join("python");
        Self::register_template(handlebars, "python_test", &python_dir.join("test.hbs").to_string_lossy(), Self::python_test_template);
        test_templates.insert("python".to_string(), "python_test".to_string());
    }
    
    fn register_template<F>(handlebars: &mut Handlebars, name: &str, file_path: &str, fallback: F)
    where
        F: Fn() -> &'static str,
    {
        let template_content = if Path::new(file_path).exists() {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    println!("Loaded custom template: {}", file_path);
                    content
                },
                Err(_) => {
                    println!("Warning: Failed to read {}, using built-in template", file_path);
                    fallback().to_string()
                }
            }
        } else {
            fallback().to_string()
        };
        
        handlebars
            .register_template_string(name, template_content)
            .unwrap_or_else(|_| panic!("Failed to register {} template", name));
    }
    
    pub fn render_overview(&self, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars.render("overview", data)
            .context("Failed to render overview template")
    }
    
    pub fn render_use_case(&self, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars.render("use_case", data)
            .context("Failed to render use case template")
    }
    
    /// Render test file for a specific language
    pub fn render_test(&self, language: &str, data: &HashMap<String, Value>) -> Result<String> {
        let language_lower = language.to_lowercase();
        let template_key = self.test_templates.get(&language_lower)
            .ok_or_else(|| anyhow::anyhow!("Unsupported language: {}", language))?;
        
        self.handlebars.render(template_key, data)
            .with_context(|| format!("Failed to render {} test template", language))
    }
    
    /// Render individual scenario test for a specific language
    /// Check if test templates are available for a language
    pub fn has_test_template(&self, language: &str) -> bool {
        self.test_templates.contains_key(language)
    }
    
    /// Get available test languages
    pub fn get_available_test_languages(&self) -> Vec<String> {
        self.test_templates.keys()
            .cloned()
            .collect()
    }
    
    fn overview_template() -> &'static str {
        r#"# {{project_name}} - Use Cases Overview

Generated on: {{generated_date}}

{{#if summary}}
## Project Summary

{{summary}}

{{/if}}
## Statistics

- **Total Use Cases:** {{total_use_cases}}
- **Total Scenarios:** {{total_scenarios}}
- **Status Distribution:**
{{#each status_counts}}
  - {{@key}}: {{this}}
{{/each}}

{{#each categories}}
## {{category_name}} Use Cases

{{#each use_cases}}
### {{id}} - {{title}}

**Status:** {{aggregated_status}}  
**Priority:** {{priority}}  
**Description:** {{description}}

{{#if scenarios}}
**Scenarios:**
{{#each scenarios}}
- **{{id}}** - {{title}} ({{status}})
{{/each}}
{{/if}}

[📖 View Details]({{category_path}}/{{id}}.md) | [🧪 View Tests](../../tests/use-cases/{{category_path}}/{{id}}_test.rs)

---
{{/each}}

{{/each}}

---
*This overview is automatically generated. Last updated: {{generated_date}}*
"#
    }
    
    // Public template getters for config copying
    pub fn get_overview_template() -> &'static str {
        Self::overview_template()
    }
    
    pub fn get_use_case_simple_template() -> &'static str {
        Self::use_case_template()
    }
    
    pub fn get_use_case_detailed_template() -> &'static str {
        Self::use_case_detailed_template()
    }
    
    pub fn get_rust_test_template() -> &'static str {
        Self::rust_test_template()
    }
    
    pub fn get_python_test_template() -> &'static str {
        Self::python_test_template()
    }
    
    fn use_case_template() -> &'static str {
        r#"{{#if metadata_enabled}}---
{{#if include_id}}id: {{id}}
{{/if}}{{#if include_title}}title: {{title}}
{{/if}}{{#if include_category}}category: {{category}}
{{/if}}{{#if include_status}}status: {{status_name}}
{{/if}}{{#if include_priority}}priority: {{priority}}
{{/if}}{{#if include_created}}created: {{created_date}}
{{/if}}{{#if include_last_updated}}last_updated: {{updated_date}}
{{/if}}{{#if include_tags}}tags: {{#if tags}}[{{#each tags}}"{{this}}"{{#unless @last}}, {{/unless}}{{/each}}]{{else}}[]{{/if}}
{{/if}}{{#each custom_fields}}{{this}}: 
{{/each}}---

{{/if}}# {{title}}

## Description

{{description}}

## Scenarios

{{#each scenarios}}
### {{title}} ({{id}})

**Status:** {{status}}

{{description}}

---
{{/each}}"#
    }
    
    fn use_case_detailed_template() -> &'static str {
        r#"{{#if metadata_enabled}}---
{{#if include_id}}id: {{id}}
{{/if}}{{#if include_title}}title: {{title}}
{{/if}}{{#if include_category}}category: {{category}}
{{/if}}{{#if include_status}}status: {{status_name}}
{{/if}}{{#if include_priority}}priority: {{priority}}
{{/if}}{{#if include_created}}created: {{created_date}}
{{/if}}{{#if include_last_updated}}last_updated: {{updated_date}}
{{/if}}{{#if include_tags}}tags: {{#if tags}}[{{#each tags}}"{{this}}"{{#unless @last}}, {{/unless}}{{/each}}]{{else}}[]{{/if}}
{{/if}}{{#each custom_fields}}{{this}}: 
{{/each}}---

{{/if}}# {{title}}

**ID:** {{id}}  
**Category:** {{category}}  
**Priority:** {{priority}}  
**Status:** {{status_name}}  
{{#if include_created}}**Created:** {{created_date}}  {{/if}}
{{#if include_last_updated}}**Last Updated:** {{updated_date}}  {{/if}}

## Description

{{description}}

{{#if tags}}
## Tags

{{#each tags}}
- {{this}}
{{/each}}

{{/if}}
{{#if custom_fields}}
## Additional Information

{{#each custom_fields}}
**{{this}}:** <!-- TODO: Fill in -->
{{/each}}

{{/if}}
## Scenarios

{{#each scenarios}}
### {{title}} ({{id}})

**Status:** {{status}}  
**Priority:** {{priority}}

{{#if description}}
**Description:** {{description}}

{{/if}}
{{#if preconditions}}
**Preconditions:**
{{#each preconditions}}
- {{this}}
{{/each}}

{{/if}}
{{#if steps}}
**Steps:**
{{#each steps}}
1. {{this}}
{{/each}}

{{/if}}
{{#if expected_outcome}}
**Expected Outcome:** {{expected_outcome}}

{{/if}}
---
{{/each}}

{{#if include_test_file}}
## Test Information

**Test File:** `{{test_file_path}}`

{{/if}}

---
*Use Case managed with MUCM - Markdown Use Case Manager*"#
    }
    
    fn rust_test_template() -> &'static str {
        r#"// =============================================================================
// AUTO-GENERATED TEST DOCUMENTATION
// Use Case: {{title}} ({{id}})
// Description: {{description}}
// Generated at: {{generated_at}}
// =============================================================================

{{#each scenarios}}
/// ## Scenario: {{title}} ({{id}})
/// **Description:** {{description}}
/// **Status:** {{status}}
/// 
{{/each}}

// =============================================================================
// AUTO-GENERATED TEST CODE
// ⚠️  WARNING: Only modify code between START/END USER IMPLEMENTATION markers!
// =============================================================================

#[cfg(test)]
mod {{test_module_name}} {
    use super::*;

{{#each scenarios}}
    #[test]
    fn test_{{snake_case_id}}() {
        // Scenario: {{title}}
        // Description: {{description}}
        
        // =============================================================================
        // START USER IMPLEMENTATION - Feel free to modify the code below this line
        // =============================================================================
        
        // TODO: Implement test for scenario: {{title}}
        
        // Arrange
        // TODO: Set up test data and preconditions
        
        // Act
        // TODO: Execute the scenario steps
        
        // Assert
        // TODO: Verify the results
        
        panic!("Test not implemented yet");
        
        // =============================================================================
        // END USER IMPLEMENTATION - Do not modify anything below this line
        // =============================================================================
    }
    
{{/each}}
}
"#
    }
    

    
    fn python_test_template() -> &'static str {
        r#"""Generated test file for use case: {{title}}
ID: {{id}}
Generated at: {{generated_at}}
"""

import unittest


class Test{{title_snake_case}}(unittest.TestCase):
    """
    Test class for use case: {{title}}
    Description: {{description}}
    """
    
    def setUp(self):
        """Set up test fixtures before each test method."""
        pass
    
    def tearDown(self):
        """Clean up after each test method."""
        pass
    
{{#each scenarios}}
    def test_{{snake_case_id}}(self):
        """
        Test for scenario: {{title}}
        Description: {{description}}
        """
        # =============================================================================
        # START USER IMPLEMENTATION - Feel free to modify the code below this line
        # =============================================================================
        
        # Arrange
        # TODO: Set up test data and preconditions
        
        # Act
        # TODO: Execute the scenario steps
        
        # Assert
        # TODO: Verify the results
        
        self.fail("Test not implemented yet")
        
        # =============================================================================
        # END USER IMPLEMENTATION - Do not modify anything below this line
        # =============================================================================

{{/each}}

if __name__ == '__main__':
    unittest.main()
"#
    }
    

}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to convert to snake_case
pub fn to_snake_case(s: &str) -> String {
    // First convert to lowercase and replace special characters with underscores
    let cleaned = s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();
    
    // Remove multiple consecutive underscores and clean up
    cleaned.split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}