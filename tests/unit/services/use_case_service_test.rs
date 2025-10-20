// tests/unit/use_case_service_test.rs
use crate::test_utils::{create_test_use_case, find_use_case_by_id};
use markdown_use_case_manager::core::models::Status;
use markdown_use_case_manager::core::services::UseCaseService;

#[test]
fn test_use_case_service_unique_id_generation() {
    let service = UseCaseService::new();

    // Test unique ID generation with filesystem checks
    let existing_use_cases = vec![
        create_test_use_case(
            "UC-SEC-001".to_string(),
            "Login".to_string(),
            "Security".to_string(),
            "".to_string(),
        ),
        create_test_use_case(
            "UC-API-001".to_string(),
            "REST API".to_string(),
            "API".to_string(),
            "".to_string(),
        ),
    ];

    // Use a temporary directory for testing
    let temp_dir = std::env::temp_dir().join("mucm_test_use_case_service");
    let temp_dir_str = temp_dir.to_string_lossy();

    let new_id = service.generate_unique_use_case_id("Security", &existing_use_cases, &temp_dir_str);
    assert!(new_id.starts_with("UC-SEC-"));
    assert!(new_id.len() > 7); // Should have format UC-SEC-XXX

    let api_id = service.generate_unique_use_case_id("API", &existing_use_cases, &temp_dir_str);
    assert!(api_id.starts_with("UC-API-"));

    let new_category_id = service.generate_unique_use_case_id("Database", &existing_use_cases, &temp_dir_str);
    assert!(new_category_id.starts_with("UC-DAT-"));
}

#[test]
fn test_scenario_management() {
    let service = UseCaseService::new();
    let mut use_case = create_test_use_case(
        "UC-TEST-001".to_string(),
        "Test Use Case".to_string(),
        "Testing".to_string(),
        "A test use case".to_string(),
    );

    // Test scenario ID generation
    let scenario_id = service.generate_scenario_id(&use_case);
    assert_eq!(scenario_id, "UC-TEST-001-S01");

    // Test adding scenario
    let added_id = service.add_scenario_to_use_case(
        &mut use_case,
        "Test Scenario".to_string(),
        Some("A test scenario".to_string()),
    );
    assert_eq!(added_id, "UC-TEST-001-S01");
    assert_eq!(use_case.scenarios.len(), 1);
    assert_eq!(use_case.scenarios[0].title, "Test Scenario");

    // Test second scenario
    let second_id =
        service.add_scenario_to_use_case(&mut use_case, "Another Scenario".to_string(), None);
    assert_eq!(second_id, "UC-TEST-001-S02");
    assert_eq!(use_case.scenarios.len(), 2);
}

#[test]
fn test_status_parsing() {
    let service = UseCaseService::new();

    assert_eq!(service.parse_status("planned").unwrap(), Status::Planned);
    assert_eq!(
        service.parse_status("in_progress").unwrap(),
        Status::InProgress
    );
    assert_eq!(
        service.parse_status("implemented").unwrap(),
        Status::Implemented
    );
    assert_eq!(service.parse_status("tested").unwrap(), Status::Tested);
    assert_eq!(service.parse_status("deployed").unwrap(), Status::Deployed);
    assert_eq!(
        service.parse_status("deprecated").unwrap(),
        Status::Deprecated
    );

    // Test case insensitive
    assert_eq!(service.parse_status("PLANNED").unwrap(), Status::Planned);
    assert_eq!(
        service.parse_status("In_Progress").unwrap(),
        Status::InProgress
    );

    // Test invalid status
    assert!(service.parse_status("invalid").is_err());
}

#[test]
fn test_finding_use_cases() {
    let use_cases = vec![
        create_test_use_case(
            "UC-SEC-001".to_string(),
            "Login".to_string(),
            "Security".to_string(),
            "".to_string(),
        ),
        create_test_use_case(
            "UC-API-001".to_string(),
            "REST API".to_string(),
            "API".to_string(),
            "".to_string(),
        ),
    ];

    let found = find_use_case_by_id(&use_cases, "UC-SEC-001");
    assert!(found.is_some());
    assert_eq!(found.unwrap().title, "Login");

    let not_found = find_use_case_by_id(&use_cases, "UC-MISSING-001");
    assert!(not_found.is_none());
}

#[test]
fn test_scenario_status_update() {
    let service = UseCaseService::new();
    let mut use_case = create_test_use_case(
        "UC-TEST-001".to_string(),
        "Test Use Case".to_string(),
        "Testing".to_string(),
        "".to_string(),
    );

    // Add a scenario
    service.add_scenario_to_use_case(&mut use_case, "Test Scenario".to_string(), None);

    let scenario_id = "UC-TEST-001-S01";

    // Update status
    let result = service.update_scenario_status(&mut use_case, scenario_id, Status::InProgress);
    assert!(result.is_ok());
    assert_eq!(use_case.scenarios[0].status, Status::InProgress);

    // Try to update non-existent scenario
    let result = service.update_scenario_status(&mut use_case, "UC-INVALID-S01", Status::Tested);
    assert!(result.is_err());
}
