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
    #[allow(dead_code)]
    pub fn generate_with_view(&self, use_case: &UseCase, view: &MethodologyView) -> Result<String> {
        // For now, just use the methodology
        // TODO: In next phase, use FieldResolver to get level-specific fields
        self.generate_with_methodology(use_case, &view.methodology)
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
