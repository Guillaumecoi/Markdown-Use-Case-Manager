// src/core/use_case_coordinator.rs
use crate::config::Config;
use crate::core::models::{UseCase, Status};
use crate::core::services::{FileService, UseCaseService};
use crate::core::templates::{TemplateEngine, to_snake_case};
use crate::core::languages::LanguageRegistry;
use anyhow::Result;
use colored::Colorize;
use serde_json::json;
use std::collections::HashMap;

/// Coordinates use case management operations
/// Now organized with modular helper methods for better maintainability
pub struct UseCaseCoordinator {
    config: Config,
    use_case_service: UseCaseService,
    file_service: FileService,
    template_engine: TemplateEngine,
    use_cases: Vec<UseCase>,
}

impl UseCaseCoordinator {
    pub fn new(config: Config) -> Result<Self> {
        let use_case_service = UseCaseService::new();
        let file_service = FileService::new(config.clone());
        let template_engine = TemplateEngine::with_config(Some(&config));

        let mut coordinator = Self {
            config,
            use_case_service,
            file_service,
            template_engine,
            use_cases: Vec::new(),
        };

        coordinator.use_cases = coordinator.file_service.load_use_cases()?;
        Ok(coordinator)
    }

    pub fn load() -> Result<Self> {
        let config = Config::load()?;
        Self::new(config)
    }

    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<String> {
        let use_case = self.create_use_case_internal(title, category, description)?;
        let use_case_id = use_case.id.clone();
        self.use_cases.push(use_case);
        self.generate_overview()?;
        Ok(use_case_id)
    }

    pub fn add_scenario_to_use_case(
        &mut self,
        use_case_id: String,
        title: String,
        description: Option<String>,
    ) -> Result<String> {
        // Find the use case index
        let use_case_index = self
            .use_cases
            .iter()
            .position(|uc| uc.id == use_case_id)
            .ok_or_else(|| anyhow::anyhow!("Use case {} not found", use_case_id))?;

        // Add scenario
        let scenario_id = self.use_case_service.add_scenario_to_use_case(
            &mut self.use_cases[use_case_index],
            title,
            description,
        );

        // Save and regenerate
        self.save_use_case_and_update(&self.use_cases[use_case_index].clone())?;
        self.generate_overview()?;

        Ok(scenario_id)
    }

    pub fn update_scenario_status(
        &mut self,
        scenario_id: String,
        status_str: String,
    ) -> Result<()> {
        let status = self.use_case_service.parse_status(&status_str)?;

        // Find the use case containing this scenario
        let use_case_index = self
            .use_cases
            .iter()
            .position(|uc| uc.scenarios.iter().any(|s| s.id == scenario_id))
            .ok_or_else(|| anyhow::anyhow!("Scenario {} not found", scenario_id))?;

        // Update the scenario status
        self.use_case_service.update_scenario_status(
            &mut self.use_cases[use_case_index],
            &scenario_id,
            status,
        )?;

        // Save and regenerate
        self.save_use_case_and_update(&self.use_cases[use_case_index].clone())?;
        self.generate_overview()?;

        println!("Updated scenario {} status to: {}", scenario_id, status);
        Ok(())
    }

    pub fn list_use_cases(&self) -> Result<()> {
        self.display_use_cases_list()
    }

    pub fn show_status(&self) -> Result<()> {
        self.display_project_status()
    }

    /// Get all use case IDs
    #[allow(clippy::unnecessary_wraps)]
    pub fn get_all_use_case_ids(&self) -> Result<Vec<String>> {
        Ok(self.use_cases.iter().map(|uc| uc.id.clone()).collect())
    }

    /// Get all scenario IDs for a specific use case
    pub fn get_scenario_ids_for_use_case(&self, use_case_id: &str) -> Result<Vec<String>> {
        let use_case = self
            .use_cases
            .iter()
            .find(|uc| uc.id == use_case_id)
            .ok_or_else(|| anyhow::anyhow!("Use case {} not found", use_case_id))?;

        Ok(use_case.scenarios.iter().map(|s| s.id.clone()).collect())
    }

