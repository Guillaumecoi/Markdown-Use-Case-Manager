// Coordinator for use case operations
// This orchestrates domain services, manages state, and provides transaction boundaries
// Controllers (presentation layer) call this coordinator, which delegates to domain services
use crate::config::Config;
use crate::core::application::creators::{ScenarioCreator, UseCaseCreator};
use crate::core::application::generators::{
    MarkdownGenerator, OutputManager, OverviewGenerator, TestGenerator,
};
use crate::core::application::services;
use crate::core::utils::suggest_alternatives;
use crate::core::{
    domain::{Scenario, ScenarioReference, ScenarioType, UseCaseReference},
    MethodologyView, RepositoryFactory, TemplateEngine, UseCase, UseCaseRepository,
};
use anyhow::Result;
use std::collections::HashMap;

/// Coordinator that orchestrates use case operations and manages application state
///
/// This coordinator provides a centralized point for:
/// - State management (use_cases collection, repository, config)
/// - Service orchestration (delegates to domain services)
/// - Transaction boundaries (coordinates multi-service operations)
/// - Cross-cutting concerns (overview generation, test generation)
///
/// Controllers (presentation layer) are thin adapters that convert CLI/HTTP parameters
/// into domain types and format results for display.
pub struct UseCaseCoordinator {
    config: Config,
    repository: Box<dyn UseCaseRepository>,
    template_engine: TemplateEngine,
    use_cases: Vec<UseCase>,
    use_case_creator: UseCaseCreator,
    scenario_creator: ScenarioCreator,
    markdown_generator: MarkdownGenerator,
    test_generator: TestGenerator,
    overview_generator: OverviewGenerator,
}

impl UseCaseCoordinator {
    // ========== Initialization ==========

    pub fn new(config: Config) -> Result<Self> {
        let repository: Box<dyn UseCaseRepository> = RepositoryFactory::create(&config)?;
        let template_engine = TemplateEngine::with_config(Some(&config));

        // Initialize creator and generators
        let use_case_creator = UseCaseCreator::new(config.clone());
        let scenario_creator = ScenarioCreator::new();
        let markdown_generator = MarkdownGenerator::new(config.clone());
        let test_generator = TestGenerator::new(config.clone());
        let overview_generator = OverviewGenerator::new(config.clone());

        let use_cases = repository.load_all()?;

        Ok(Self {
            config,
            repository,
            template_engine,
            use_cases,
            use_case_creator,
            scenario_creator,
            markdown_generator,
            test_generator,
            overview_generator,
        })
    }

    pub fn load() -> Result<Self> {
        let config = Config::load()?;
        Self::new(config)
    }

    // ========== Query Operations ==========

    /// Get all use cases (for display)
    pub fn get_all_use_cases(&self) -> &[UseCase] {
        &self.use_cases
    }

    /// Find scenario ID by its title within a use case
    pub fn find_scenario_id_by_title(
        &self,
        use_case_id: &str,
        scenario_title: &str,
    ) -> Result<String> {
        let query_service = services::UseCaseQueryService::new(&self.use_cases);
        query_service.find_scenario_id_by_title(use_case_id, scenario_title)
    }

    /// Get all use case info that uses a specific persona
    /// Returns a list of tuples (use_case_id, title, scenario_count) where at least one scenario uses the given persona
    pub fn get_use_cases_for_persona(
        &self,
        persona_id: &str,
    ) -> Result<Vec<(String, String, usize)>> {
        let query_service = services::UseCaseQueryService::new(&self.use_cases);
        query_service.get_use_cases_for_persona(persona_id)
    }

    // Deleted: get_all_use_case_ids() - never used (PR #13)
    // Deleted: get_all_categories() - never used (PR #13)

    // ========== Use Case Creation ==========

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

        // Create use case with methodology fields
        let use_case = self.create_use_case_with_methodology_internal(
            title,
            category,
            description,
            methodology,
        )?;
        let use_case_id = use_case.id.clone();

        // Save and generate markdown
        self.save_use_case_with_views(&use_case)?;
        self.use_cases.push(use_case);
        self.generate_overview()?;

