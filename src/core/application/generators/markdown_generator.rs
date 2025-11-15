//! Markdown generator for use case documentation.
//!
//! Handles generation of markdown documentation from use cases using templates.

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::config::Config;
use crate::core::{TemplateEngine, UseCase};

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

        self.template_engine
            .render_use_case_with_methodology(&data, methodology)
    }
}
