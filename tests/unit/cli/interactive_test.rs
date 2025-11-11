// Unit tests for CLI interactive menu logic
use markdown_use_case_manager::UseCaseApplicationService;
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
    let mut coordinator = UseCaseApplicationService::load()?;

    // Simulate interactive create use case workflow
    let use_case_id = coordinator.create_use_case_with_methodology(
        "Interactive Test".to_string(),
        "testing".to_string(),
        Some("Created via interactive mode".to_string()),
        "feature",
    )?;
    assert_eq!(use_case_id, "UC-TES-001");

    // Simulate getting use case IDs for selection (what the interactive menu would show)
    let use_case_ids = coordinator.get_all_use_case_ids()?;
    assert_eq!(use_case_ids.len(), 1);
    assert!(use_case_ids.contains(&"UC-TES-001".to_string()));

    // Verify the workflow worked
    let final_use_case_ids = coordinator.get_all_use_case_ids()?;
    assert_eq!(final_use_case_ids.len(), 1);

    Ok(())
}

/// Test category suggestions logic for interactive mode
#[test]
#[serial]
fn test_interactive_category_suggestions() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;

    crate::test_utils::init_project_with_language(None)?;
    let mut coordinator = UseCaseApplicationService::load()?;

    // Initially no categories
    let categories = coordinator.get_all_categories()?;
    assert!(categories.is_empty());

    // Create use cases with different categories
    coordinator.create_use_case_with_methodology(
        "Auth Use Case".to_string(),
        "authentication".to_string(),
        None,
        "feature",
    )?;

    coordinator.create_use_case_with_methodology(
        "API Use Case".to_string(),
        "api".to_string(),
        None,
        "feature",
    )?;

    coordinator.create_use_case_with_methodology(
        "Another Auth Use Case".to_string(),
        "authentication".to_string(), // Duplicate
        None,
        "feature",
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

/// Test complete interactive workflow simulation
#[test]
#[serial]
fn test_complete_interactive_workflow_simulation() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;

    // Simulate: mucm init
    crate::test_utils::init_project_with_language(Some("rust".to_string()))?;

    // Simulate: enter interactive mode
    let mut coordinator = UseCaseApplicationService::load()?;

    // Simulate: interactive workflow - create multiple use cases
    let _uc1 = coordinator.create_use_case_with_methodology(
        "User Authentication".to_string(),
        "auth".to_string(),
        Some("Handle user login and logout".to_string()),
        "feature",
    )?;

    let _uc2 = coordinator.create_use_case_with_methodology(
        "Data Export".to_string(),
        "api".to_string(),
        Some("Export data in various formats".to_string()),
        "feature",
    )?;

    // Verify final state
    let all_use_cases = coordinator.get_all_use_case_ids()?;
    assert_eq!(all_use_cases.len(), 2);

    let categories = coordinator.get_all_categories()?;
    assert_eq!(categories.len(), 2);
    assert!(categories.contains(&"api".to_string()));
    assert!(categories.contains(&"auth".to_string()));

    Ok(())
}
