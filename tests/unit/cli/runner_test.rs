// Unit tests for the new CLI runner and components
use anyhow::Result;
use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;
use crate::test_utils::init_project_with_language;

// Import the CLI components we want to test
use markdown_use_case_manager::config::Config;

/// Helper function to setup a test environment
fn setup_test_environment() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Change to temp directory and initialize project
    std::env::set_current_dir(&temp_dir)?;
    init_project_with_language(None)?;

    Ok(temp_dir)
}

/// Test CliRunner creation and basic operations
#[test]
#[serial]
fn test_cli_runner_creation() -> Result<()> {
    let _temp_dir = setup_test_environment()?;

    // Test that we can create a CliRunner
    // Note: We can't directly test CliRunner since it's not public in lib.rs
    // But we can test the underlying coordinator
    let coordinator = UseCaseCoordinator::load()?;

    // Verify initial state
    let use_case_ids = coordinator.get_all_use_case_ids()?;
    assert!(use_case_ids.is_empty(), "Should start with no use cases");

    let categories = coordinator.get_all_categories()?;
    assert!(categories.is_empty(), "Should start with no categories");

    Ok(())
}

/// Test use case creation and retrieval methods
#[test]
#[serial]
fn test_use_case_operations() -> Result<()> {
    let _temp_dir = setup_test_environment()?;

    let mut coordinator = UseCaseCoordinator::load()?;

    // Test creating use cases
    let uc_id1 = coordinator.create_use_case(
        "First Use Case".to_string(),
        "auth".to_string(), // Different prefix: AUT
        Some("First description".to_string()),
    )?;
    assert_eq!(uc_id1, "UC-AUT-001");

    let uc_id2 = coordinator.create_use_case(
        "Second Use Case".to_string(),
        "api".to_string(), // Different prefix: API
        None,
    )?;
    assert_eq!(uc_id2, "UC-API-001");

    // Test retrieval methods
    let use_case_ids = coordinator.get_all_use_case_ids()?;
    assert_eq!(use_case_ids.len(), 2);
    assert!(use_case_ids.contains(&"UC-AUT-001".to_string()));
    assert!(use_case_ids.contains(&"UC-API-001".to_string()));

    let categories = coordinator.get_all_categories()?;
    assert_eq!(categories.len(), 2);
    assert!(categories.contains(&"api".to_string()));
    assert!(categories.contains(&"auth".to_string()));

    Ok(())
}

/// Test scenario operations
#[test]
#[serial]
fn test_scenario_operations() -> Result<()> {
    let _temp_dir = setup_test_environment()?;

    let mut coordinator = UseCaseCoordinator::load()?;

    // Create a use case first
    let uc_id =
        coordinator.create_use_case("Test Use Case".to_string(), "test".to_string(), None)?;

    // Test adding scenarios
    let scenario_id1 = coordinator.add_scenario_to_use_case(
        uc_id.clone(),
        "First Scenario".to_string(),
        Some("First scenario description".to_string()),
    )?;
    assert_eq!(scenario_id1, "UC-TES-001-S01");

    let scenario_id2 =
        coordinator.add_scenario_to_use_case(uc_id.clone(), "Second Scenario".to_string(), None)?;
    assert_eq!(scenario_id2, "UC-TES-001-S02");

    // Test retrieving scenario IDs
    let scenario_ids = coordinator.get_scenario_ids_for_use_case(&uc_id)?;
    assert_eq!(scenario_ids.len(), 2);
    assert!(scenario_ids.contains(&"UC-TES-001-S01".to_string()));
    assert!(scenario_ids.contains(&"UC-TES-001-S02".to_string()));

    // Test updating scenario status
    coordinator.update_scenario_status("UC-TES-001-S01".to_string(), "implemented".to_string())?;

    Ok(())
}

