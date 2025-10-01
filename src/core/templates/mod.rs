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
            
        let use_case_template_file = match use_case_style {
            "detailed" => "templates/use_case_detailed.hbs",
            _ => "templates/use_case_simple.hbs", // default to simple
        };
        
        // Try to load templates from external files, fall back to built-in
        Self::register_template(&mut handlebars, "use_case", use_case_template_file, Self::use_case_template);
        Self::register_template(&mut handlebars, "rust_test", "templates/rust_test.hbs", Self::rust_test_template);
        Self::register_template(&mut handlebars, "scenario_test", "templates/scenario_test.hbs", Self::scenario_test_template);
        Self::register_template(&mut handlebars, "overview", "templates/overview.hbs", Self::overview_template);
        
        Self { handlebars }
    }
    
    fn register_template<F>(handlebars: &mut Handlebars, name: &str, file_path: &str, fallback: F)
    where
        F: Fn() -> &'static str,
    {
        let template_content = if Path::new(file_path).exists() {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    println!("📄 Loaded custom template: {}", file_path);
                    content
                },
                Err(_) => {
                    println!("⚠️  Failed to read {}, using built-in template", file_path);
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
    
    pub fn render_rust_test(&self, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars.render("rust_test", data)
            .context("Failed to render rust test template")
    }
    
    pub fn render_scenario_test(&self, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars.render("scenario_test", data)
            .context("Failed to render scenario test template")
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
    
    fn rust_test_template() -> &'static str {
        r#"// Generated test file for use case: {{title}}
// ID: {{id}}
// Generated at: {{generated_at}}

#[cfg(test)]
mod {{test_module_name}} {
    use super::*;

    // Use case: {{title}}
    // Description: {{description}}
    
{{#each scenarios}}
    #[test]
    fn test_{{snake_case_id}}() {
        // Scenario: {{title}}
        // Description: {{description}}
        
        // TODO: Implement test for scenario: {{title}}
        // Expected outcome: {{expected_outcome}}
        
        // Arrange
        // TODO: Set up test data and preconditions
        {{#each preconditions}}
        // Precondition: {{this}}
        {{/each}}
        
        // Act
        // TODO: Execute the scenario steps
        {{#each steps}}
        // Step: {{this}}
        {{/each}}
        
        // Assert
        // TODO: Verify the expected outcome
        // Expected: {{expected_outcome}}
        
        panic!("Test not implemented yet");
    }
    
{{/each}}
}
"#
    }
    
    fn scenario_test_template() -> &'static str {
        r#"// Generated test file for scenario: {{scenario_title}}
// Use Case: {{use_case_title}} ({{use_case_id}})
// Scenario ID: {{scenario_id}}
// Generated at: {{generated_at}}

#[cfg(test)]
mod {{test_module_name}} {
    use super::*;

    /// Test for scenario: {{scenario_title}}
    /// Description: {{scenario_description}}
    #[test]
    fn test_{{test_module_name}}() {
        // Scenario: {{scenario_title}}
        // Use Case: {{use_case_title}}
        
        // TODO: Implement test for scenario: {{scenario_title}}
        // Expected outcome: {{expected_outcome}}
        
        // Arrange
        // TODO: Set up test data and preconditions
        {{#each preconditions}}
        // Precondition: {{this}}
        {{/each}}
        
        // Act
        // TODO: Execute the scenario steps
        {{#each steps}}
        // Step: {{this}}
        {{/each}}
        
        // Assert
        // TODO: Verify the expected outcome
        // Expected: {{expected_outcome}}
        
        panic!("Test not implemented yet");
    }
}
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