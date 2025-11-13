use crate::core::domain::entities::{Scenario, ScenarioStep, ScenarioType, Status, UseCase};
use std::collections::HashSet;

/// Domain service for scenario-related business logic
pub struct ScenarioService;

impl ScenarioService {
    /// Validate that a scenario has all required fields
    pub fn validate_scenario(scenario: &Scenario) -> Result<(), String> {
        if scenario.id.trim().is_empty() {
            return Err("Scenario ID cannot be empty".to_string());
        }

        if scenario.title.trim().is_empty() {
            return Err("Scenario title cannot be empty".to_string());
        }

        if scenario.description.trim().is_empty() {
            return Err("Scenario description cannot be empty".to_string());
        }

        // Validate steps
        for (index, step) in scenario.steps.iter().enumerate() {
            if step.actor.trim().is_empty() {
                return Err(format!("Step {}: actor cannot be empty", index + 1));
            }
            if step.action.trim().is_empty() {
                return Err(format!("Step {}: action cannot be empty", index + 1));
            }
            if step.description.trim().is_empty() {
                return Err(format!("Step {}: description cannot be empty", index + 1));
            }
        }

        // Check for duplicate step orders
        let mut orders = HashSet::new();
        for step in &scenario.steps {
            if !orders.insert(step.order) {
                return Err(format!("Duplicate step order: {}", step.order));
            }
        }

        Ok(())
    }

    /// Validate that a scenario step is valid
    pub fn validate_scenario_step(step: &ScenarioStep) -> Result<(), String> {
        if step.actor.trim().is_empty() {
            return Err("Step actor cannot be empty".to_string());
        }
        if step.action.trim().is_empty() {
            return Err("Step action cannot be empty".to_string());
        }
        if step.description.trim().is_empty() {
            return Err("Step description cannot be empty".to_string());
        }
        Ok(())
    }

    /// Update scenario status with validation
    pub fn update_scenario_status(scenario: &mut Scenario, new_status: Status) -> Result<(), String> {
        // Validate status transition
        if !scenario.status.can_transition_to(&new_status) {
            return Err(format!(
                "Cannot transition from {} to {}",
                scenario.status, new_status
            ));
        }

        scenario.set_status(new_status);
        Ok(())
    }

    /// Add a step to a scenario with validation
    pub fn add_step_to_scenario(scenario: &mut Scenario, step: ScenarioStep) -> Result<(), String> {
        Self::validate_scenario_step(&step)?;

        // Check for duplicate order
        if scenario.steps.iter().any(|s| s.order == step.order) {
            return Err(format!("Step with order {} already exists", step.order));
        }

        scenario.add_step(step);
        Ok(())
    }

    /// Remove a step from a scenario
    pub fn remove_step_from_scenario(scenario: &mut Scenario, step_order: usize) -> Result<(), String> {
        let initial_len = scenario.steps.len();
        scenario.steps.retain(|s| s.order != step_order);

        if scenario.steps.len() == initial_len {
            return Err(format!("Step with order {} not found", step_order));
        }

        scenario.metadata.touch();
        Ok(())
    }

    /// Reorder steps in a scenario (renumber them sequentially)
    pub fn reorder_scenario_steps(scenario: &mut Scenario) {
        for (index, step) in scenario.steps.iter_mut().enumerate() {
            step.order = (index + 1) as usize;
        }
        scenario.metadata.touch();
    }

    /// Get all unique actors in a scenario
    pub fn get_scenario_actors(scenario: &Scenario) -> Vec<String> {
        scenario.actors()
    }

    /// Calculate effective preconditions for a scenario within a use case
    pub fn get_effective_preconditions(scenario: &Scenario, use_case: &UseCase) -> Vec<String> {
        scenario.effective_preconditions(use_case)
    }

    /// Calculate effective postconditions for a scenario within a use case
    pub fn get_effective_postconditions(scenario: &Scenario, use_case: &UseCase) -> Vec<String> {
        scenario.effective_postconditions(use_case)
    }

    /// Check if a scenario can be considered complete
    pub fn is_scenario_complete(scenario: &Scenario) -> bool {
        !scenario.steps.is_empty() && scenario.status.is_complete()
    }

