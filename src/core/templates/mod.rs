use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    test_templates: HashMap<String, String>,
    processor_registry: crate::core::processors::methodology_processor::MethodologyRegistry,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();

        // Register methodology-specific templates
        handlebars.register_template_string(
            "simple_use_case",
            include_str!("../../../templates/methodologies/simple/use_case.hbs"),
        )?;
        handlebars.register_template_string(
            "business_use_case", 
            include_str!("../../../templates/methodologies/business/use_case.hbs"),
        )?;
        handlebars.register_template_string(
            "testing_use_case",
            include_str!("../../../templates/methodologies/testing/use_case.hbs"),
        )?;
        
        // Register overview template (using simple methodology as default)
        handlebars.register_template_string(
            "overview",
            include_str!("../../../templates/methodologies/simple/overview.hbs"),
        )?;

        // Register legacy templates for backwards compatibility
        handlebars.register_template_string(
            "use_case_simple",
            include_str!("../../../templates/methodologies/simple/use_case.hbs"),
        )?;
        handlebars.register_template_string(
            "use_case_detailed",
            include_str!("../../../templates/methodologies/business/use_case.hbs"),
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

        // Initialize processor registry
        use crate::core::processors::create_default_registry;
        let processor_registry = create_default_registry();

        Ok(TemplateEngine {
            handlebars,
            test_templates,
            processor_registry,
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
        self.render_use_case_with_template("use_case_simple", data)
    }

    /// Render use case with specific template
    pub fn render_use_case_with_template(&self, template_name: &str, data: &HashMap<String, Value>) -> Result<String> {
        self.handlebars
            .render(template_name, data)
            .with_context(|| format!("Failed to render use case with template: {}", template_name))
    }

    /// Render use case with methodology-specific template
    #[allow(dead_code)]
    pub fn render_use_case_for_methodology(&self, data: &HashMap<String, Value>, methodology: &str) -> Result<String> {
        let template_name = match methodology {
            "simple" => "simple_use_case",
            "business" => "business_use_case", 
            "testing" => "testing_use_case",
            _ => return Err(anyhow::anyhow!("Unknown methodology: {}", methodology)),
        };
        
        self.render_use_case_with_template(template_name, data)
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

    /// Render use case with methodology-specific processing
    pub fn render_use_case_with_methodology(&self, data: &HashMap<String, Value>, methodology: &str) -> Result<String> {
        // Get the methodology processor
        let processor = self.processor_registry.get_processor(methodology)
            .ok_or_else(|| anyhow::anyhow!("Unknown methodology: {}", methodology))?;

        // Extract use case and scenarios from data for processing
        let use_case = self.extract_use_case_from_data(data)?;
        let context = crate::core::processors::UseCaseContext {
            use_case_id: use_case.id.clone(),
            category: use_case.category.clone(),
            business_context: std::collections::HashMap::new(),
        };

        // Process scenarios with the methodology
        let processed = processor.process_scenarios(&use_case.scenarios, &context);

        // Create enhanced template data
        let mut enhanced_data = data.clone();
        self.add_processed_scenario_data(&mut enhanced_data, &processed, processor)?;

        // Render with the enhanced data
        let template_name = format!("use_case_{}", methodology);
        if self.handlebars.get_template(&template_name).is_some() {
            self.render_use_case_with_template(&template_name, &enhanced_data)
        } else {
            // Fallback to simple template if methodology-specific template doesn't exist
            self.render_use_case_with_template("use_case_simple", &enhanced_data)
        }
    }

    /// Get information about a specific methodology
    pub fn get_methodology_info(&self, methodology_id: &str) -> Option<(String, String)> {
        self.processor_registry.get_processor(methodology_id)
            .map(|processor| (processor.display_name().to_string(), processor.description().to_string()))
    }
    
    /// Get available methodology processors
    pub fn available_methodologies(&self) -> Vec<String> {
        self.processor_registry.available_methodologies()
    }



    // Helper methods for methodology processing
    fn extract_use_case_from_data(&self, data: &HashMap<String, Value>) -> Result<crate::core::models::UseCase> {
        // Create a basic UseCase from template data
        // This is a simplified extraction - in a real scenario, we'd have richer data
        let id = data.get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("UC-TEMP-001")
            .to_string();
        
        let title = data.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Use Case")
            .to_string();
        
        let category = data.get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("General")
            .to_string();
        
        let description = data.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let priority = crate::core::models::use_case::Priority::Medium; // Default priority
        
        // Extract scenarios if available
        let mut scenarios = Vec::new();
        if let Some(scenarios_value) = data.get("scenarios") {
            if let Some(scenarios_array) = scenarios_value.as_array() {
                scenarios = scenarios_array.iter()
                    .filter_map(|s| self.value_to_scenario(s).ok())
                    .collect();
            }
        }

        let mut use_case = crate::core::models::UseCase::new(id, title, category, description, priority);
        
        // Add scenarios manually since there's no with_scenarios method
        for scenario in scenarios {
            use_case.add_scenario(scenario);
        }

        Ok(use_case)
    }

    fn value_to_scenario(&self, value: &Value) -> Result<crate::core::models::Scenario> {
        let title = value.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Scenario")
            .to_string();
        
        let description = value.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let id = value.get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("S-TEMP-001")
            .to_string();

        Ok(crate::core::models::Scenario::new(id, title, description))
    }

    fn add_processed_scenario_data(&self, data: &mut HashMap<String, Value>, processed: &crate::core::processors::ProcessedScenarios, processor: &dyn crate::core::processors::MethodologyProcessor) -> Result<()> {
        // Add methodology-specific data to template variables
        data.insert("methodology_name".to_string(), serde_json::Value::String(processor.display_name().to_string()));
        data.insert("methodology_description".to_string(), serde_json::Value::String(processor.description().to_string()));
        
        // Add processed scenario counts
        data.insert("primary_flows_count".to_string(), serde_json::Value::Number(processed.primary_flows.len().into()));
        data.insert("alternative_flows_count".to_string(), serde_json::Value::Number(processed.alternative_flows.len().into()));
        data.insert("error_flows_count".to_string(), serde_json::Value::Number(processed.error_flows.len().into()));
        
        // Add methodology-specific metadata
        data.insert("methodology_data".to_string(), serde_json::Value::Object(
            processed.methodology_data.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        ));

        Ok(())
    }

    // Public template getters for config copying
    pub fn get_overview_template() -> &'static str {
        include_str!("../../../templates/methodologies/simple/overview.hbs")
    }

    pub fn get_use_case_simple_template() -> &'static str {
        include_str!("../../../templates/methodologies/simple/use_case.hbs")
    }

    pub fn get_use_case_detailed_template() -> &'static str {
        include_str!("../../../templates/methodologies/business/use_case.hbs")
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
