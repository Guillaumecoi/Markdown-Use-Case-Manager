//! Overview generator for project documentation.
//!
//! Handles generation of project overview documentation that summarizes all use cases.

use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::Config;
use crate::core::file_operations::FileOperations;
use crate::core::{TemplateEngine, UseCase};

/// Generator for project overview documentation.
pub struct OverviewGenerator {
    config: Config,
    file_operations: FileOperations,
    template_engine: TemplateEngine,
}

impl OverviewGenerator {
    /// Creates a new overview generator with the given configuration.
    pub fn new(config: Config) -> Self {
        let file_operations = FileOperations::new(config.clone());
        let template_engine = TemplateEngine::with_config(Some(&config));
        Self {
            config,
            file_operations,
            template_engine,
        }
    }

    /// Generates and saves the project overview file.
    ///
    /// Creates an overview document that includes:
    /// - Project name and generation date
    /// - Total use case count
    /// - Use cases grouped by category with id, title, status, and priority
    pub fn generate(&self, use_cases: &[UseCase]) -> Result<()> {
        let mut data = HashMap::new();

        // Basic counts
        data.insert("total_use_cases".to_string(), json!(use_cases.len()));

        // Project name and generated date
        data.insert("project_name".to_string(), json!(self.config.project.name));
        data.insert(
            "generated_date".to_string(),
            json!(chrono::Utc::now().format("%Y-%m-%d").to_string()),
        );

        // Group use cases by category
        let mut categories_map: HashMap<String, Vec<serde_json::Map<String, Value>>> =
            HashMap::new();
        for uc in use_cases {
            categories_map
                .entry(uc.category.clone())
                .or_default()
                .push({
                    let mut uc_data = serde_json::Map::new();
                    uc_data.insert("id".to_string(), json!(uc.id));
                    uc_data.insert("title".to_string(), json!(uc.title));
                    uc_data.insert(
                        "aggregated_status".to_string(),
                        json!(uc.status().display_name()),
                    );
                    uc_data.insert("priority".to_string(), json!(uc.priority.to_string()));
                    uc_data
                });
        }

        // Convert to array format expected by template
        let categories: Vec<serde_json::Map<String, Value>> = categories_map
            .into_iter()
            .map(|(category_name, use_cases)| {
                let mut cat = serde_json::Map::new();
                cat.insert("category_name".to_string(), json!(category_name));
                cat.insert("use_cases".to_string(), json!(use_cases));
                cat
            })
            .collect();

        data.insert("categories".to_string(), json!(categories));

        let overview_content = self.template_engine.render_overview(&data)?;
        self.file_operations.save_overview(&overview_content)?;

        Ok(())
    }
}