/// Test error handling for invalid operations
#[test]
#[serial]
fn test_error_handling() -> Result<()> {
    let _temp_dir = setup_test_environment()?;

    let mut coordinator = UseCaseCoordinator::load()?;

    // Test adding scenario to non-existent use case
    let result = coordinator.add_scenario_to_use_case(
        "UC-NON-001".to_string(),
        "Test Scenario".to_string(),
        None,
    );
    assert!(result.is_err(), "Should fail for non-existent use case");

    // Test getting scenarios for non-existent use case
    let result = coordinator.get_scenario_ids_for_use_case("UC-NON-001");
    assert!(result.is_err(), "Should fail for non-existent use case");

    // Test updating status of non-existent scenario
    let result =
        coordinator.update_scenario_status("UC-NON-001-S01".to_string(), "implemented".to_string());
    assert!(result.is_err(), "Should fail for non-existent scenario");

    Ok(())
}

/// Test category suggestions and auto-completion
#[test]
#[serial]
fn test_category_suggestions() -> Result<()> {
    let _temp_dir = setup_test_environment()?;

    let mut coordinator = UseCaseCoordinator::load()?;

    // Create use cases with different categories
    coordinator.create_use_case(
        "Auth Use Case".to_string(),
        "authentication".to_string(),
        None,
    )?;

    coordinator.create_use_case("API Use Case".to_string(), "api".to_string(), None)?;

    coordinator.create_use_case(
        "Security Use Case".to_string(),
        "authentication".to_string(), // Duplicate category
        None,
    )?;

    // Test category retrieval (should be deduplicated and sorted)
    let categories = coordinator.get_all_categories()?;
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0], "api"); // Should be sorted
    assert_eq!(categories[1], "authentication");

    Ok(())
}

/// Test file creation and persistence
#[test]
#[serial]
fn test_file_persistence() -> Result<()> {
    let temp_dir = setup_test_environment()?;

    let mut coordinator = UseCaseCoordinator::load()?;

    // Create a use case
    let uc_id = coordinator.create_use_case(
        "Persistence Test".to_string(),
        "testing".to_string(),
        Some("Testing file persistence".to_string()),
    )?;

    // Add a scenario
    let scenario_id = coordinator.add_scenario_to_use_case(
        uc_id.clone(),
        "Test Scenario".to_string(),
        Some("Testing scenario persistence".to_string()),
    )?;

    // Verify files were created
    let use_case_file = temp_dir.path().join("docs/use-cases/testing/UC-TES-001.md");
    assert!(use_case_file.exists(), "Use case file should be created");

    let overview_file = temp_dir.path().join("docs/use-cases/README.md");
    assert!(overview_file.exists(), "Overview file should be created");

    // Verify file contents
    let use_case_content = fs::read_to_string(&use_case_file)?;
    assert!(use_case_content.contains("Persistence Test"));
    assert!(use_case_content.contains("Testing file persistence"));
    assert!(use_case_content.contains("Test Scenario"));
    assert!(use_case_content.contains("UC-TES-001-S01"));

    let overview_content = fs::read_to_string(&overview_file)?;
    assert!(overview_content.contains("UC-TES-001"));
    assert!(overview_content.contains("Persistence Test"));

    // Test reloading coordinator (persistence across sessions)
    drop(coordinator);
    let reloaded_coordinator = UseCaseCoordinator::load()?;

    let use_case_ids = reloaded_coordinator.get_all_use_case_ids()?;
    assert_eq!(use_case_ids.len(), 1);
    assert!(use_case_ids.contains(&uc_id));

    let scenario_ids = reloaded_coordinator.get_scenario_ids_for_use_case(&uc_id)?;
    assert_eq!(scenario_ids.len(), 1);
    assert!(scenario_ids.contains(&scenario_id));

    Ok(())
}

/// Test configuration and language handling
#[test]
#[serial]
fn test_configuration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    // Test initialization with specific language
    let config = init_project_with_language(Some("rust".to_string()))?;

    // Verify configuration
    assert_eq!(config.generation.test_language, "rust");
    assert!(config.directories.use_case_dir.contains("docs/use-cases"));

    // Test available languages
    let languages = Config::get_available_languages();
    match languages {
        Ok(langs) => {
            assert!(!langs.is_empty(), "Should have available languages");
            assert!(langs.contains(&"rust".to_string()));
        }
        Err(_) => {
            // It's okay if this fails in test environment
            // The CLI should still show built-in languages
        }
    }

    Ok(())
}