    /// Get all categories in use
    #[allow(clippy::unnecessary_wraps)]
    pub fn get_all_categories(&self) -> Result<Vec<String>> {
        let mut categories: Vec<String> = self
            .use_cases
            .iter()
            .map(|uc| uc.category.clone())
            .collect();

        categories.sort();
        categories.dedup();
        Ok(categories)
    }

    // ========== Methodology Management ==========

    /// Create a use case with specific methodology
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
    ) -> Result<String> {
        // Validate methodology exists
        let available_methodologies = self.template_engine.available_methodologies();
        if !available_methodologies.contains(&methodology.to_string()) {
            return Err(anyhow::anyhow!(
                "Unknown methodology '{}'. Available: {:?}",
                methodology,
                available_methodologies
            ));
        }

        let use_case = self.create_use_case_internal(title, category, description)?;
        let use_case_id = use_case.id.clone();
        
        // Save the use case with methodology-specific rendering
        self.save_use_case_with_methodology(&use_case, methodology)?;
        self.use_cases.push(use_case);
        self.generate_overview()?;
        
        Ok(use_case_id)
    }

    /// List available methodologies
    pub fn list_available_methodologies(&self) -> Vec<String> {
        self.template_engine.available_methodologies()
    }

    /// Get methodology information
    pub fn get_methodology_info(&self, methodology: &str) -> Option<(String, String)> {
        self.template_engine.get_methodology_info(methodology)
    }

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: &str,
        methodology: &str,
    ) -> Result<()> {
        // Find the use case
        let use_case = self
            .use_cases
            .iter()
            .find(|uc| uc.id == use_case_id)
            .ok_or_else(|| anyhow::anyhow!("Use case {} not found", use_case_id))?
            .clone();

        // Validate methodology exists
        let available_methodologies = self.template_engine.available_methodologies();
        if !available_methodologies.contains(&methodology.to_string()) {
            return Err(anyhow::anyhow!(
                "Unknown methodology '{}'. Available: {:?}",
                methodology,
                available_methodologies
            ));
        }

        // Regenerate with new methodology
        self.save_use_case_with_methodology(&use_case, methodology)?;
        println!("✅ Regenerated {} with {} methodology", use_case_id, methodology);
        
        Ok(())
    }

    // ========== Modular Helper Methods ==========

    /// Internal helper to create use cases
    fn create_use_case_internal(
        &self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<UseCase> {
        let use_case_id = self
            .use_case_service
            .generate_unique_use_case_id(&category, &self.use_cases, &self.config.directories.use_case_dir);
        let description = description.unwrap_or_default();

        let use_case = self.use_case_service.create_use_case(
            use_case_id.clone(),
            title,
            category,
            description,
        );

        // Generate markdown and save
        let markdown_content = self.generate_use_case_markdown(&use_case)?;
        self.file_service
            .save_use_case(&use_case, &markdown_content)?;

        Ok(use_case)
    }

    /// Helper to save use case and handle test generation
    fn save_use_case_and_update(&self, use_case: &UseCase) -> Result<()> {
        // Generate markdown and save
        let markdown_content = self.generate_use_case_markdown(use_case)?;
        self.file_service.save_use_case(use_case, &markdown_content)?;

        // Generate test file if enabled
        if self.config.generation.auto_generate_tests {
            self.generate_test_file(use_case)?;
        }

        Ok(())
    }

    /// Save use case with specific methodology rendering
    fn save_use_case_with_methodology(&self, use_case: &UseCase, methodology: &str) -> Result<()> {
        // Generate methodology-specific markdown and save
        let markdown_content = self.generate_use_case_markdown_with_methodology(use_case, methodology)?;
        self.file_service.save_use_case(use_case, &markdown_content)?;

        // Generate test file if enabled
        if self.config.generation.auto_generate_tests {
            self.generate_test_file(use_case)?;
        }

        println!("💾 Saved {} with {} methodology", use_case.id, methodology);
        Ok(())
    }

    /// Helper to display use cases list
    fn display_use_cases_list(&self) -> Result<()> {
        if self.use_cases.is_empty() {
            println!("No use cases found. Create one with 'mucm create'");
            return Ok(());
        }

        println!("\n{}", "📋 Use Cases".bold().blue());
        println!("{}", "━".repeat(50));

        for use_case in &self.use_cases {
            let status_display = format!("{}", use_case.status());
            println!(
                "{} {} [{}] - {}",
                status_display,
                use_case.id.cyan(),
                use_case.category.yellow(),
                use_case.title.bold()
            );

            if !use_case.scenarios.is_empty() {
                for scenario in &use_case.scenarios {
                    println!(
                        "  └─ {} {} - {}",
                        scenario.status,
                        scenario.id.bright_black(),
                        scenario.title
                    );
                }
            }
            println!();
        }

        Ok(())
    }

    /// Helper to display project status
    fn display_project_status(&self) -> Result<()> {
        let total_use_cases = self.use_cases.len();
        let total_scenarios: usize = self.use_cases.iter().map(|uc| uc.scenarios.len()).sum();

        let mut status_counts: HashMap<Status, usize> = HashMap::new();
        for use_case in &self.use_cases {
            *status_counts.entry(use_case.status()).or_insert(0) += 1;
        }

        println!("\n{}", "📊 Project Status".bold().blue());
        println!("{}", "━".repeat(50));
        println!("Total Use Cases: {}", total_use_cases.to_string().cyan());
        println!("Total Scenarios: {}", total_scenarios.to_string().cyan());
        println!();

        for (status, count) in status_counts {
            println!("{}: {}", status, count.to_string().cyan());
        }

        Ok(())
    }

    /// Helper to generate use case markdown
    fn generate_use_case_markdown(&self, use_case: &UseCase) -> Result<String> {
        let mut data = HashMap::new();
        data.insert("id".to_string(), json!(use_case.id));
        data.insert("title".to_string(), json!(use_case.title));
        data.insert("category".to_string(), json!(use_case.category));
        data.insert("priority".to_string(), json!(use_case.priority.to_string()));
        data.insert(
            "status_name".to_string(),
            json!(use_case.status().display_name()),
        );
        data.insert("description".to_string(), json!(use_case.description));
        data.insert("scenarios".to_string(), json!(use_case.scenarios));
        data.insert("metadata".to_string(), json!(use_case.metadata));

        // Format dates nicely (YYYY-MM-DD)
        data.insert(
            "created_date".to_string(),
            json!(use_case.metadata.created_at.format("%Y-%m-%d").to_string()),
        );
        data.insert(
            "updated_date".to_string(),
            json!(use_case.metadata.updated_at.format("%Y-%m-%d").to_string()),
        );

        // Add metadata configuration
        let metadata_config = &self.config.metadata;
        data.insert(
            "metadata_enabled".to_string(),
            json!(metadata_config.enabled),
        );
        data.insert("include_id".to_string(), json!(metadata_config.include_id));
        data.insert(
            "include_title".to_string(),
            json!(metadata_config.include_title),
        );
        data.insert(
            "include_category".to_string(),
            json!(metadata_config.include_category),
        );
        data.insert(
            "include_status".to_string(),
            json!(metadata_config.include_status),
        );
        data.insert(
            "include_priority".to_string(),
            json!(metadata_config.include_priority),
        );
        data.insert(
            "include_created".to_string(),
            json!(metadata_config.include_created),
        );
        data.insert(
            "include_last_updated".to_string(),
            json!(metadata_config.include_last_updated),
        );
        
        // Create dynamic list of enabled custom fields
        let mut enabled_fields = Vec::new();
        if metadata_config.include_prerequisites { enabled_fields.push("prerequisites"); }
        if metadata_config.include_personas { enabled_fields.push("personas"); }
        if metadata_config.include_author { enabled_fields.push("author"); }
        if metadata_config.include_reviewer { enabled_fields.push("reviewer"); }
        if metadata_config.include_business_value { enabled_fields.push("business_value"); }
        if metadata_config.include_complexity { enabled_fields.push("complexity"); }
        if metadata_config.include_epic { enabled_fields.push("epic"); }
        if metadata_config.include_acceptance_criteria { enabled_fields.push("acceptance_criteria"); }
        if metadata_config.include_assumptions { enabled_fields.push("assumptions"); }
        if metadata_config.include_constraints { enabled_fields.push("constraints"); }
        
        data.insert(
            "custom_fields".to_string(),
            json!(enabled_fields),
        );

        self.template_engine.render_use_case(&data)
    }

    /// Helper to generate use case markdown with specific methodology
    fn generate_use_case_markdown_with_methodology(&self, use_case: &UseCase, methodology: &str) -> Result<String> {
        // Create data in external metadata format to match new templates
        let mut data = HashMap::new();
        
        // Core metadata section
        let mut core = HashMap::new();
        core.insert("id".to_string(), json!(use_case.id));
        core.insert("title".to_string(), json!(use_case.title));
        core.insert("category".to_string(), json!(use_case.category));
        core.insert("priority".to_string(), json!(use_case.priority.to_string()));
        core.insert("status".to_string(), json!(use_case.status().display_name()));
        core.insert("description".to_string(), json!(use_case.description));
        core.insert("created_date".to_string(), json!(use_case.metadata.created_at.format("%Y-%m-%d").to_string()));
        core.insert("last_updated".to_string(), json!(use_case.metadata.updated_at.format("%Y-%m-%d").to_string()));
        data.insert("core".to_string(), json!(core));
        
        // Stakeholders section (provide defaults for external metadata format)
        let mut stakeholders = HashMap::new();
        stakeholders.insert("primary_actor".to_string(), json!("System User"));
        stakeholders.insert("secondary_actors".to_string(), json!(Vec::<String>::new()));
        stakeholders.insert("primary_users".to_string(), json!(Vec::<String>::new()));
        stakeholders.insert("decision_makers".to_string(), json!(Vec::<String>::new()));
        data.insert("stakeholders".to_string(), json!(stakeholders));
        
        // Business section
        let mut business = HashMap::new();
        business.insert("value_proposition".to_string(), json!(None::<String>));
        business.insert("success_metrics".to_string(), json!(Vec::<String>::new()));
        business.insert("estimated_effort".to_string(), json!(None::<String>));
        business.insert("expected_roi".to_string(), json!(None::<String>));
        data.insert("business".to_string(), json!(business));
        
        // Technical section
        let mut technical = HashMap::new();
        technical.insert("architecture_requirements".to_string(), json!(Vec::<String>::new()));
        technical.insert("integration_points".to_string(), json!(Vec::<String>::new()));
        technical.insert("security_requirements".to_string(), json!(Vec::<String>::new()));
        technical.insert("performance_requirements".to_string(), json!(HashMap::<String, String>::new()));
        data.insert("technical".to_string(), json!(technical));
        
        // Testing section
        let mut testing = HashMap::new();
        testing.insert("test_strategy".to_string(), json!(None::<String>));
        testing.insert("test_scenarios".to_string(), json!(Vec::<String>::new()));
        testing.insert("automation_requirements".to_string(), json!(Vec::<String>::new()));
        testing.insert("quality_metrics".to_string(), json!(HashMap::<String, String>::new()));
        data.insert("testing".to_string(), json!(testing));
        
        // Add scenarios
        data.insert("scenarios".to_string(), json!(use_case.scenarios));
        
        // Add legacy flat fields for backward compatibility
        data.insert("id".to_string(), json!(use_case.id));
        data.insert("title".to_string(), json!(use_case.title));
        data.insert("category".to_string(), json!(use_case.category));
        data.insert("priority".to_string(), json!(use_case.priority.to_string()));
        data.insert("status_name".to_string(), json!(use_case.status().display_name()));
        data.insert("description".to_string(), json!(use_case.description));
        data.insert("created_date".to_string(), json!(use_case.metadata.created_at.format("%Y-%m-%d").to_string()));
        data.insert("updated_date".to_string(), json!(use_case.metadata.updated_at.format("%Y-%m-%d").to_string()));

        // Add metadata configuration for legacy compatibility
        let metadata_config = &self.config.metadata;
        data.insert("metadata_enabled".to_string(), json!(metadata_config.enabled));
        data.insert("include_id".to_string(), json!(metadata_config.include_id));
        data.insert(
            "include_title".to_string(),
            json!(metadata_config.include_title),
        );
        data.insert(
            "include_category".to_string(),
            json!(metadata_config.include_category),
        );
        data.insert(
            "include_status".to_string(),
            json!(metadata_config.include_status),
        );
        data.insert(
            "include_priority".to_string(),
            json!(metadata_config.include_priority),
        );
        data.insert(
            "include_created".to_string(),
            json!(metadata_config.include_created),
        );
        data.insert(
            "include_last_updated".to_string(),
            json!(metadata_config.include_last_updated),
        );
        
        // Create dynamic list of enabled custom fields
        let mut enabled_fields = Vec::new();
        if metadata_config.include_prerequisites { enabled_fields.push("prerequisites"); }
        if metadata_config.include_personas { enabled_fields.push("personas"); }
        if metadata_config.include_author { enabled_fields.push("author"); }
        if metadata_config.include_reviewer { enabled_fields.push("reviewer"); }
        if metadata_config.include_business_value { enabled_fields.push("business_value"); }
        if metadata_config.include_complexity { enabled_fields.push("complexity"); }
        if metadata_config.include_epic { enabled_fields.push("epic"); }
        if metadata_config.include_acceptance_criteria { enabled_fields.push("acceptance_criteria"); }
        if metadata_config.include_assumptions { enabled_fields.push("assumptions"); }
        if metadata_config.include_constraints { enabled_fields.push("constraints"); }
        
        data.insert(
            "custom_fields".to_string(),
            json!(enabled_fields),
        );

        // Use methodology-specific rendering
        self.template_engine.render_use_case_with_methodology(&data, methodology)
    }

    /// Helper to generate test files
    fn generate_test_file(&self, use_case: &UseCase) -> Result<()> {
        if use_case.scenarios.is_empty() {
            return Ok(());
        }

        if !self
            .template_engine
            .has_test_template(&self.config.generation.test_language)
        {
            println!(
                "Warning: Test language '{}' not supported, skipping test generation",
                self.config.generation.test_language
            );
            return Ok(());
        }

        let language_registry = LanguageRegistry::new();
        let file_extension = language_registry
            .get(&self.config.generation.test_language)
            .map(|lang| lang.file_extension())
            .unwrap_or("txt");

        let mut data = HashMap::new();
        data.insert("id".to_string(), json!(use_case.id));
        data.insert("title".to_string(), json!(use_case.title));
        data.insert(
            "title_snake_case".to_string(),
            json!(to_snake_case(&use_case.title)),
        );
        data.insert("description".to_string(), json!(use_case.description));
        data.insert("category".to_string(), json!(use_case.category));
        data.insert(
            "category_snake_case".to_string(),
            json!(to_snake_case(&use_case.category)),
        );
        data.insert(
            "test_module_name".to_string(),
            json!(to_snake_case(&use_case.id)),
        );
        data.insert(
            "generated_at".to_string(),
            json!(chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string()),
        );

        // Prepare scenarios data
        let scenarios_data: serde_json::Value = use_case
            .scenarios
            .iter()
            .map(|scenario| {
                json!({
                    "id": scenario.id,
                    "snake_case_id": to_snake_case(&scenario.id),
                    "title": scenario.title,
                    "description": scenario.description,
                    "status": scenario.status.to_string(),
                })
            })
            .collect();

        data.insert("scenarios".to_string(), scenarios_data);

        // Handle existing files
        if self.file_service.test_file_exists(use_case, file_extension)
            && !self.config.generation.overwrite_test_documentation
        {
            println!("⚠️  Test file exists and overwrite_test_documentation=false, skipping");
            return Ok(());
        }

        // Generate new test content
        let test_content = self
            .template_engine
            .render_test(&self.config.generation.test_language, &data)?;

        self.file_service
            .save_test_file(use_case, &test_content, file_extension)?;

        println!(
            "Generated test file: {}/{}",
            to_snake_case(&use_case.category),
            format!("{}.{}", to_snake_case(&use_case.id), file_extension).cyan()
        );

        Ok(())
    }

    /// Helper to generate overview
    fn generate_overview(&self) -> Result<()> {
        // Prepare data for template
        let mut template_data = HashMap::new();

        template_data.insert("project_name".to_string(), json!(self.config.project.name));
        template_data.insert(
            "generated_date".to_string(),
            json!(chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string()),
        );

        let total_scenarios: usize = self.use_cases.iter().map(|uc| uc.scenarios.len()).sum();
        template_data.insert("total_use_cases".to_string(), json!(self.use_cases.len()));
        template_data.insert("total_scenarios".to_string(), json!(total_scenarios));

        // Status distribution
        let mut status_counts = HashMap::new();
        for use_case in &self.use_cases {
            let status_str = use_case.status().to_string();
            *status_counts.entry(status_str).or_insert(0) += 1;
        }
        template_data.insert("status_counts".to_string(), json!(status_counts));

        // Group by category
        let mut categories: HashMap<String, Vec<&UseCase>> = HashMap::new();
        for use_case in &self.use_cases {
            categories
                .entry(use_case.category.clone())
                .or_default()
                .push(use_case);
        }

        let mut category_data = Vec::new();
        for (category, use_cases) in categories {
            let mut use_case_data = Vec::new();
            for use_case in use_cases {
                let mut uc_data = HashMap::new();
                uc_data.insert("id".to_string(), json!(use_case.id));
                uc_data.insert("title".to_string(), json!(use_case.title));
                uc_data.insert("description".to_string(), json!(use_case.description));
                uc_data.insert("priority".to_string(), json!(use_case.priority.to_string()));
                uc_data.insert(
                    "aggregated_status".to_string(),
                    json!(use_case.status().to_string()),
                );
                uc_data.insert(
                    "category_path".to_string(),
                    json!(to_snake_case(&use_case.category)),
                );
                uc_data.insert(
                    "scenario_count".to_string(),
                    json!(use_case.scenarios.len()),
                );

                let scenario_data: Vec<serde_json::Value> = use_case
                    .scenarios
                    .iter()
                    .map(|s| {
                        json!({
                            "id": s.id,
                            "title": s.title,
                            "status": s.status.to_string()
                        })
                    })
                    .collect();

                uc_data.insert("scenarios".to_string(), json!(scenario_data));
                use_case_data.push(json!(uc_data));
            }

            let cat_data = json!({
                "category_name": category,
                "use_cases": use_case_data
            });
            category_data.push(cat_data);
        }

        template_data.insert("categories".to_string(), json!(category_data));

        // Render and save
        let content = self.template_engine.render_overview(&template_data)?;
        self.file_service.save_overview(&content)?;

        Ok(())
    }
}