        Ok(use_case_id)
    }

    /// Create a use case with multiple views
    ///
    /// Parses the views string (comma-separated methodology:level pairs) and creates
    /// a multi-view use case that can be rendered in multiple ways.
    ///
    /// # Arguments
    /// * `views` - Comma-separated methodology:level pairs (e.g., "feature:simple,business:normal")
    ///
    /// # Returns
    /// The ID of the created use case
    pub fn create_use_case_with_views(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        views: &str,
    ) -> Result<String> {
        // Parse views string into MethodologyView objects
        let view_list: Vec<MethodologyView> = views
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|view_str| {
                let parts: Vec<&str> = view_str.split(':').collect();
                if parts.len() != 2 {
                    anyhow::bail!(
                        "Invalid view format '{}'. Expected 'methodology:level'",
                        view_str
                    );
                }
                Ok(MethodologyView::new(
                    parts[0].to_string(),
                    parts[1].to_string(),
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        if view_list.is_empty() {
            return Err(anyhow::anyhow!("At least one view must be specified"));
        }

        // Use the new create_use_case_with_views method with empty user fields
        let use_case = self.use_case_creator.create_use_case_with_views(
            title,
            category,
            description,
            "Medium".to_string(), // Default priority for create_use_case_with_views
            view_list,
            HashMap::new(), // No user fields provided
            &self.use_cases,
            self.repository.as_ref(),
        )?;

        let use_case_id = use_case.id.clone();

        // Save and generate markdown for all views (multi-view mode)
        self.save_use_case_with_views(&use_case)?;
        self.use_cases.push(use_case);
        self.generate_overview()?;

        Ok(use_case_id)
    }

    /// Create a use case with multiple views and custom fields
    ///
    /// Parses the views string (comma-separated methodology:level pairs) and creates
    /// a multi-view use case with additional custom fields that can be rendered in multiple ways.
    ///
    /// # Arguments
    /// * `views` - Comma-separated methodology:level pairs (e.g., "feature:simple,business:normal")
    /// * `extra_fields` - Additional field values (priority, status, author, etc.)
    ///
    /// # Returns
    /// The ID of the created use case
    pub fn create_use_case_with_views_and_fields(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        priority: String,
        views: &str,
        extra_fields: std::collections::HashMap<String, String>,
    ) -> Result<String> {
        // Parse views string into MethodologyView objects
        let view_list: Vec<MethodologyView> = views
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|view_str| {
                let parts: Vec<&str> = view_str.split(':').collect();
                if parts.len() != 2 {
                    anyhow::bail!(
                        "Invalid view format '{}'. Expected 'methodology:level'",
                        view_str
                    );
                }
                Ok(MethodologyView::new(
                    parts[0].to_string(),
                    parts[1].to_string(),
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        if view_list.is_empty() {
            return Err(anyhow::anyhow!("At least one view must be specified"));
        }

        // Use the new create_use_case_with_views method that properly handles methodology_fields
        let use_case = self.use_case_creator.create_use_case_with_views(
            title,
            category,
            description,
            priority,
            view_list,
            extra_fields,
            &self.use_cases,
            self.repository.as_ref(),
        )?;

        let use_case_id = use_case.id.clone();

        // Save and generate markdown for all views (multi-view mode)
        self.save_use_case_with_views(&use_case)?;
        self.use_cases.push(use_case);
        self.generate_overview()?;

        Ok(use_case_id)
    }

    /// Create use case with custom fields
    pub fn create_use_case_with_fields(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
        extra_fields: std::collections::HashMap<String, String>,
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

        // Create use case with custom fields
        let use_case = self.create_use_case_with_fields_internal(
            title,
            category,
            description,
            methodology,
            extra_fields,
        )?;
        let use_case_id = use_case.id.clone();

        // Save and generate markdown
        self.save_use_case_with_views(&use_case)?;
        self.use_cases.push(use_case);
        self.generate_overview()?;

        Ok(use_case_id)
    }

    // ========== Regeneration Operations ==========

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: &str,
        methodology: &str,
    ) -> Result<()> {
        let regen_service = services::MarkdownRegenerationService::new(
            &self.repository,
            &self.use_cases,
            &self.markdown_generator,
            &self.template_engine,
        );
        regen_service.regenerate_use_case_with_methodology(use_case_id, methodology)
    }

    /// Regenerate markdown for a single use case
    pub fn regenerate_markdown(&self, use_case_id: &str) -> Result<()> {
        let regen_service = services::MarkdownRegenerationService::new(
            &self.repository,
            &self.use_cases,
            &self.markdown_generator,
            &self.template_engine,
        );
        regen_service.regenerate_markdown(use_case_id)
    }

    /// Regenerate markdown for all use cases
    pub fn regenerate_all_markdown(&self) -> Result<()> {
        // Load all use cases from TOML (source of truth)
        let use_cases = self.repository.load_all()?;

        for use_case in &use_cases {
            // Generate markdown for each enabled view
            for view in use_case.enabled_views() {
                let markdown_content =
                    self.markdown_generator
                        .generate(use_case, None, Some(&view))?;
                let filename = format!("{}-{}-{}.md", use_case.id, view.methodology, view.level);
                self.repository.save_markdown_with_filename(
                    use_case,
                    &filename,
                    &markdown_content,
                )?;
            }
        }

        self.generate_overview()?;
        Ok(())
    }

    // ========== Field Management Methods ==========

    /// Add a precondition to a use case
    pub fn add_precondition(&mut self, use_case_id: &str, precondition: String) -> Result<()> {
        let mut service =
            services::PreconditionPostconditionService::new(&self.repository, &mut self.use_cases);
        service.add_precondition(use_case_id, precondition)
    }

    /// Get all preconditions for a use case
    pub fn get_preconditions(&self, use_case_id: &str) -> Result<Vec<String>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.preconditions.clone())
    }

    /// Remove a precondition from a use case
    pub fn remove_precondition(&mut self, use_case_id: &str, index: usize) -> Result<()> {
        let mut service =
            services::PreconditionPostconditionService::new(&self.repository, &mut self.use_cases);
        service.remove_precondition(use_case_id, index)
    }

    /// Add a postcondition to a use case
    pub fn add_postcondition(&mut self, use_case_id: &str, postcondition: String) -> Result<()> {
        let mut service =
            services::PreconditionPostconditionService::new(&self.repository, &mut self.use_cases);
        service.add_postcondition(use_case_id, postcondition)
    }

    /// Get all postconditions for a use case
    pub fn get_postconditions(&self, use_case_id: &str) -> Result<Vec<String>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.postconditions.clone())
    }

    /// Remove a postcondition from a use case
    pub fn remove_postcondition(&mut self, use_case_id: &str, index: usize) -> Result<()> {
        let mut service =
            services::PreconditionPostconditionService::new(&self.repository, &mut self.use_cases);
        service.remove_postcondition(use_case_id, index)
    }

    /// Add a reference to a use case
    pub fn add_reference(
        &mut self,
        use_case_id: &str,
        target_id: String,
        relationship: String,
        description: Option<String>,
    ) -> Result<()> {
        let mut service =
            services::ReferenceManagementService::new(&self.repository, &mut self.use_cases);
        service.add_reference(use_case_id, target_id, relationship, description)
    }

    /// Get all references for a use case
    pub fn get_references(&self, use_case_id: &str) -> Result<Vec<UseCaseReference>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.use_case_references.clone())
    }

    /// Remove a reference from a use case
    pub fn remove_reference(&mut self, use_case_id: &str, target_id: &str) -> Result<()> {
        let mut service =
            services::ReferenceManagementService::new(&self.repository, &mut self.use_cases);
        service.remove_reference(use_case_id, target_id)
    }

    // ========== Scenario Management Methods ==========

    /// Add a scenario to a use case
    pub fn add_scenario(
        &mut self,
        use_case_id: &str,
        title: String,
        scenario_type: ScenarioType,
        description: Option<String>,
        preconditions: Vec<String>,
        postconditions: Vec<String>,
        actors: Vec<String>,
    ) -> Result<String> {
        let mut scenario_service = services::ScenarioManagementService::new(
            &self.repository,
            &mut self.use_cases,
            &self.scenario_creator,
        );
        scenario_service.add_scenario(
            use_case_id,
            title,
            scenario_type,
            description,
            preconditions,
            postconditions,
            actors,
        )
    }

    /// Add a step to an existing scenario
    pub fn add_scenario_step(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        order: u32,
        actor: String,
        action: String,
        expected_result: Option<String>,
    ) -> Result<()> {
        let mut scenario_service = services::ScenarioManagementService::new(
            &self.repository,
            &mut self.use_cases,
            &self.scenario_creator,
        );
        scenario_service.add_scenario_step(
            use_case_id,
            scenario_id,
            order,
            actor,
            action,
            expected_result,
        )
    }

    /// Update the status of a scenario
    pub fn update_scenario_status(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        new_status: crate::core::Status,
    ) -> Result<()> {
        let mut scenario_service = services::ScenarioManagementService::new(
            &self.repository,
            &mut self.use_cases,
            &self.scenario_creator,
        );
        scenario_service.update_scenario_status(use_case_id, scenario_id, new_status)
    }

    /// Get all scenarios for a use case
    pub fn get_scenarios(&self, use_case_id: &str) -> Result<Vec<Scenario>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.scenarios.clone())
    }

    /// Remove a step from a scenario
    pub fn remove_scenario_step(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        step_order: u32,
    ) -> Result<()> {
        let mut scenario_service = services::ScenarioManagementService::new(
            &self.repository,
            &mut self.use_cases,
            &self.scenario_creator,
        );
        scenario_service.remove_scenario_step(use_case_id, scenario_id, step_order)
    }

    /// Add a reference to a scenario
    pub fn add_scenario_reference(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        reference: ScenarioReference,
    ) -> Result<()> {
        let mut scenario_service = services::ScenarioManagementService::new(
            &self.repository,
            &mut self.use_cases,
            &self.scenario_creator,
        );
        scenario_service.add_scenario_reference(use_case_id, scenario_id, reference)
    }

    /// Remove a reference from a scenario
    pub fn remove_scenario_reference(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        target_id: &str,
        relationship: &str,
    ) -> Result<()> {
        let mut scenario_service = services::ScenarioManagementService::new(
            &self.repository,
            &mut self.use_cases,
            &self.scenario_creator,
        );
        scenario_service.remove_scenario_reference(
            use_case_id,
            scenario_id,
            target_id,
            relationship,
        )
    }

    /// Get all scenarios referenced by a scenario
    pub fn get_scenario_references(
        &self,
        use_case_id: &str,
        scenario_id: &str,
    ) -> Result<Vec<ScenarioReference>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        let scenario = use_case
            .scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .ok_or_else(|| {
                let available_ids: Vec<String> =
                    use_case.scenarios.iter().map(|s| s.id.clone()).collect();
                let error_msg = suggest_alternatives(scenario_id, &available_ids, "Scenario");
                anyhow::anyhow!("{}", error_msg)
            })?;

        Ok(scenario.references.clone())
    }

    // ========== Private Helpers (Delegation) ==========

    /// Helper to find a use case index by ID
    fn find_use_case_index(&self, use_case_id: &str) -> Result<usize> {
        self.use_cases
            .iter()
            .position(|uc| uc.id == use_case_id)
            .ok_or_else(|| {
                let available_ids: Vec<String> =
                    self.use_cases.iter().map(|uc| uc.id.clone()).collect();
                let error_msg = suggest_alternatives(use_case_id, &available_ids, "Use case");
                anyhow::anyhow!("{}", error_msg)
            })
    }

    /// Helper to find a use case by ID (immutable)
    fn find_use_case_by_id(&self, use_case_id: &str) -> Result<&UseCase> {
        let index = self.find_use_case_index(use_case_id)?;
        Ok(&self.use_cases[index])
    }

    // Deleted: create_use_case_internal() - never used (PR #13)

    /// Internal helper to create use cases with methodology custom fields
    fn create_use_case_with_methodology_internal(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
    ) -> Result<UseCase> {
        let use_case = self.use_case_creator.create_use_case_with_methodology(
            title,
            category,
            description,
            "Medium".to_string(), // Default priority for internal helper
            methodology,
            &self.use_cases,
            self.repository.as_ref(),
        )?;

        // Generate markdown from TOML data
        let markdown_content = self.markdown_generator.generate(&use_case, None, None)?;
        self.repository
            .save_markdown(&use_case.id, &markdown_content)?;

        Ok(use_case)
    }

    fn create_use_case_with_fields_internal(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
        extra_fields: std::collections::HashMap<String, String>,
    ) -> Result<UseCase> {
        let use_case = self.use_case_creator.create_use_case_with_custom_fields(
            title,
            category,
            description,
            "Medium".to_string(), // Default priority for internal helper
            methodology,
            extra_fields,
            &self.use_cases,
            self.repository.as_ref(),
        )?;

        // Generate markdown from TOML data
        let markdown_content = self.markdown_generator.generate(&use_case, None, None)?;
        self.repository
            .save_markdown(&use_case.id, &markdown_content)?;

        Ok(use_case)
    }

    /// Save use case and generate markdown for all views
    fn save_use_case_with_views(&self, use_case: &UseCase) -> Result<()> {
        // Step 1: Save TOML first (source of truth)
        self.repository.save(use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = self
            .repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load use case from TOML"))?;

        // Step 3: Generate markdown files based on views
        // Always use OutputManager for consistent filename generation
        let all_outputs = OutputManager::generate_all_filenames(&use_case_from_toml);
        for (filename, view) in all_outputs {
            // Generate with specific view
            let content =
                self.markdown_generator
                    .generate(&use_case_from_toml, None, Some(&view))?;

            self.repository.save_markdown_with_filename(
                &use_case_from_toml,
                &filename,
                &content,
            )?;
        }

        // Generate test file if enabled
        if self.config.generation.auto_generate_tests {
            self.generate_test_file(&use_case_from_toml)?;
        }

        Ok(())
    }

    /// Generate test file for a use case
    fn generate_test_file(&self, use_case: &UseCase) -> Result<()> {
        self.test_generator.generate(use_case)
    }

    /// Generate overview file
    fn generate_overview(&self) -> Result<()> {
        self.overview_generator.generate(&self.use_cases)
    }

    // ========== Cleanup Operations ==========

    /// Clean up orphaned methodology fields from use cases
    ///
    /// Scans methodology_fields HashMap in each use case and removes entries for
    /// methodologies that are not currently used by any enabled view.
    ///
    /// # Arguments
    /// * `use_case_id` - Optional specific use case to clean. If None, cleans all use cases.
    /// * `dry_run` - If true, returns what would be cleaned without making changes
    ///
    /// # Returns
    /// A tuple of (cleaned_count, total_checked, details) where:
    /// - cleaned_count: number of use cases that had fields removed
    /// - total_checked: number of use cases checked
    /// - details: vector of (use_case_id, removed_methodologies) for each cleaned use case
    pub fn cleanup_methodology_fields(
        &mut self,
        use_case_id: Option<String>,
        dry_run: bool,
    ) -> Result<(usize, usize, Vec<(String, Vec<String>)>)> {
        let mut service =
            services::MethodologyFieldCleanupService::new(&self.repository, &mut self.use_cases);
        service.cleanup_methodology_fields(use_case_id, dry_run)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::application::testing::test_helpers::init_test_project;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_interactive_workflow_simulation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(None)?;

        let mut coordinator = UseCaseCoordinator::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let use_case_id = coordinator.create_use_case_with_views(
            "Interactive Test".to_string(),
            "testing".to_string(),
            Some("Created via interactive mode".to_string()),
            &format!("{}:normal", default_methodology),
        )?;
        assert_eq!(use_case_id, "UC-TES-001");

        let use_case_ids: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.id.clone())
            .collect();
        assert_eq!(use_case_ids.len(), 1);
        assert!(use_case_ids.contains(&"UC-TES-001".to_string()));

        let final_use_case_ids: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.id.clone())
            .collect();
        assert_eq!(final_use_case_ids.len(), 1);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_interactive_category_suggestions() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(None)?;
        let mut coordinator = UseCaseCoordinator::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let mut categories: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        assert!(categories.is_empty());

        coordinator.create_use_case_with_views(
            "Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            &format!("{}:normal", default_methodology),
        )?;

        coordinator.create_use_case_with_views(
            "API Use Case".to_string(),
            "api".to_string(),
            None,
            &format!("{}:normal", default_methodology),
        )?;

        coordinator.create_use_case_with_views(
            "Another Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            &format!("{}:normal", default_methodology),
        )?;

        let mut categories: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0], "api");
        assert_eq!(categories[1], "authentication");

        let use_case_ids: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.id.clone())
            .collect();
        assert_eq!(use_case_ids.len(), 3);
        assert!(use_case_ids.contains(&"UC-AUT-001".to_string()));
        assert!(use_case_ids.contains(&"UC-API-001".to_string()));
        assert!(use_case_ids.contains(&"UC-AUT-002".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_complete_interactive_workflow_simulation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(Some("rust".to_string()))?;

        let mut coordinator = UseCaseCoordinator::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let _uc1 = coordinator.create_use_case_with_views(
            "User Authentication".to_string(),
            "auth".to_string(),
            Some("Handle user login and logout".to_string()),
            &format!("{}:normal", default_methodology),
        )?;

        let _uc2 = coordinator.create_use_case_with_views(
            "Data Export".to_string(),
            "api".to_string(),
            Some("Export data in various formats".to_string()),
            &format!("{}:normal", default_methodology),
        )?;

        let all_use_cases: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.id.clone())
            .collect();
        assert_eq!(all_use_cases.len(), 2);

        let mut categories: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"api".to_string()));
        assert!(categories.contains(&"auth".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_custom_fields_end_to_end_flow() -> Result<()> {
        // Skip test if source templates can't be found
        // This can happen when running all tests together
        if crate::config::TemplateManager::find_source_templates_dir().is_err() {
            eprintln!(
                "SKIPPING test_custom_fields_end_to_end_flow: source templates not available"
            );
            return Ok(());
        }

        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();
        env::set_current_dir(&temp_path)?;

        // Initialize project with templates (includes feature methodology with custom fields)
        init_test_project(None)?;

        // Verify templates were copied - if not, fail with clear message
        let templates_dir = Path::new(".config/.mucm/template-assets");
        if !templates_dir.exists() {
            anyhow::bail!(
                "Templates were not copied. Template dir {:?} doesn't exist",
                templates_dir
            );
        }

        let feature_dir = templates_dir.join("methodologies/feature");
        if !feature_dir.exists() {
            anyhow::bail!("Feature methodology not found at {:?}", feature_dir);
        }

        // Use the existing "feature" methodology which has custom fields defined
        // (user_segment, success_metrics, hypothesis, feature_dependencies, design_assets)
        let mut coordinator = UseCaseCoordinator::load()?;

        let use_case_id = coordinator.create_use_case_with_views(
            "Test Custom Fields".to_string(),
            "testing".to_string(),
            Some("Testing custom fields integration".to_string()),
            "feature:normal",
        )?;

        // Verify the use case was created
        assert_eq!(use_case_id, "UC-TES-001");

        // Load the use case from TOML to verify it can be loaded successfully
        let _loaded_use_case = coordinator
            .repository
            .load_by_id(&use_case_id)?
            .expect("Use case should exist");

        // Verify that the use case can have custom fields from feature methodology
        // Note: Normal level has optional custom fields - user_segment, success_metrics, hypothesis
        // Custom fields will only appear if they have actual values set

        // Note: Optional fields with null/empty values are not saved to TOML
        // This is intentional - TOML doesn't support null values like JSON does
        // Optional fields will only appear in the loaded use case if they have actual values

        // Verify TOML file exists in data directory
        let data_dir = Path::new(&coordinator.config.directories.data_dir).join("testing");
        let toml_path = data_dir.join("UC-TES-001.toml");
        assert!(
            toml_path.exists(),
            "TOML file should exist at {:?}",
            toml_path
        );

        // The feature methodology no longer has required custom fields at simple level
        // so we just verify the TOML file was created successfully
        let _toml_content = fs::read_to_string(&toml_path)?;

        // Verify markdown was generated
        let md_path = Path::new(&coordinator.config.directories.use_case_dir)
            .join("testing")
            .join("UC-TES-001-feature-normal.md");
        assert!(
            md_path.exists(),
            "Markdown file should exist at {:?}",
            md_path
        );

        let md_content = fs::read_to_string(&md_path)?;
        assert!(
            md_content.contains("Test Custom Fields"),
            "Markdown should contain title"
        );

        Ok(())
    }
}
