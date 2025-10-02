// src/core/services/use_case_service.rs
use crate::core::models::{Scenario, Status, UseCase};
use anyhow::Result;

/// Core business logic for use case management
/// This service focuses purely on domain operations without I/O concerns
pub struct UseCaseService;

impl UseCaseService {
    pub fn new() -> Self {
        Self
    }

    /// Generate a use case ID based on category and existing use cases
    pub fn generate_use_case_id(&self, category: &str, existing_use_cases: &[UseCase]) -> String {
        let category_prefix = category.to_uppercase().chars().take(3).collect::<String>();
        let existing_count = existing_use_cases
            .iter()
            .filter(|uc| uc.category.to_uppercase() == category.to_uppercase())
            .count();

        format!("UC-{}-{:03}", category_prefix, existing_count + 1)
    }

    /// Create a new use case with the given parameters
    pub fn create_use_case(
        &self,
        id: String,
        title: String,
        category: String,
        description: String,
    ) -> UseCase {
        UseCase::new(id, title, category, description)
    }

    /// Generate a scenario ID for a use case
    pub fn generate_scenario_id(&self, use_case: &UseCase) -> String {
        let scenario_count = use_case.scenarios.len();
        format!("{}-S{:02}", use_case.id, scenario_count + 1)
    }

    /// Add a scenario to a use case
    pub fn add_scenario_to_use_case(
        &self,
        use_case: &mut UseCase,
        title: String,
        description: Option<String>,
    ) -> String {
        let scenario_id = self.generate_scenario_id(use_case);
        let description = description.unwrap_or_default();
        
        let scenario = Scenario::new(scenario_id.clone(), title, description);
        use_case.add_scenario(scenario);
        
        scenario_id
    }

    /// Update scenario status in a use case
    pub fn update_scenario_status(
        &self,
        use_case: &mut UseCase,
        scenario_id: &str,
        status: Status,
    ) -> Result<()> {
        if use_case.update_scenario_status(scenario_id, status) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Scenario {} not found in use case {}", scenario_id, use_case.id))
        }
    }

    /// Parse status string to Status enum
    pub fn parse_status(&self, status_str: &str) -> Result<Status> {
        match status_str.to_lowercase().as_str() {
            "planned" => Ok(Status::Planned),
            "in_progress" => Ok(Status::InProgress),
            "implemented" => Ok(Status::Implemented),
            "tested" => Ok(Status::Tested),
            "deployed" => Ok(Status::Deployed),
            "deprecated" => Ok(Status::Deprecated),
            _ => Err(anyhow::anyhow!(
                "Invalid status: {}. Valid options: planned, in_progress, implemented, tested, deployed, deprecated", 
                status_str
            )),
        }
    }

    /// Find a use case by ID
    #[allow(dead_code)]
    pub fn find_use_case_by_id<'a>(
        &self,
        use_cases: &'a [UseCase],
        use_case_id: &str,
    ) -> Option<&'a UseCase> {
        use_cases.iter().find(|uc| uc.id == use_case_id)
    }

    /// Find a mutable use case by ID
    #[allow(dead_code)]
    pub fn find_use_case_by_id_mut<'a>(
        &self,
        use_cases: &'a mut [UseCase],
        use_case_id: &str,
    ) -> Option<&'a mut UseCase> {
        use_cases.iter_mut().find(|uc| uc.id == use_case_id)
    }

    /// Find a use case that contains a specific scenario
    #[allow(dead_code)]
    pub fn find_use_case_by_scenario_id<'a>(
        &self,
        use_cases: &'a mut [UseCase],
        scenario_id: &str,
    ) -> Option<&'a mut UseCase> {
        use_cases.iter_mut().find(|uc| {
            uc.scenarios.iter().any(|s| s.id == scenario_id)
        })
    }
}

impl Default for UseCaseService {
    fn default() -> Self {
        Self::new()
    }
}