// Unit tests for UseCase struct and related functionality
// Unit tests for UseCase struct and related functionality
use crate::test_utils::{create_test_use_case, set_scenario_status};
use markdown_use_case_manager::core::models::{Priority, Scenario, Status, UseCase};

/// Test create_test_use_case() creates use case with correct initial values
#[test]
fn test_use_case_new() {
    let use_case = create_test_use_case(
        "UC-001".to_string(),
        "Test Use Case".to_string(),
        "authentication".to_string(),
        "Test description for use case".to_string(),
    );

    assert_eq!(use_case.id, "UC-001");
    assert_eq!(use_case.title, "Test Use Case");
    assert_eq!(use_case.category, "authentication");
    assert_eq!(use_case.description, "Test description for use case");
    assert_eq!(use_case.priority, Priority::Medium);
    assert!(use_case.scenarios.is_empty());
    assert!(use_case.prerequisites.is_empty());
    assert!(use_case.metadata.created_at.timestamp() > 0);
}

/// Test UseCase::status() returns correct aggregated status
#[test]
fn test_use_case_status_aggregation() {
    let mut use_case = create_test_use_case(
        "UC-002".to_string(),
        "Status Test".to_string(),
        "test".to_string(),
        "Testing status aggregation".to_string(),
    );

    // Empty use case should have Planned status
    assert_eq!(use_case.status(), Status::Planned);

    // Add scenarios with different statuses
    let scenario1 = Scenario::new(
        "SC-001".to_string(),
        "Scenario 1".to_string(),
        "Description 1".to_string(),
    );
    let mut scenario2 = Scenario::new(
        "SC-002".to_string(),
        "Scenario 2".to_string(),
        "Description 2".to_string(),
    );
    set_scenario_status(&mut scenario2, Status::InProgress);
    let mut scenario3 = Scenario::new(
        "SC-003".to_string(),
        "Scenario 3".to_string(),
        "Description 3".to_string(),
    );
    set_scenario_status(&mut scenario3, Status::Tested);

    use_case.add_scenario(scenario1);
    assert_eq!(use_case.status(), Status::Planned); // All planned

    use_case.add_scenario(scenario2);
    assert_eq!(use_case.status(), Status::InProgress); // Lowest non-planned

    use_case.add_scenario(scenario3);
    assert_eq!(use_case.status(), Status::InProgress); // Still lowest non-planned
}

/// Test UseCase::add_scenario() functionality
#[test]
fn test_use_case_add_scenario() {
    let mut use_case = create_test_use_case(
        "UC-004".to_string(),
        "Scenario Addition Test".to_string(),
        "test".to_string(),
        "Testing scenario addition".to_string(),
    );

    let original_updated_at = use_case.metadata.updated_at;

    let scenario = Scenario::new(
        "SC-004-001".to_string(),
        "First Scenario".to_string(),
        "First scenario description".to_string(),
    );

    use_case.add_scenario(scenario);

    assert_eq!(use_case.scenarios.len(), 1);
    assert_eq!(use_case.scenarios[0].id, "SC-004-001");
    assert_eq!(use_case.scenarios[0].title, "First Scenario");
    assert!(use_case.metadata.updated_at >= original_updated_at);
}

/// Test UseCase::update_scenario_status() functionality
#[test]
fn test_use_case_update_scenario_status() {
    let mut use_case = create_test_use_case(
        "UC-005".to_string(),
        "Scenario Status Update Test".to_string(),
        "test".to_string(),
        "Testing scenario status updates".to_string(),
    );

    let scenario = Scenario::new(
        "SC-005-001".to_string(),
        "Test Scenario".to_string(),
        "Test scenario for status update".to_string(),
    );

    use_case.add_scenario(scenario);
    let original_updated_at = use_case.metadata.updated_at;

    // Update existing scenario status
    let updated = use_case.update_scenario_status("SC-005-001", Status::Tested);
    assert!(updated);
    assert_eq!(use_case.scenarios[0].status, Status::Tested);
    assert!(use_case.metadata.updated_at >= original_updated_at);

    // Try to update non-existent scenario
    let not_updated = use_case.update_scenario_status("SC-005-999", Status::Deployed);
    assert!(!not_updated);
}

