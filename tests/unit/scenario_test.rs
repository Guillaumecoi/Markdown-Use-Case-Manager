// Unit tests for Scenario struct and related functionality
use use_case_manager::core::models::{Scenario, Status};

/// Test Scenario::new() creates scenario with correct initial values
#[test]
fn test_scenario_new() {
    let scenario = Scenario::new(
        "SC-001".to_string(),
        "Test Scenario".to_string(),
        "Test description for scenario".to_string()
    );
    
    assert_eq!(scenario.id, "SC-001");
    assert_eq!(scenario.title, "Test Scenario");
    assert_eq!(scenario.description, "Test description for scenario");
    assert_eq!(scenario.status, Status::Planned);
    assert!(scenario.test_file.is_none());
    
    // Metadata should be initialized
    assert!(scenario.metadata.created_at.timestamp() > 0);
}

/// Test Scenario::set_status() updates status and metadata
#[test]
fn test_scenario_set_status() {
    let mut scenario = Scenario::new(
        "SC-002".to_string(),
        "Status Test Scenario".to_string(),
        "Testing status updates".to_string()
    );
    
    let original_updated = scenario.metadata.updated_at;
    
    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    scenario.set_status(Status::InProgress);
    
    assert_eq!(scenario.status, Status::InProgress);
    assert!(scenario.metadata.updated_at > original_updated);
}

/// Test multiple status updates
#[test]
fn test_scenario_multiple_status_updates() {
    let mut scenario = Scenario::new(
        "SC-003".to_string(),
        "Multi Status Test".to_string(),
        "Testing multiple status changes".to_string()
    );
    
    let mut last_updated = scenario.metadata.updated_at;
    
    let statuses = [Status::InProgress,
        Status::Implemented,
        Status::Tested,
        Status::Deployed];
    
    for status in statuses.iter() {
        scenario.set_status(*status);
        assert_eq!(scenario.status, *status);
        // Verify timestamp gets updated with each status change
        assert!(scenario.metadata.updated_at >= last_updated);
        last_updated = scenario.metadata.updated_at;
    }
}

/// Test scenario with empty strings
#[test]
fn test_scenario_empty_strings() {
    let scenario = Scenario::new(
        "".to_string(),
        "".to_string(),
        "".to_string()
    );
    
    assert_eq!(scenario.id, "");
    assert_eq!(scenario.title, "");
    assert_eq!(scenario.description, "");
    assert_eq!(scenario.status, Status::Planned);
}

/// Test scenario with long strings
#[test]
fn test_scenario_long_strings() {
    let long_id = "SC-VERY-LONG-ID-WITH-MANY-CHARACTERS-001".to_string();
    let long_title = "A Very Long Title That Describes A Complex Scenario With Multiple Components And Detailed Information".to_string();
    let long_description = "This is an extremely long description that contains a lot of detailed information about the scenario including background context, specific requirements, expected outcomes, and various edge cases that need to be considered during implementation and testing phases of the development lifecycle.".to_string();
    
    let scenario = Scenario::new(long_id.clone(), long_title.clone(), long_description.clone());
    
    assert_eq!(scenario.id, long_id);
    assert_eq!(scenario.title, long_title);
    assert_eq!(scenario.description, long_description);
}

/// Test scenario test_file field functionality
#[test]
fn test_scenario_test_file() {
    let mut scenario = Scenario::new(
        "SC-004".to_string(),
        "Test File Scenario".to_string(),
        "Testing test file association".to_string()
    );
    
    assert!(scenario.test_file.is_none());
    
    scenario.test_file = Some("test_scenario_004.rs".to_string());
    assert_eq!(scenario.test_file, Some("test_scenario_004.rs".to_string()));
    
    scenario.test_file = None;
    assert!(scenario.test_file.is_none());
}

