use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    test_templates: HashMap<String, String>,
    methodologies: Vec<String>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();

        // Dynamically discover and register methodologies from source-templates/methodologies/
        let methodologies_path = Path::new("source-templates/methodologies");
        let mut methodologies = Vec::new();

        if methodologies_path.exists() {
            for entry in fs::read_dir(methodologies_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    if let Some(methodology_name) = path.file_name().and_then(|n| n.to_str()) {
                        // Register templates for this methodology: uc_simple.hbs, uc_normal.hbs, uc_detailed.hbs
                        let simple_path = path.join("uc_simple.hbs");
                        let normal_path = path.join("uc_normal.hbs");
                        let detailed_path = path.join("uc_detailed.hbs");
                        
                        if simple_path.exists() {
                            let template = fs::read_to_string(&simple_path)?;
                            handlebars.register_template_string(&format!("{}-simple", methodology_name), template)?;
                        }
                        
                        if normal_path.exists() {
                            let template = fs::read_to_string(&normal_path)?;
                            handlebars.register_template_string(&format!("{}-normal", methodology_name), template)?;
                        }
                        
                        if detailed_path.exists() {
                            let template = fs::read_to_string(&detailed_path)?;
                            handlebars.register_template_string(&format!("{}-detailed", methodology_name), template)?;
                        }
                        
                        methodologies.push(methodology_name.to_string());
                    }
                }
            }
        }

        // Register general overview template (not methodology-specific)
        let overview_path = Path::new("source-templates/overview.hbs");
        if overview_path.exists() {
            let template = fs::read_to_string(overview_path)?;
            handlebars.register_template_string("overview", template)?;
        }

        // Register general persona template (not methodology-specific)
        let persona_path = Path::new("source-templates/persona.hbs");
        if persona_path.exists() {
            let template = fs::read_to_string(persona_path)?;
            handlebars.register_template_string("persona", template)?;
        }

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

    pub fn render_use_case(&self, _data: &HashMap<String, Value>) -> Result<String> {
        anyhow::bail!("No methodology specified. Use render_use_case_with_methodology() and specify a valid methodology from source-templates/methodologies/")
    }

    /// Render use case with specific template
    pub fn render_use_case_with_template(&self, template_name: &str, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars
            .render(template_name, data)
            .with_context(|| format!("Failed to render use case with template: {}", template_name))
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

    /// Render use case with methodology-specific template
    /// Simple version: just renders the template with the data - no processing
    pub fn render_use_case_with_methodology(&self, data: &HashMap<String, Value>, methodology: &str) -> Result<String> {
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

    /// Get information about a specific methodology
    pub fn get_methodology_info(&self, methodology_id: &str) -> Option<(String, String)> {
        // Check if this methodology exists
        if self.methodologies.contains(&methodology_id.to_string()) {
            // Try to read config.toml for this methodology
            let config_path = Path::new("source-templates/methodologies")
                .join(methodology_id)
                .join("config.toml");
            
            if let Ok(config_content) = fs::read_to_string(&config_path) {
                // Parse TOML to get name and description
                if let Ok(config) = toml::from_str::<toml::Value>(&config_content) {
                    let name = config.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or(methodology_id)
                        .to_string();
                    let description = config.get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    return Some((name, description));
                }
            }
            
            // Fallback if config doesn't exist or can't be parsed
            Some((methodology_id.to_string(), format!("{} methodology", methodology_id)))
        } else {
            None
        }
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