/// Test UseCase with multiple scenarios
#[test]
fn test_use_case_multiple_scenarios() {
    let mut use_case = create_test_use_case(
        "UC-007".to_string(),
        "Multiple Scenarios Test".to_string(),
        "test".to_string(),
        "Testing multiple scenarios".to_string(),
    );

    // Add multiple scenarios
    for i in 1..=5 {
        let scenario = Scenario::new(
            format!("SC-007-{:03}", i),
            format!("Scenario {}", i),
            format!("Description for scenario {}", i),
        );
        use_case.add_scenario(scenario);
    }

    assert_eq!(use_case.scenarios.len(), 5);

    // Verify all scenarios are present and ordered
    for (i, scenario) in use_case.scenarios.iter().enumerate() {
        let expected_id = format!("SC-007-{:03}", i + 1);
        assert_eq!(scenario.id, expected_id);
    }
}

/// Test UseCase clone functionality
#[test]
fn test_use_case_clone() {
    let mut use_case = create_test_use_case(
        "UC-008".to_string(),
        "Clone Test".to_string(),
        "test".to_string(),
        "Testing clone functionality".to_string(),
    );

    let scenario = Scenario::new(
        "SC-008-001".to_string(),
        "Clone Scenario".to_string(),
        "Scenario for clone test".to_string(),
    );
    use_case.add_scenario(scenario);

    let cloned = use_case.clone();

    assert_eq!(use_case.id, cloned.id);
    assert_eq!(use_case.title, cloned.title);
    assert_eq!(use_case.category, cloned.category);
    assert_eq!(use_case.description, cloned.description);
    assert_eq!(use_case.priority, cloned.priority);
    assert_eq!(use_case.scenarios.len(), cloned.scenarios.len());
}

/// Test UseCase serialization and deserialization
#[test]
fn test_use_case_serialization() {
    let mut use_case = create_test_use_case(
        "UC-009".to_string(),
        "Serialization Test".to_string(),
        "serialization".to_string(),
        "Testing serialization capabilities".to_string(),
    );

    let scenario = Scenario::new(
        "SC-009-001".to_string(),
        "Serialization Scenario".to_string(),
        "Scenario for serialization test".to_string(),
    );
    use_case.add_scenario(scenario);

    // Test JSON serialization
    let json = serde_json::to_string(&use_case).expect("Failed to serialize to JSON");
    let deserialized: UseCase =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    assert_eq!(use_case.id, deserialized.id);
    assert_eq!(use_case.title, deserialized.title);
    assert_eq!(use_case.category, deserialized.category);
    assert_eq!(use_case.scenarios.len(), deserialized.scenarios.len());
}

/// Test UseCase with complex scenario status combinations
#[test]
fn test_use_case_complex_status_scenarios() {
    let mut use_case = create_test_use_case(
        "UC-010".to_string(),
        "Complex Status Test".to_string(),
        "test".to_string(),
        "Testing complex status scenarios".to_string(),
    );

    // Test deprecated always wins
    let mut scenario1 = Scenario::new(
        "SC-1".to_string(),
        "Scenario 1".to_string(),
        "Desc 1".to_string(),
    );
    set_scenario_status(&mut scenario1, Status::Deployed);

    let mut scenario2 = Scenario::new(
        "SC-2".to_string(),
        "Scenario 2".to_string(),
        "Desc 2".to_string(),
    );
    set_scenario_status(&mut scenario2, Status::Deprecated);

    let mut scenario3 = Scenario::new(
        "SC-3".to_string(),
        "Scenario 3".to_string(),
        "Desc 3".to_string(),
    );
    set_scenario_status(&mut scenario3, Status::Tested);

    use_case.add_scenario(scenario1);
    use_case.add_scenario(scenario2);
    use_case.add_scenario(scenario3);

    assert_eq!(use_case.status(), Status::Deprecated);
}

/// Test UseCase with empty strings and edge cases
#[test]
fn test_use_case_edge_cases() {
    let use_case = create_test_use_case(
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );

    assert_eq!(use_case.id, "");
    assert_eq!(use_case.title, "");
    assert_eq!(use_case.category, "");
    assert_eq!(use_case.description, "");
    assert_eq!(use_case.status(), Status::Planned);
}

/// Test UseCase prerequisite functionality
#[test]
fn test_use_case_prerequisites() {
    let mut use_case = create_test_use_case(
        "UC-011".to_string(),
        "Prerequisites Test".to_string(),
        "test".to_string(),
        "Testing prerequisites functionality".to_string(),
    );

    assert!(use_case.prerequisites.is_empty());

    use_case
        .prerequisites
        .push("UC-001 must be completed".to_string());
    use_case
        .prerequisites
        .push("User authentication system".to_string());

    assert_eq!(use_case.prerequisites.len(), 2);
    assert!(use_case
        .prerequisites
        .contains(&"UC-001 must be completed".to_string()));
}
