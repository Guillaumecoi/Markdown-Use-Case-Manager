use crate::core::domain::entities::{ReferenceType, ScenarioReference, UseCase};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Validates scenario references and prevents circular dependencies
pub struct ScenarioReferenceValidator;

impl ScenarioReferenceValidator {
    /// Validate that adding a reference won't create a circular dependency
    pub fn validate_no_circular_reference(
        use_case: &UseCase,
        from_scenario_id: &str,
        to_scenario_id: &str,
    ) -> Result<()> {
        // Build dependency graph
        let graph = Self::build_dependency_graph(use_case);

        // Check if adding this edge would create a cycle
        if Self::would_create_cycle(&graph, from_scenario_id, to_scenario_id) {
            anyhow::bail!(
                "Adding reference from {} to {} would create a circular dependency",
                from_scenario_id,
                to_scenario_id
            );
        }

        Ok(())
    }

    /// Build a dependency graph from scenario references
    fn build_dependency_graph(use_case: &UseCase) -> HashMap<String, Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for scenario in &use_case.scenarios {
            let dependencies: Vec<String> = scenario
                .references
                .iter()
                .filter(|r| matches!(r.ref_type, ReferenceType::Scenario))
                .map(|r| r.target_id.clone())
                .collect();

            graph.insert(scenario.id.clone(), dependencies);
        }

        graph
    }

    /// Check if adding an edge would create a cycle using DFS
    fn would_create_cycle(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool {
        // If we can reach 'from' starting from 'to', adding from->to creates a cycle
        Self::can_reach(graph, to, from)
    }

    /// Check if we can reach 'target' starting from 'start' using DFS
    fn can_reach(graph: &HashMap<String, Vec<String>>, start: &str, target: &str) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![start.to_string()];

        while let Some(current) = stack.pop() {
            if current == target {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        false
    }

    /// Get all scenarios that would be affected by a change to a scenario
    pub fn find_affected_scenarios(use_case: &UseCase, scenario_id: &str) -> Vec<String> {
        use_case
            .scenarios
            .iter()
            .filter(|s| s.references_scenario(scenario_id))
            .map(|s| s.id.clone())
            .collect()
    }

    /// Validate that a reference is valid (target exists, no self-reference, etc.)
    pub fn validate_reference(
        use_case: &UseCase,
        from_scenario_id: &str,
        reference: &ScenarioReference,
    ) -> Result<()> {
        // Check for self-reference
        if matches!(reference.ref_type, ReferenceType::Scenario)
            && reference.target_id == from_scenario_id
        {
            anyhow::bail!("Scenario cannot reference itself");
        }

        // Check that target exists
        match reference.ref_type {
            ReferenceType::Scenario => {
                if !use_case
                    .scenarios
                    .iter()
                    .any(|s| s.id == reference.target_id)
                {
                    anyhow::bail!(
                        "Referenced scenario '{}' does not exist",
                        reference.target_id
                    );
                }
            }
            ReferenceType::UseCase => {
                // For now, we don't validate external use case references
                // This could be enhanced to check against a use case registry
            }
        }

        // Check for circular reference if it's a scenario reference
        if matches!(reference.ref_type, ReferenceType::Scenario) {
            Self::validate_no_circular_reference(use_case, from_scenario_id, &reference.target_id)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::{Scenario, ScenarioType};

    fn create_test_use_case_with_scenarios() -> UseCase {
        let mut uc = UseCase::new(
            "UC-TEST-001".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        // Add three scenarios
        uc.add_scenario(Scenario::new(
            "UC-TEST-001-S01".to_string(),
            "Scenario 1".to_string(),
            "First scenario".to_string(),
            ScenarioType::HappyPath,
        ));

        uc.add_scenario(Scenario::new(
            "UC-TEST-001-S02".to_string(),
            "Scenario 2".to_string(),
            "Second scenario".to_string(),
            ScenarioType::AlternativeFlow,
        ));

        uc.add_scenario(Scenario::new(
            "UC-TEST-001-S03".to_string(),
            "Scenario 3".to_string(),
            "Third scenario".to_string(),
            ScenarioType::ExceptionFlow,
        ));

        uc
    }

    #[test]
    fn test_no_cycle_with_linear_dependencies() {
        let mut uc = create_test_use_case_with_scenarios();

        // S01 -> S02 is fine
        uc.scenarios[0].add_reference(ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S02".to_string(),
            "extends".to_string(),
        ));

        // S02 -> S03 is fine
        let result = ScenarioReferenceValidator::validate_no_circular_reference(
            &uc,
            "UC-TEST-001-S02",
            "UC-TEST-001-S03",
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_direct_cycle_detected() {
        let mut uc = create_test_use_case_with_scenarios();

        // S01 -> S02
        uc.scenarios[0].add_reference(ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S02".to_string(),
            "extends".to_string(),
        ));

        // S02 -> S01 would create a cycle
        let result = ScenarioReferenceValidator::validate_no_circular_reference(
            &uc,
            "UC-TEST-001-S02",
            "UC-TEST-001-S01",
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_indirect_cycle_detected() {
        let mut uc = create_test_use_case_with_scenarios();

        // S01 -> S02
        uc.scenarios[0].add_reference(ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S02".to_string(),
            "extends".to_string(),
        ));

        // S02 -> S03
        uc.scenarios[1].add_reference(ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S03".to_string(),
            "extends".to_string(),
        ));

        // S03 -> S01 would create an indirect cycle
        let result = ScenarioReferenceValidator::validate_no_circular_reference(
            &uc,
            "UC-TEST-001-S03",
            "UC-TEST-001-S01",
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_find_affected_scenarios() {
        let mut uc = create_test_use_case_with_scenarios();

        // S02 references S01
        uc.scenarios[1].add_reference(ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S01".to_string(),
            "extends".to_string(),
        ));

        // S03 also references S01
        uc.scenarios[2].add_reference(ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S01".to_string(),
            "depends_on".to_string(),
        ));

        let affected = ScenarioReferenceValidator::find_affected_scenarios(&uc, "UC-TEST-001-S01");

        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&"UC-TEST-001-S02".to_string()));
        assert!(affected.contains(&"UC-TEST-001-S03".to_string()));
    }

    #[test]
    fn test_validate_reference_self_reference() {
        let uc = create_test_use_case_with_scenarios();

        let reference = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S01".to_string(),
            "extends".to_string(),
        );

        let result =
            ScenarioReferenceValidator::validate_reference(&uc, "UC-TEST-001-S01", &reference);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot reference itself"));
    }

    #[test]
    fn test_validate_reference_nonexistent_scenario() {
        let uc = create_test_use_case_with_scenarios();

        let reference = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-TEST-001-S99".to_string(),
            "extends".to_string(),
        );

        let result =
            ScenarioReferenceValidator::validate_reference(&uc, "UC-TEST-001-S01", &reference);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validate_reference_use_case_allowed() {
        let uc = create_test_use_case_with_scenarios();

        let reference = ScenarioReference::new(
            ReferenceType::UseCase,
            "UC-AUTH-001".to_string(),
            "depends_on".to_string(),
        );

        let result =
            ScenarioReferenceValidator::validate_reference(&uc, "UC-TEST-001-S01", &reference);

        assert!(result.is_ok());
    }
}
