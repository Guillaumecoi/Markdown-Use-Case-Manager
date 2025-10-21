// Unit tests for CLI interactive menu logic
use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
use serial_test::serial;
use std::env;
use tempfile::TempDir;

/// Test that we can create a coordinator and simulate the CliRunner operations
#[test]
#[serial]
fn test_interactive_workflow_simulation() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;

    // Initialize project (simulates CLI init)
    crate::test_utils::init_project_with_language(None)?;

    // Load coordinator (simulates CliRunner::new())
    let mut coordinator = UseCaseCoordinator::load()?;

    // Simulate interactive create use case workflow
    let use_case_id = coordinator.create_use_case(
        "Interactive Test".to_string(),
        "testing".to_string(),
        Some("Created via interactive mode".to_string()),
    )?;
    assert_eq!(use_case_id, "UC-TES-001");

    // Simulate getting use case IDs for selection (what the interactive menu would show)
    let use_case_ids = coordinator.get_all_use_case_ids()?;
    assert_eq!(use_case_ids.len(), 1);
    assert!(use_case_ids.contains(&"UC-TES-001".to_string()));

    // Simulate adding scenario through interactive workflow
    let scenario_id = coordinator.add_scenario_to_use_case(
        "UC-TES-001".to_string(),
        "Interactive Scenario".to_string(),
        Some("Added via interactive mode".to_string()),
    )?;
    assert_eq!(scenario_id, "UC-TES-001-S01");

    // Simulate getting scenario IDs for selection
    let scenario_ids = coordinator.get_scenario_ids_for_use_case("UC-TES-001")?;
    assert_eq!(scenario_ids.len(), 1);
    assert!(scenario_ids.contains(&"UC-TES-001-S01".to_string()));

    // Simulate updating status through interactive workflow
    coordinator.update_scenario_status("UC-TES-001-S01".to_string(), "implemented".to_string())?;

    // Verify the workflow worked
    let final_use_case_ids = coordinator.get_all_use_case_ids()?;
    assert_eq!(final_use_case_ids.len(), 1);

    let final_scenario_ids = coordinator.get_scenario_ids_for_use_case("UC-TES-001")?;
    assert_eq!(final_scenario_ids.len(), 1);

    Ok(())
}

/// Test category suggestions logic for interactive mode
#[test]
#[serial]
fn test_interactive_category_suggestions() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;

    crate::test_utils::init_project_with_language(None)?;
    let mut coordinator = UseCaseCoordinator::load()?;

    // Initially no categories
    let categories = coordinator.get_all_categories()?;
    assert!(categories.is_empty());

    // Create use cases with different categories
    coordinator.create_use_case(
        "Auth Use Case".to_string(),
        "authentication".to_string(),
        None,
    )?;

    coordinator.create_use_case("API Use Case".to_string(), "api".to_string(), None)?;

    coordinator.create_use_case(
        "Another Auth Use Case".to_string(),
        "authentication".to_string(), // Duplicate
        None,
    )?;

    // Test category suggestions (should be deduplicated and sorted)
    let categories = coordinator.get_all_categories()?;
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0], "api");
    assert_eq!(categories[1], "authentication");

    // Test use case selection options
    let use_case_ids = coordinator.get_all_use_case_ids()?;
    assert_eq!(use_case_ids.len(), 3);
    assert!(use_case_ids.contains(&"UC-AUT-001".to_string()));
    assert!(use_case_ids.contains(&"UC-API-001".to_string()));
    assert!(use_case_ids.contains(&"UC-AUT-002".to_string()));

    Ok(())
}

/// Test error scenarios that interactive mode would handle
#[test]
#[serial]
fn test_interactive_error_scenarios() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;

    crate::test_utils::init_project_with_language(None)?;
    let mut coordinator = UseCaseCoordinator::load()?;

    // Test scenario: try to add scenario when no use cases exist
    let use_case_ids = coordinator.get_all_use_case_ids()?;
    assert!(use_case_ids.is_empty());

    // This would be caught by interactive mode to show "No use cases found"
    let result =
        coordinator.add_scenario_to_use_case("UC-NON-001".to_string(), "Test".to_string(), None);
    assert!(result.is_err());

    // Test scenario: try to update scenario status when no scenarios exist
    let result =
        coordinator.update_scenario_status("UC-NON-001-S01".to_string(), "implemented".to_string());
    assert!(result.is_err());

    // Create a use case first
    coordinator.create_use_case("Test Use Case".to_string(), "test".to_string(), None)?;

    // Now use cases exist but no scenarios
    let scenario_ids = coordinator.get_scenario_ids_for_use_case("UC-TES-001")?;
    assert!(scenario_ids.is_empty());

    // This would be caught by interactive mode to show "No scenarios found"
    let result =
        coordinator.update_scenario_status("UC-TES-001-S01".to_string(), "implemented".to_string());
    assert!(result.is_err());

    Ok(())
}

/// Test complete interactive workflow simulation
#[test]
#[serial]
fn test_complete_interactive_workflow_simulation() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;

    // Simulate: mucm init
    crate::test_utils::init_project_with_language(Some("rust".to_string()))?;

    // Simulate: enter interactive mode
    let mut coordinator = UseCaseCoordinator::load()?;

    // Simulate: interactive workflow - create multiple use cases
    let uc1 = coordinator.create_use_case(
        "User Authentication".to_string(),
        "auth".to_string(),
        Some("Handle user login and logout".to_string()),
    )?;

    let uc2 = coordinator.create_use_case(
        "Data Export".to_string(),
        "api".to_string(),
        Some("Export data in various formats".to_string()),
    )?;

    // Simulate: add scenarios to first use case
    let s1 = coordinator.add_scenario_to_use_case(
        uc1.clone(),
        "Happy Path Login".to_string(),
        Some("User provides valid credentials".to_string()),
    )?;

    let s2 = coordinator.add_scenario_to_use_case(
        uc1.clone(),
        "Invalid Credentials".to_string(),
        Some("User provides invalid credentials".to_string()),
    )?;

    // Simulate: add scenario to second use case
    let s3 =
        coordinator.add_scenario_to_use_case(uc2.clone(), "Export as CSV".to_string(), None)?;

    // Simulate: update statuses
    coordinator.update_scenario_status(s1, "implemented".to_string())?;
    coordinator.update_scenario_status(s2, "tested".to_string())?;
    coordinator.update_scenario_status(s3, "planned".to_string())?;

    // Verify final state
    let all_use_cases = coordinator.get_all_use_case_ids()?;
    assert_eq!(all_use_cases.len(), 2);

    let categories = coordinator.get_all_categories()?;
    assert_eq!(categories.len(), 2);
    assert!(categories.contains(&"api".to_string()));
    assert!(categories.contains(&"auth".to_string()));

    let auth_scenarios = coordinator.get_scenario_ids_for_use_case(&uc1)?;
    assert_eq!(auth_scenarios.len(), 2);

    let api_scenarios = coordinator.get_scenario_ids_for_use_case(&uc2)?;
    assert_eq!(api_scenarios.len(), 1);

    Ok(())
}
