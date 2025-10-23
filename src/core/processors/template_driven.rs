use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use anyhow::{Context, Result};

/// Simple template-driven processor
/// Just reads TOML config, loads Handlebars templates, and renders with data
#[allow(dead_code)]
pub struct TemplateProcessor {
    /// Handlebars engine for rendering
    engine: handlebars::Handlebars<'static>,
    /// Loaded methodology configurations
    configs: HashMap<String, MethodologyConfig>,
}

/// Simple methodology configuration from config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyConfig {
    pub template: TemplateInfo,
    #[serde(default)]
    pub generation: GenerationOptions,
    #[serde(default)]
    pub custom_fields: HashMap<String, CustomField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    #[serde(default = "default_style")]
    pub preferred_style: String,
}

fn default_style() -> String {
    "normal".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerationOptions {
    #[serde(default)]
    pub auto_generate_tests: bool,
    #[serde(default)]
    pub overwrite_test_documentation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub required: bool,
}

#[allow(dead_code)]
impl TemplateProcessor {
    /// Create a new template processor
    pub fn new() -> Self {
        let mut engine = handlebars::Handlebars::new();
        engine.set_strict_mode(false); // Allow missing variables
        
        Self {
            engine,
            configs: HashMap::new(),
        }
    }
    
    /// Load a methodology from a directory
    /// Expects: {dir}/config.toml and {dir}/*.hbs template files
    pub fn load_methodology<P: AsRef<Path>>(&mut self, methodology_name: &str, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        
        // Load config.toml
        let config_path = dir.join("config.toml");
        let config_content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;
        
        let config: MethodologyConfig = toml::from_str(&config_content)
            .with_context(|| format!("Failed to parse config.toml for {}", methodology_name))?;
        
        // Load all .hbs template files
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("hbs") {
                let template_content = fs::read_to_string(&path)?;
                let template_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                
                // Register as "{methodology}-{template_name}"
                let full_name = format!("{}-{}", methodology_name, template_name);
                self.engine.register_template_string(&full_name, template_content)?;
            }
        }
        
        self.configs.insert(methodology_name.to_string(), config);
        Ok(())
    }
    
    /// Render a template with the given data
    /// Template name format: "{methodology}-{style}" (e.g., "developer-simple", "business-detailed")
    pub fn render(&self, template_name: &str, data: &HashMap<String, serde_json::Value>) -> Result<String> {
        self.engine.render(template_name, data)
            .with_context(|| format!("Failed to render template: {}", template_name))
    }
    
    /// Get methodology configuration
    pub fn get_config(&self, methodology: &str) -> Option<&MethodologyConfig> {
        self.configs.get(methodology)
    }
    
    /// Get all loaded methodology names
    pub fn list_methodologies(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_template_processor_creation() {
        let processor = TemplateProcessor::new();
        assert_eq!(processor.list_methodologies().len(), 0);
    }
    
    #[test]
    fn test_simple_rendering() {
        let mut processor = TemplateProcessor::new();
        
        // Register a simple template
        processor.engine.register_template_string(
            "test-simple",
            "# {{title}}\n\n{{description}}"
        ).unwrap();
        
        let mut data = HashMap::new();
        data.insert("title".to_string(), json!("Test Use Case"));
        data.insert("description".to_string(), json!("This is a test"));
        
        let result = processor.render("test-simple", &data).unwrap();
        assert!(result.contains("# Test Use Case"));
        assert!(result.contains("This is a test"));
    }
}
