// Application service for use case operations
// This orchestrates domain services and infrastructure
use crate::config::Config;
use crate::core::application::creators::{ScenarioCreator, UseCaseCreator};
use crate::core::application::generators::{MarkdownGenerator, OverviewGenerator, TestGenerator};
use crate::core::utils::suggest_alternatives;
use crate::core::{
    domain::{Scenario, ScenarioReference, ScenarioType, UseCaseReference},
    ReferenceType, RepositoryFactory, ScenarioReferenceValidator, TemplateEngine, UseCase,
    UseCaseRepository,
};
use anyhow::Result;

/// Application service that coordinates use case operations
/// This replaces the old UseCaseCoordinator with clean architecture
pub struct UseCaseApplicationService {
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

impl UseCaseApplicationService {
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
        let index = self.find_use_case_index(use_case_id)?;
        let use_case = &self.use_cases[index];

        use_case
            .scenarios
            .iter()
            .find(|s| s.title == scenario_title)
            .map(|s| s.id.clone())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Scenario with title '{}' not found in use case '{}'",
                    scenario_title,
                    use_case_id
                )
            })
    }

    /// Get all use case info that uses a specific persona
    /// Returns a list of tuples (use_case_id, title, scenario_count) where at least one scenario uses the given persona
    pub fn get_use_cases_for_persona(
        &self,
        persona_id: &str,
    ) -> Result<Vec<(String, String, usize)>> {
        let mut matching_use_cases = Vec::new();

        // Scan all loaded use cases for scenarios that use this persona
        for use_case in &self.use_cases {
            let scenario_count = use_case
                .scenarios
                .iter()
                .filter(|scenario| scenario.persona.as_deref() == Some(persona_id))
                .count();

            if scenario_count > 0 {
                matching_use_cases.push((
                    use_case.id.clone(),
                    use_case.title.clone(),
                    scenario_count,
                ));
            }
        }

        Ok(matching_use_cases)
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
        self.save_use_case_with_methodology(&use_case, methodology)?;
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
        self.save_use_case_with_methodology(&use_case, methodology)?;
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
        // Find the use case
        let use_case = match self.use_cases.iter().find(|uc| uc.id == use_case_id) {
            Some(uc) => uc.clone(),
            None => {
                // Get available use case IDs for suggestions
                let available_ids: Vec<String> =
                    self.use_cases.iter().map(|uc| uc.id.clone()).collect();
                let error_msg = suggest_alternatives(use_case_id, &available_ids, "Use case");
                return Err(anyhow::anyhow!("{}", error_msg));
            }
        };

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

        Ok(())
    }

    /// Regenerate markdown for a single use case
    pub fn regenerate_markdown(&self, use_case_id: &str) -> Result<()> {
        // Load use case from TOML (source of truth)
        let use_case = match self.repository.load_by_id(use_case_id)? {
            Some(uc) => uc,
            None => {
                // Get available use case IDs for suggestions
                let available_ids: Vec<String> =
                    self.use_cases.iter().map(|uc| uc.id.clone()).collect();
                let error_msg = suggest_alternatives(use_case_id, &available_ids, "Use case");
                return Err(anyhow::anyhow!("{}", error_msg));
            }
        };

        // Generate markdown from TOML data
        let markdown_content = self.generate_use_case_markdown(&use_case)?;
        self.repository
            .save_markdown(use_case_id, &markdown_content)?;

        Ok(())
    }

    /// Regenerate markdown for all use cases
    pub fn regenerate_all_markdown(&self) -> Result<()> {
        // Load all use cases from TOML (source of truth)
        let use_cases = self.repository.load_all()?;

        for use_case in &use_cases {
            // Generate markdown from TOML data
            let markdown_content = self.generate_use_case_markdown(use_case)?;
            self.repository
                .save_markdown(&use_case.id, &markdown_content)?;
        }

        self.generate_overview()?;
        Ok(())
    }

    // ========== Field Management Methods ==========

    /// Add a precondition to a use case
    pub fn add_precondition(&mut self, use_case_id: &str, precondition: String) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();
        use_case.add_precondition(precondition);
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;
        Ok(())
    }

    /// Get all preconditions for a use case
    pub fn get_preconditions(&self, use_case_id: &str) -> Result<Vec<String>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.preconditions.clone())
    }

    /// Remove a precondition from a use case
    pub fn remove_precondition(&mut self, use_case_id: &str, index: usize) -> Result<()> {
        let index_in_vec = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index_in_vec].clone();

        // Convert 1-based index to 0-based
        let zero_based_index = index.saturating_sub(1);
        if zero_based_index >= use_case.preconditions.len() {
            return Err(anyhow::anyhow!(
                "Precondition index {} is out of bounds",
                index
            ));
        }

        use_case.preconditions.remove(zero_based_index);
        self.repository.save(&use_case)?;
        self.use_cases[index_in_vec] = use_case;
        Ok(())
    }

    /// Add a postcondition to a use case
    pub fn add_postcondition(&mut self, use_case_id: &str, postcondition: String) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();
        use_case.add_postcondition(postcondition);
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;
        Ok(())
    }

    /// Get all postconditions for a use case
    pub fn get_postconditions(&self, use_case_id: &str) -> Result<Vec<String>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.postconditions.clone())
    }

    /// Remove a postcondition from a use case
    pub fn remove_postcondition(&mut self, use_case_id: &str, index: usize) -> Result<()> {
        let index_in_vec = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index_in_vec].clone();

        // Convert 1-based index to 0-based
        let zero_based_index = index.saturating_sub(1);
        if zero_based_index >= use_case.postconditions.len() {
            return Err(anyhow::anyhow!(
                "Postcondition index {} is out of bounds",
                index
            ));
        }

        use_case.postconditions.remove(zero_based_index);
        self.repository.save(&use_case)?;
        self.use_cases[index_in_vec] = use_case;
        Ok(())
    }

    /// Add a reference to a use case
    pub fn add_reference(
        &mut self,
        use_case_id: &str,
        target_id: String,
        relationship: String,
        description: Option<String>,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();
        let reference = UseCaseReference::new(target_id, relationship);
        let reference = if let Some(desc) = description {
            reference.with_description(desc)
        } else {
            reference
        };
        use_case.add_reference(reference);
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;
        Ok(())
    }

    /// Get all references for a use case
    pub fn get_references(&self, use_case_id: &str) -> Result<Vec<UseCaseReference>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.use_case_references.clone())
    }

    /// Remove a reference from a use case
    pub fn remove_reference(&mut self, use_case_id: &str, target_id: &str) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();
        use_case
            .use_case_references
            .retain(|r| r.target_id != target_id);
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;
        Ok(())
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
        let index = self.find_use_case_index(use_case_id)?;
        let use_case = &self.use_cases[index];

        let scenario = self.scenario_creator.create_scenario(
            use_case,
            title,
            scenario_type,
            description,
            preconditions,
            postconditions,
            actors,
        );

        let mut updated_use_case = self.use_cases[index].clone();
        updated_use_case.add_scenario(scenario.clone());
        self.repository.save(&updated_use_case)?;
        self.use_cases[index] = updated_use_case;

        Ok(scenario.id)
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
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        let step =
            self.scenario_creator
                .create_scenario_step(order, actor, action, expected_result);

        use_case.add_step_to_scenario(scenario_id, step)?;
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Update the status of a scenario
    pub fn update_scenario_status(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        new_status: crate::core::Status,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        use_case.update_scenario_status(scenario_id, new_status)?;
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
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
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        use_case.remove_step_from_scenario(scenario_id, step_order)?;
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Add a reference to a scenario
    pub fn add_scenario_reference(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        reference: ScenarioReference,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        // Inline the functionality of deleted add_reference_to_scenario
        let scenario_index = use_case
            .scenarios
            .iter()
            .position(|s| s.id == scenario_id)
            .ok_or_else(|| anyhow::anyhow!("Scenario with ID '{}' not found", scenario_id))?;

        // Validate no circular reference for scenario-to-scenario references
        if matches!(reference.ref_type, ReferenceType::Scenario) {
            ScenarioReferenceValidator::validate_no_circular_reference(
                &use_case,
                scenario_id,
                &reference.target_id,
            )?;
        }

        // Add the reference directly to the scenario
        use_case.scenarios[scenario_index].add_reference(reference);
        use_case.metadata.touch();

        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Remove a reference from a scenario
    pub fn remove_scenario_reference(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        target_id: &str,
        relationship: &str,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        // Inline the functionality of deleted remove_reference_from_scenario
        let scenario_index = use_case
            .scenarios
            .iter()
            .position(|s| s.id == scenario_id)
            .ok_or_else(|| anyhow::anyhow!("Scenario with ID '{}' not found", scenario_id))?;

        use_case.scenarios[scenario_index].remove_reference(target_id, relationship);
        use_case.metadata.touch();
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
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
            methodology,
            &self.use_cases,
            self.repository.as_ref(),
        )?;

        // Generate markdown from TOML data
        let markdown_content = self.generate_use_case_markdown(&use_case)?;
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
            methodology,
            extra_fields,
            &self.use_cases,
            self.repository.as_ref(),
        )?;

        // Generate markdown from TOML data
        let markdown_content = self.generate_use_case_markdown(&use_case)?;
        self.repository
            .save_markdown(&use_case.id, &markdown_content)?;

        Ok(use_case)
    }

    /// Save use case with specific methodology rendering
    fn save_use_case_with_methodology(&self, use_case: &UseCase, methodology: &str) -> Result<()> {
        // Step 1: Save TOML first (source of truth)
        self.repository.save(use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = self
            .repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load use case from TOML"))?;

        // Step 3: Generate methodology-specific markdown from TOML data
        let markdown_content =
            self.generate_use_case_markdown_with_methodology(&use_case_from_toml, methodology)?;
        self.repository
            .save_markdown(&use_case.id, &markdown_content)?;

        // Generate test file if enabled
        if self.config.generation.auto_generate_tests {
            self.generate_test_file(&use_case_from_toml)?;
        }

        Ok(())
    }

    /// Helper to generate use case markdown
    fn generate_use_case_markdown(&self, use_case: &UseCase) -> Result<String> {
        self.markdown_generator.generate(use_case)
    }

    /// Helper to generate use case markdown with specific methodology
    fn generate_use_case_markdown_with_methodology(
        &self,
        use_case: &UseCase,
        methodology: &str,
    ) -> Result<String> {
        self.markdown_generator
            .generate_with_methodology(use_case, methodology)
    }

    /// Generate test file for a use case
    fn generate_test_file(&self, use_case: &UseCase) -> Result<()> {
        self.test_generator.generate(use_case)
    }

    /// Generate overview file
    fn generate_overview(&self) -> Result<()> {
        self.overview_generator.generate(&self.use_cases)
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

        let mut coordinator = UseCaseApplicationService::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let use_case_id = coordinator.create_use_case_with_methodology(
            "Interactive Test".to_string(),
            "testing".to_string(),
            Some("Created via interactive mode".to_string()),
            &default_methodology,
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
        let mut coordinator = UseCaseApplicationService::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let mut categories: Vec<String> = coordinator
            .use_cases
            .iter()
            .map(|uc| uc.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        assert!(categories.is_empty());

        coordinator.create_use_case_with_methodology(
            "Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            &default_methodology,
        )?;

        coordinator.create_use_case_with_methodology(
            "API Use Case".to_string(),
            "api".to_string(),
            None,
            &default_methodology,
        )?;

        coordinator.create_use_case_with_methodology(
            "Another Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            &default_methodology,
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

        let mut coordinator = UseCaseApplicationService::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let _uc1 = coordinator.create_use_case_with_methodology(
            "User Authentication".to_string(),
            "auth".to_string(),
            Some("Handle user login and logout".to_string()),
            &default_methodology,
        )?;

        let _uc2 = coordinator.create_use_case_with_methodology(
            "Data Export".to_string(),
            "api".to_string(),
            Some("Export data in various formats".to_string()),
            &default_methodology,
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

        // Use the existing "feature" methodology which already has custom fields defined
        // (user_story, acceptance_criteria, epic_link, sprint, story_points, etc.)
        let mut coordinator = UseCaseApplicationService::load()?;

        let use_case_id = coordinator.create_use_case_with_methodology(
            "Test Custom Fields".to_string(),
            "testing".to_string(),
            Some("Testing custom fields integration".to_string()),
            "feature",
        )?;

        // Verify the use case was created
        assert_eq!(use_case_id, "UC-TES-001");

        // Load the use case from TOML to verify custom fields were saved
        let loaded_use_case = coordinator
            .repository
            .load_by_id(&use_case_id)?
            .expect("Use case should exist");

        // Verify custom fields from feature methodology are present
        // Check for required fields (from source-templates/methodologies/feature/config.toml)
        assert!(
            loaded_use_case.extra.contains_key("user_story"),
            "user_story should be present (required field). Found keys: {:?}",
            loaded_use_case.extra.keys().collect::<Vec<_>>()
        );
        assert!(
            loaded_use_case.extra.contains_key("acceptance_criteria"),
            "acceptance_criteria should be present (required field)"
        );

        // Note: Optional fields with null/empty values are not saved to TOML
        // This is intentional - TOML doesn't support null values like JSON does
        // Optional fields will only appear in the loaded use case if they have actual values

        // Verify TOML file exists in data directory and contains custom fields
        let data_dir = Path::new(&coordinator.config.directories.data_dir).join("testing");
        let toml_path = data_dir.join("UC-TES-001.toml");
        assert!(
            toml_path.exists(),
            "TOML file should exist at {:?}",
            toml_path
        );

        let toml_content = fs::read_to_string(&toml_path)?;
        assert!(
            toml_content.contains("user_story"),
            "TOML should contain user_story field"
        );
        assert!(
            toml_content.contains("acceptance_criteria"),
            "TOML should contain acceptance_criteria field"
        );

        // Verify markdown was generated
        let md_path = Path::new(&coordinator.config.directories.use_case_dir)
            .join("testing")
            .join("UC-TES-001.md");
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