    /// Get scenarios by type from a use case
    pub fn get_scenarios_by_type(use_case: &UseCase, scenario_type: ScenarioType) -> Vec<&Scenario> {
        use_case.scenarios_by_type(scenario_type)
    }

    /// Generate a summary of a scenario
    pub fn get_scenario_summary(scenario: &Scenario) -> String {
        format!(
            "{} ({}): {} steps, status: {}",
            scenario.title,
            scenario.scenario_type,
            scenario.steps.len(),
            scenario.status
        )
    }

    /// Validate that all scenarios in a use case are valid
    pub fn validate_use_case_scenarios(use_case: &UseCase) -> Result<(), String> {
        for scenario in &use_case.scenarios {
            Self::validate_scenario(scenario)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::Metadata;

    fn create_test_scenario() -> Scenario {
        Scenario::new(
            "UC-TEST-001-S01".to_string(),
            "Test Scenario".to_string(),
            "A test scenario".to_string(),
            ScenarioType::HappyPath,
        )
    }

    fn create_test_use_case() -> UseCase {
        UseCase::new(
            "UC-TEST-001".to_string(),
            "Test Use Case".to_string(),
            "Test".to_string(),
            "A test use case".to_string(),
            "medium".to_string(),
        ).unwrap()
    }

    #[test]
    fn test_validate_scenario_valid() {
        let mut scenario = create_test_scenario();
        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));

        assert!(ScenarioService::validate_scenario(&scenario).is_ok());
    }

    #[test]
    fn test_validate_scenario_empty_id() {
        let scenario = Scenario {
            id: "".to_string(),
            ..create_test_scenario()
        };

        assert!(ScenarioService::validate_scenario(&scenario).is_err());
    }

    #[test]
    fn test_validate_scenario_empty_title() {
        let scenario = Scenario {
            title: "".to_string(),
            ..create_test_scenario()
        };

        assert!(ScenarioService::validate_scenario(&scenario).is_err());
    }

