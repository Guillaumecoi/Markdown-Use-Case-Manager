use anyhow::Result;

use crate::config::Config;
use crate::core::use_case_coordinator::UseCaseCoordinator;
use super::dto::{DisplayResult, SelectionOptions};

/// Controller for use case operations
pub struct UseCaseController {
    coordinator: UseCaseCoordinator,
}

impl UseCaseController {
    /// Create a new controller
    pub fn new() -> Result<Self> {
        let coordinator = UseCaseCoordinator::load()?;
        Ok(Self { coordinator })
    }

    /// Create a new use case with the default methodology
    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<DisplayResult> {
        let config = Config::load()?;
        let default_methodology = config.templates.default_methodology.clone();
        
        self.create_use_case_with_methodology(title, category, description, default_methodology)
    }

    /// Create a new use case with a specific methodology
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: String,
    ) -> Result<DisplayResult> {
        let use_case_id = self.coordinator.create_use_case_with_methodology(
            title,
            category,
            description,
            &methodology
        )?;
        
        Ok(DisplayResult::success(format!(
            "Created use case: {} with {} methodology",
            use_case_id, methodology
        )))
    }

    /// Add a scenario to a use case
    pub fn add_scenario(
        &mut self,
        use_case_id: String,
        title: String,
        description: Option<String>,
    ) -> Result<DisplayResult> {
        let scenario_id = self.coordinator.add_scenario_to_use_case(
            use_case_id,
            title,
            description
        )?;
        
        Ok(DisplayResult::success(format!("Added scenario: {}", scenario_id)))
    }

    /// Update scenario status
    pub fn update_scenario_status(
        &mut self,
        scenario_id: String,
        status: String,
    ) -> Result<DisplayResult> {
        self.coordinator.update_scenario_status(scenario_id.clone(), status.clone())?;
        
        Ok(DisplayResult::success(format!(
            "Updated scenario {} status to {}",
            scenario_id, status
        )))
    }

    /// List all use cases
    pub fn list_use_cases(&mut self) -> Result<()> {
        self.coordinator.list_use_cases()
    }

    /// Show project status
    pub fn show_status(&mut self) -> Result<()> {
        self.coordinator.show_status()
    }

    /// Get all use case IDs
    pub fn get_use_case_ids(&mut self) -> Result<SelectionOptions> {
        let ids = self.coordinator.get_all_use_case_ids()?;
        Ok(SelectionOptions::new(ids))
    }

    /// Get scenario IDs for a specific use case
    pub fn get_scenario_ids(&mut self, use_case_id: &str) -> Result<SelectionOptions> {
        let ids = self.coordinator.get_scenario_ids_for_use_case(use_case_id)?;
        Ok(SelectionOptions::new(ids))
    }

    /// Get all categories in use
    pub fn get_categories(&mut self) -> Result<SelectionOptions> {
        let categories = self.coordinator.get_all_categories()?;
        Ok(SelectionOptions::new(categories))
    }

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<DisplayResult> {
        self.coordinator.regenerate_use_case_with_methodology(&use_case_id, &methodology)?;
        
        Ok(DisplayResult::success(format!(
            "Regenerated use case {} with {} methodology",
            use_case_id, methodology
        )))
    }

    /// Regenerate markdown for a single use case
    pub fn regenerate_use_case(&mut self, use_case_id: &str) -> Result<()> {
        self.coordinator.regenerate_markdown(use_case_id)
    }

    /// Regenerate markdown for all use cases
    pub fn regenerate_all_use_cases(&mut self) -> Result<()> {
        self.coordinator.regenerate_all_markdown()
    }
}
