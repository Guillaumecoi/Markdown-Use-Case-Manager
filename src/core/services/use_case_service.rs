// src/core/services/use_case_service.rs
use crate::core::models::{Priority, Scenario, Status, UseCase};
use crate::core::template_engine::to_snake_case;
use std::path::Path;
use anyhow::Result;

/// Core business logic for use case management
/// This service focuses purely on domain operations without I/O concerns
#[derive(Clone)]
pub struct UseCaseService;

impl UseCaseService {
    pub fn new() -> Self {
        Self
    }

    /// Generate a use case ID based on category and existing use cases
    #[allow(clippy::unused_self)]
    /// Generate a unique use case ID that checks both in-memory use cases and filesystem
    pub fn generate_unique_use_case_id(&self, category: &str, use_cases: &[UseCase], use_case_dir: &str) -> String {
        let category_prefix = category.to_uppercase().chars().take(3).collect::<String>();
        let category_dir = Path::new(use_case_dir).join(to_snake_case(category));
        
        // Find the highest existing number by checking both in-memory and filesystem
        let mut max_number = 0;
        
        // Check in-memory use cases
        for uc in use_cases.iter() {
            if uc.category.to_lowercase() == category.to_lowercase() {
                if let Some(num) = self.extract_number_from_id(&uc.id, &category_prefix) {
                    max_number = max_number.max(num);
                }
            }
        }
        
        // Check filesystem for existing files
        if category_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&category_dir) {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".md") && file_name.starts_with(&format!("UC-{}-", category_prefix)) {
                            let id_part = file_name.trim_end_matches(".md");
                            if let Some(num) = self.extract_number_from_id(id_part, &category_prefix) {
                                max_number = max_number.max(num);
                            }
                        }
                    }
                }
            }
        }
        
        format!("UC-{}-{:03}", category_prefix, max_number + 1)
    }
    
    /// Extract number from ID like "UC-CON-001" -> Some(1)
    fn extract_number_from_id(&self, id: &str, category_prefix: &str) -> Option<usize> {
        let expected_prefix = format!("UC-{}-", category_prefix);
        if id.starts_with(&expected_prefix) {
            let number_part = &id[expected_prefix.len()..];
            number_part.parse::<usize>().ok()
        } else {
            None
        }
    }

    /// Create a new use case with the given parameters
    #[allow(clippy::unused_self)]
    pub fn create_use_case(
        &self,
        id: String,
        title: String,
        category: String,
        description: String,
    ) -> UseCase {
        UseCase::new(id, title, category, description, Priority::Medium)
    }

    /// Generate a scenario ID for a use case
    #[allow(clippy::unused_self)]
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
    #[allow(clippy::unused_self)]
    pub fn update_scenario_status(
        &self,
        use_case: &mut UseCase,
        scenario_id: &str,
        status: Status,
    ) -> Result<()> {
        if use_case.update_scenario_status(scenario_id, status) {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Scenario {} not found in use case {}",
                scenario_id,
                use_case.id
            ))
        }
    }

    /// Parse status string to Status enum
    #[allow(clippy::unused_self)]
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
}

impl Default for UseCaseService {
    fn default() -> Self {
        Self::new()
    }
}