    #[test]
    fn test_validate_scenario_duplicate_step_orders() {
        let mut scenario = create_test_scenario();
        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            1, // duplicate order
            "System".to_string(),
            "responds".to_string(),
            "success".to_string(),
        ));

        assert!(ScenarioService::validate_scenario(&scenario).is_err());
    }

    #[test]
    fn test_validate_scenario_step_valid() {
        let step = ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        );

        assert!(ScenarioService::validate_scenario_step(&step).is_ok());
    }

    #[test]
    fn test_validate_scenario_step_empty_actor() {
        let step = ScenarioStep::new(
            1,
            "".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        );

        assert!(ScenarioService::validate_scenario_step(&step).is_err());
    }

    #[test]
    fn test_update_scenario_status_valid() {
        let mut scenario = create_test_scenario();
        assert_eq!(scenario.status, Status::Planned);

        assert!(ScenarioService::update_scenario_status(&mut scenario, Status::InProgress).is_ok());
        assert_eq!(scenario.status, Status::InProgress);
    }

    #[test]
    fn test_update_scenario_status_invalid_transition() {
        let mut scenario = create_test_scenario();
        scenario.set_status(Status::Tested);

        // Cannot go backward in the workflow (Tested -> Implemented is invalid)
        assert!(ScenarioService::update_scenario_status(&mut scenario, Status::Implemented).is_err());
    }

    #[test]
    fn test_add_step_to_scenario() {
        let mut scenario = create_test_scenario();
        let step = ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        );

        assert!(ScenarioService::add_step_to_scenario(&mut scenario, step).is_ok());
        assert_eq!(scenario.steps.len(), 1);
    }

    #[test]
    fn test_add_step_to_scenario_duplicate_order() {
        let mut scenario = create_test_scenario();
        let step1 = ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        );
        let step2 = ScenarioStep::new(
            1, // duplicate
            "System".to_string(),
            "responds".to_string(),
            "success".to_string(),
        );

        assert!(ScenarioService::add_step_to_scenario(&mut scenario, step1).is_ok());
        assert!(ScenarioService::add_step_to_scenario(&mut scenario, step2).is_err());
    }

    #[test]
    fn test_remove_step_from_scenario() {
        let mut scenario = create_test_scenario();
        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));

        assert!(ScenarioService::remove_step_from_scenario(&mut scenario, 1).is_ok());
        assert!(scenario.steps.is_empty());
    }

    #[test]
    fn test_remove_step_from_scenario_not_found() {
        let mut scenario = create_test_scenario();

        assert!(ScenarioService::remove_step_from_scenario(&mut scenario, 1).is_err());
    }

    #[test]
    fn test_reorder_scenario_steps() {
        let mut scenario = create_test_scenario();
        scenario.add_step(ScenarioStep::new(
            3,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "opens".to_string(),
            "page".to_string(),
        ));

        ScenarioService::reorder_scenario_steps(&mut scenario);

        assert_eq!(scenario.steps[0].order, 1);
        assert_eq!(scenario.steps[1].order, 2);
    }

    #[test]
    fn test_get_scenario_actors() {
        let mut scenario = create_test_scenario();
        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            2,
            "System".to_string(),
            "responds".to_string(),
            "success".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            3,
            "User".to_string(),
            "sees".to_string(),
            "result".to_string(),
        ));

        let actors = ScenarioService::get_scenario_actors(&scenario);
        assert_eq!(actors.len(), 2);
        assert!(actors.contains(&"User".to_string()));
        assert!(actors.contains(&"System".to_string()));
    }

    #[test]
    fn test_get_effective_preconditions() {
        let mut use_case = create_test_use_case();
        use_case.add_precondition("User logged in".to_string());

        let mut scenario = create_test_scenario();
        scenario.add_precondition("Valid session".to_string());

        let effective = ScenarioService::get_effective_preconditions(&scenario, &use_case);
        assert_eq!(effective.len(), 2);
        assert!(effective.contains(&"User logged in".to_string()));
        assert!(effective.contains(&"Valid session".to_string()));
    }

    #[test]
    fn test_is_scenario_complete() {
        let mut scenario = create_test_scenario();
        assert!(!ScenarioService::is_scenario_complete(&scenario)); // no steps

        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));
        assert!(!ScenarioService::is_scenario_complete(&scenario)); // not complete status

        scenario.set_status(Status::Deployed);
        assert!(ScenarioService::is_scenario_complete(&scenario)); // complete
    }

    #[test]
    fn test_get_scenarios_by_type() {
        let mut use_case = create_test_use_case();
        use_case.add_scenario(Scenario::new(
            "UC-TEST-001-S01".to_string(),
            "Happy Path".to_string(),
            "Success scenario".to_string(),
            ScenarioType::HappyPath,
        ));
        use_case.add_scenario(Scenario::new(
            "UC-TEST-001-S02".to_string(),
            "Error Case".to_string(),
            "Error scenario".to_string(),
            ScenarioType::ExceptionFlow,
        ));

        let happy_paths = ScenarioService::get_scenarios_by_type(&use_case, ScenarioType::HappyPath);
        assert_eq!(happy_paths.len(), 1);
        assert_eq!(happy_paths[0].title, "Happy Path");
    }

    #[test]
    fn test_get_scenario_summary() {
        let mut scenario = create_test_scenario();
        scenario.add_step(ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "button".to_string(),
        ));

        let summary = ScenarioService::get_scenario_summary(&scenario);
        assert!(summary.contains("Test Scenario"));
        assert!(summary.contains("happy_path"));
        assert!(summary.contains("1 steps"));
        assert!(summary.contains("PLANNED"));
    }

    #[test]
    fn test_validate_use_case_scenarios() {
        let mut use_case = create_test_use_case();
        use_case.add_scenario(create_test_scenario());

        assert!(ScenarioService::validate_use_case_scenarios(&use_case).is_ok());
    }

    #[test]
    fn test_validate_use_case_scenarios_invalid() {
        let mut use_case = create_test_use_case();
        let invalid_scenario = Scenario {
            title: "".to_string(), // invalid
            ..create_test_scenario()
        };
        use_case.add_scenario(invalid_scenario);

        assert!(ScenarioService::validate_use_case_scenarios(&use_case).is_err());
    }
}