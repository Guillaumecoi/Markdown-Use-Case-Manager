// tests/unit/use_case_service_test.rs
use crate::test_utils::{create_test_use_case, find_use_case_by_id};
use markdown_use_case_manager::core::domain::entities::Status;
use markdown_use_case_manager::core::domain::services::UseCaseService;

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

    let new_id =
        service.generate_unique_use_case_id("Security", &existing_use_cases, &temp_dir_str);
    assert!(new_id.starts_with("UC-SEC-"));
    assert!(new_id.len() > 7); // Should have format UC-SEC-XXX

    let api_id = service.generate_unique_use_case_id("API", &existing_use_cases, &temp_dir_str);
    assert!(api_id.starts_with("UC-API-"));

    let new_category_id =
        service.generate_unique_use_case_id("Database", &existing_use_cases, &temp_dir_str);
    assert!(new_category_id.starts_with("UC-DAT-"));
}

#[test]
fn test_status_parsing() {
    assert_eq!(Status::from_str("planned").unwrap(), Status::Planned);
    assert_eq!(Status::from_str("in_progress").unwrap(), Status::InProgress);
    assert_eq!(
        Status::from_str("implemented").unwrap(),
        Status::Implemented
    );
    assert_eq!(Status::from_str("tested").unwrap(), Status::Tested);
    assert_eq!(Status::from_str("deployed").unwrap(), Status::Deployed);
    assert_eq!(Status::from_str("deprecated").unwrap(), Status::Deprecated);

    // Test case insensitive
    assert_eq!(Status::from_str("PLANNED").unwrap(), Status::Planned);
    assert_eq!(Status::from_str("In_Progress").unwrap(), Status::InProgress);

    // Test invalid status
    assert!(Status::from_str("invalid").is_err());
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