/// Test scenario clone functionality
#[test]
fn test_scenario_clone() {
    let mut scenario = Scenario::new(
        "SC-005".to_string(),
        "Clone Test".to_string(),
        "Testing clone functionality".to_string()
    );
    
    scenario.set_status(Status::Tested);
    scenario.test_file = Some("test_file.rs".to_string());
    
    let cloned = scenario.clone();
    
    assert_eq!(scenario.id, cloned.id);
    assert_eq!(scenario.title, cloned.title);
    assert_eq!(scenario.description, cloned.description);
    assert_eq!(scenario.status, cloned.status);
    assert_eq!(scenario.test_file, cloned.test_file);
    assert_eq!(scenario.metadata.created_at, cloned.metadata.created_at);
    assert_eq!(scenario.metadata.updated_at, cloned.metadata.updated_at);
}

/// Test scenario serialization and deserialization
#[test]
fn test_scenario_serialization() {
    let mut scenario = Scenario::new(
        "SC-006".to_string(),
        "Serialization Test".to_string(),
        "Testing serialization capabilities".to_string()
    );
    
    scenario.set_status(Status::Implemented);
    scenario.test_file = Some("serialization_test.rs".to_string());
    
    // Test serialization (used internally)
    let json = serde_json::to_string(&scenario).expect("Failed to serialize");
    let deserialized: Scenario = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(scenario.id, deserialized.id);
    assert_eq!(scenario.title, deserialized.title);
    assert_eq!(scenario.description, deserialized.description);
    assert_eq!(scenario.status, deserialized.status);
    assert_eq!(scenario.test_file, deserialized.test_file);
    assert_eq!(scenario.metadata.created_at, deserialized.metadata.created_at);
    assert_eq!(scenario.metadata.updated_at, deserialized.metadata.updated_at);
}

/// Test scenario debug formatting
#[test]
fn test_scenario_debug() {
    let scenario = Scenario::new(
        "SC-007".to_string(),
        "Debug Test".to_string(),
        "Testing debug output".to_string()
    );
    
    let debug_str = format!("{:?}", scenario);
    
    assert!(debug_str.contains("Scenario"));
    assert!(debug_str.contains("SC-007"));
    assert!(debug_str.contains("Debug Test"));
    assert!(debug_str.contains("Testing debug output"));
}

/// Test scenario with special characters
#[test]
fn test_scenario_special_characters() {
    let scenario = Scenario::new(
        "SC-SPECIAL-001".to_string(),
        "Test with \"quotes\" & <brackets>".to_string(),
        "Description with 'single quotes', newlines\nand tabs\t".to_string()
    );
    
    assert_eq!(scenario.id, "SC-SPECIAL-001");
    assert!(scenario.title.contains("\"quotes\""));
    assert!(scenario.title.contains("<brackets>"));
    assert!(scenario.description.contains("'single quotes'"));
    assert!(scenario.description.contains("\n"));
    assert!(scenario.description.contains("\t"));
}

/// Test scenario status transitions follow business logic
#[test]
fn test_scenario_status_transitions() {
    let mut scenario = Scenario::new(
        "SC-008".to_string(),
        "Status Transition Test".to_string(),
        "Testing realistic status transitions".to_string()
    );
    
    // Typical workflow: Planned -> InProgress -> Implemented -> Tested -> Deployed
    assert_eq!(scenario.status, Status::Planned);
    
    scenario.set_status(Status::InProgress);
    assert_eq!(scenario.status, Status::InProgress);
    
    scenario.set_status(Status::Implemented);
    assert_eq!(scenario.status, Status::Implemented);
    
    scenario.set_status(Status::Tested);
    assert_eq!(scenario.status, Status::Tested);
    
    scenario.set_status(Status::Deployed);
    assert_eq!(scenario.status, Status::Deployed);
    
    // Can also go backwards or skip steps
    scenario.set_status(Status::InProgress);
    assert_eq!(scenario.status, Status::InProgress);
    
    scenario.set_status(Status::Deprecated);
    assert_eq!(scenario.status, Status::Deprecated);
}