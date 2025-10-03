// tests/unit/use_case_service_test.rs
use markdown_use_case_manager::core::models::{Status, UseCase};
use markdown_use_case_manager::core::services::UseCaseService;
use crate::test_utils::find_use_case_by_id;

#[test]
fn test_use_case_service_pure_business_logic() {
    let service = UseCaseService::new();
    
    // Test ID generation
    let existing_use_cases = vec![
        UseCase::new("UC-SEC-001".to_string(), "Login".to_string(), "Security".to_string(), "".to_string()),
        UseCase::new("UC-API-001".to_string(), "REST API".to_string(), "API".to_string(), "".to_string()),
    ];
    
    let new_id = service.generate_use_case_id("Security", &existing_use_cases);
    assert_eq!(new_id, "UC-SEC-002");
    
    let api_id = service.generate_use_case_id("API", &existing_use_cases);
    assert_eq!(api_id, "UC-API-002");
    
    let new_category_id = service.generate_use_case_id("Database", &existing_use_cases);
    assert_eq!(new_category_id, "UC-DAT-001");
}

#[test]
fn test_scenario_management() {
    let service = UseCaseService::new();
    let mut use_case = UseCase::new(
        "UC-TEST-001".to_string(),
        "Test Use Case".to_string(), 
        "Testing".to_string(),
        "A test use case".to_string()
    );
    
    // Test scenario ID generation
    let scenario_id = service.generate_scenario_id(&use_case);
    assert_eq!(scenario_id, "UC-TEST-001-S01");
    
    // Test adding scenario
    let added_id = service.add_scenario_to_use_case(
        &mut use_case,
        "Test Scenario".to_string(),
        Some("A test scenario".to_string())
    );
    assert_eq!(added_id, "UC-TEST-001-S01");
    assert_eq!(use_case.scenarios.len(), 1);
    assert_eq!(use_case.scenarios[0].title, "Test Scenario");
    
    // Test second scenario
    let second_id = service.add_scenario_to_use_case(
        &mut use_case,
        "Another Scenario".to_string(),
        None
    );
    assert_eq!(second_id, "UC-TEST-001-S02");
    assert_eq!(use_case.scenarios.len(), 2);
}

#[test]
fn test_status_parsing() {
    let service = UseCaseService::new();
    
    assert_eq!(service.parse_status("planned").unwrap(), Status::Planned);
    assert_eq!(service.parse_status("in_progress").unwrap(), Status::InProgress);
    assert_eq!(service.parse_status("implemented").unwrap(), Status::Implemented);
    assert_eq!(service.parse_status("tested").unwrap(), Status::Tested);
    assert_eq!(service.parse_status("deployed").unwrap(), Status::Deployed);
    assert_eq!(service.parse_status("deprecated").unwrap(), Status::Deprecated);
    
    // Test case insensitive
    assert_eq!(service.parse_status("PLANNED").unwrap(), Status::Planned);
    assert_eq!(service.parse_status("In_Progress").unwrap(), Status::InProgress);
    
    // Test invalid status
    assert!(service.parse_status("invalid").is_err());
}

#[test]
fn test_finding_use_cases() {
    let use_cases = vec![
        UseCase::new("UC-SEC-001".to_string(), "Login".to_string(), "Security".to_string(), "".to_string()),
        UseCase::new("UC-API-001".to_string(), "REST API".to_string(), "API".to_string(), "".to_string()),
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
    let mut use_case = UseCase::new(
        "UC-TEST-001".to_string(),
        "Test Use Case".to_string(),
        "Testing".to_string(), 
        "".to_string()
    );
    
    // Add a scenario
    service.add_scenario_to_use_case(
        &mut use_case,
        "Test Scenario".to_string(),
        None
    );
    
    let scenario_id = "UC-TEST-001-S01";
    
    // Update status
    let result = service.update_scenario_status(&mut use_case, scenario_id, Status::InProgress);
    assert!(result.is_ok());
    assert_eq!(use_case.scenarios[0].status, Status::InProgress);
    
    // Try to update non-existent scenario
    let result = service.update_scenario_status(&mut use_case, "UC-INVALID-S01", Status::Tested);
    assert!(result.is_err());
}