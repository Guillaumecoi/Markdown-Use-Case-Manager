use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    /// Map of language name to template name for test generation
    /// TODO: Use this when implementing test file generation feature
    test_templates: HashMap<String, String>,
    methodologies: Vec<String>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();

        // Register custom helpers for actor and persona support
        super::helpers::register_helpers(&mut handlebars);

        // First try to load templates from user's config directory
        // Then fall back to source-templates if not found
        let user_templates_path = Path::new(".config/.mucm")
            .join(crate::config::Config::TEMPLATES_DIR)
            .join("methodologies");
        let source_templates_path = Path::new("source-templates/methodologies").to_path_buf();

        let methodologies_path = if user_templates_path.exists() {
            &user_templates_path
        } else {
            &source_templates_path
        };

        let mut methodologies = Vec::new();

        if methodologies_path.exists() {
            for entry in fs::read_dir(methodologies_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    if let Some(methodology_name) = path.file_name().and_then(|n| n.to_str()) {
                        // Register all uc_*.hbs templates for this methodology
                        // This allows users to add custom levels beyond simple/normal/detailed
                        for template_entry in fs::read_dir(&path)? {
                            let template_entry = template_entry?;
                            let template_path = template_entry.path();
                            
                            if template_path.is_file() {
                                if let Some(filename) = template_path.file_name().and_then(|n| n.to_str()) {
                                    // Only register uc_*.hbs files
                                    if filename.starts_with("uc_") && filename.ends_with(".hbs") {
                                        // Extract level name from filename (e.g., "uc_simple.hbs" -> "simple")
                                        let level_name = filename
                                            .strip_prefix("uc_")
                                            .and_then(|s| s.strip_suffix(".hbs"))
                                            .unwrap_or(filename);
                                        
                                        let template = fs::read_to_string(&template_path)?;
                                        handlebars.register_template_string(
                                            &format!("{}-{}", methodology_name, level_name),
                                            template,
                                        )?;
                                    }
                                }
                            }
                        }

                        methodologies.push(methodology_name.to_string());
                    }
                }
            }
        }

        // Register general overview template (not methodology-specific)
        // Overview.hbs is at the root of template-assets, not in methodologies subdirectory
        let overview_path = if user_templates_path.parent().is_some() && user_templates_path.parent().unwrap().exists() {
            user_templates_path.parent().unwrap().join("overview.hbs") // .config/.mucm/{TEMPLATES_DIR}/overview.hbs
        } else {
            Path::new("source-templates/overview.hbs").to_path_buf()
        };
        if overview_path.exists() {
            let template = fs::read_to_string(overview_path)?;
            handlebars.register_template_string("overview", template)?;
        } else {
            // If no overview template found, register a default one
            let default_overview_template = r#"# {{project_name}} - Use Cases Overview

Generated on {{generated_date}}

Total Use Cases: {{total_use_cases}}

## Use Cases by Category

{{#each categories}}
### {{category_name}}

{{#each use_cases}}
- **[{{id}}]** {{title}} - Priority: {{priority}}, Status: {{aggregated_status}}
{{/each}}

{{/each}}
"#;
            handlebars.register_template_string("overview", default_overview_template)?;
        }

        // Register language test templates using LanguageRegistry
        let mut test_templates = HashMap::new();

        use super::super::languages::LanguageRegistry;
        use crate::config::TemplateManager;

        // Try to load language templates, but don't fail if source templates not available
        match TemplateManager::find_source_templates_dir() {
            Ok(templates_dir) => {
                match LanguageRegistry::new_dynamic(&templates_dir) {
                    Ok(language_registry) => {
                        for language_name in language_registry.available_languages() {
                            if let Some(language) = language_registry.get(&language_name) {
                                let template_name = format!("{}_test", language.name());
                                handlebars.register_template_string(
                                    &template_name,
                                    language.test_template(),
                                )?;
                                test_templates.insert(language.name().to_string(), template_name);
                            }
                        }
                    }
                    Err(_) => {
                        // If language registry fails, continue without language templates
                        // This allows the template engine to work in test environments
                    }
                }
            }
            Err(_) => {
                // If source templates not available, continue without language templates
                // This allows the template engine to work in test environments
            }
        }

        // If no test templates were found (e.g., in test environments), provide defaults
        if test_templates.is_empty() {
            let default_languages = vec!["rust", "python", "javascript"];
            let default_test_template = r#"# Test for {{title}}

This is a generated test file for the use case: {{title}}

Use case ID: {{id}}
Category: {{category}}
Priority: {{priority}}

Description: {{description}}

Status: {{status}}

Generated at: {{generated_at}}
"#;

            for lang in default_languages {
                let template_name = format!("{}_test", lang);
                handlebars.register_template_string(&template_name, default_test_template)?;
                test_templates.insert(lang.to_string(), template_name);
            }
        }

        // If no methodologies were found (e.g., in test environments), provide defaults
        if methodologies.is_empty() {
            methodologies = vec![
                "business".to_string(),
                "developer".to_string(),
                "feature".to_string(),
                "tester".to_string(),
            ];

            // Register default templates for these methodologies
            // These are simple fallback templates that just output the use case data
            let default_template = r#"# {{title}}

**ID:** {{id}}
**Category:** {{category}}
**Priority:** {{priority}}

## Description
{{description}}

## Status
{{status}}

## Metadata
- **Created:** {{created}}
- **Last Updated:** {{last_updated}}
"#;

            for methodology in &methodologies {
                handlebars.register_template_string(
                    &format!("{}-simple", methodology),
                    default_template,
                )?;
                handlebars.register_template_string(
                    &format!("{}-normal", methodology),
                    default_template,
                )?;
                handlebars.register_template_string(
                    &format!("{}-detailed", methodology),
                    default_template,
                )?;
            }
        }

        Ok(TemplateEngine {
            handlebars,
            test_templates,
            methodologies,
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

    /// Render use case with specific template
    pub fn render_use_case_with_template(
        &self,
        template_name: &str,
        data: &HashMap<String, Value>,
    ) -> Result<String> {
        self.handlebars
            .render(template_name, data)
            .with_context(|| format!("Failed to render use case with template: {}", template_name))
    }

    /// Render test file for a specific language
    /// TODO: Call this when implementing test generation feature (auto_generate_tests config)
    #[allow(dead_code)]
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

    /// Check if test templates are available for a language
    /// TODO: Use this before calling render_test to provide helpful error messages
    #[allow(dead_code)]
    pub fn has_test_template(&self, language: &str) -> bool {
        self.test_templates.contains_key(language)
    }

    /// Render use case with methodology-specific template
    /// Simple version: just renders the template with the data - no processing
    pub fn render_use_case_with_methodology(
        &self,
        data: &HashMap<String, Value>,
        methodology: &str,
    ) -> Result<String> {
        // Check if methodology template exists
        let template_name = format!("{}-simple", methodology);
        if self.handlebars.get_template(&template_name).is_none() {
            anyhow::bail!(
                "Invalid source-templates: Methodology '{}' does not have a valid uc_simple.hbs template. \
                Check source-templates/methodologies/{}/uc_simple.hbs exists and is valid.",
                methodology, methodology
            );
        }

        self.render_use_case_with_template(&template_name, data)
    }

    /// Get available methodologies
    pub fn available_methodologies(&self) -> Vec<String> {
        self.methodologies.clone()
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
