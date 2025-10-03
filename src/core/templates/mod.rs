use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    test_templates: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();

        // Register built-in templates
        handlebars.register_template_string(
            "use_case_simple",
            include_str!("../../../templates/use_case_simple.hbs"),
        )?;
        handlebars.register_template_string(
            "use_case_detailed",
            include_str!("../../../templates/use_case_detailed.hbs"),
        )?;
        handlebars.register_template_string(
            "overview",
            include_str!("../../../templates/overview.hbs"),
        )?;

        // Register language test templates using LanguageRegistry
        let mut test_templates = HashMap::new();

        use crate::core::languages::LanguageRegistry;
        let language_registry = LanguageRegistry::new();
        for language_name in language_registry.available_languages() {
            if let Some(language) = language_registry.get(&language_name) {
                let template_name = format!("{}_test", language.name());
                handlebars.register_template_string(&template_name, language.test_template())?;
                test_templates.insert(language.name().to_string(), template_name);
            }
        }

        Ok(TemplateEngine {
            handlebars,
            test_templates,
        })
    }

    pub fn with_config(_config: Option<&crate::config::Config>) -> Self {
        Self::new().unwrap()
    }

    pub fn render_overview(&self, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars
            .render("overview", data)
            .context("Failed to render overview template")
    }

    pub fn render_use_case(&self, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars
            .render("use_case_simple", data)
            .context("Failed to render use case template")
    }

    /// Render test file for a specific language
    pub fn render_test(&self, language: &str, data: &HashMap<String, Value>) -> Result<String> {
        let language_lower = language.to_lowercase();
        let template_key = self
            .test_templates
            .get(&language_lower)
            .ok_or_else(|| anyhow::anyhow!("Unsupported language: {}", language))?;

        self.handlebars
            .render(template_key, data)
            .with_context(|| format!("Failed to render {} test template", language))
    }

    /// Render individual scenario test for a specific language
    /// Check if test templates are available for a language
    pub fn has_test_template(&self, language: &str) -> bool {
        self.test_templates.contains_key(language)
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

    fn use_case_template() -> &'static str {
        r#"{{#if metadata_enabled}}---
{{#if include_id}}id: {{id}}
{{/if}}{{#if include_title}}title: {{title}}
{{/if}}{{#if include_category}}category: {{category}}
{{/if}}{{#if include_status}}status: {{status_name}}
{{/if}}{{#if include_priority}}priority: {{priority}}
{{/if}}{{#if include_created}}created: {{created_date}}
{{/if}}{{#if include_last_updated}}last_updated: {{updated_date}}
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
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

// Helper function to convert to snake_case
pub fn to_snake_case(s: &str) -> String {
    // First convert to lowercase and replace special characters with underscores
    let cleaned = s
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

    // Remove multiple consecutive underscores and clean up
    cleaned
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}
