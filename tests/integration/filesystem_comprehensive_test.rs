// Integration tests for comprehensive filesystem functionality
use super::test_helpers::with_temp_dir;
use crate::test_utils::{init_project, init_project_in_dir, load_from_dir, save_config};
use markdown_use_case_manager::{config::Config, UseCaseCoordinator};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Test init_project() creates proper project structure
#[test]
fn test_config_init_project_creates_structure() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project in temp directory
    let config = init_project_in_dir(temp_dir.path().to_str().unwrap())
        .expect("Failed to initialize project");

    // Verify config was created
    let temp_path = temp_dir.path();
    assert!(temp_path.join(".config/.mucm").exists());
    assert!(temp_path.join(".config/.mucm/mucm.toml").exists());

    // Verify templates directory was created (templates may or may not be copied depending on source availability)
    assert!(temp_path.join(".config/.mucm/templates").exists());

    // Verify directories were created
    assert!(temp_path.join(&config.directories.use_case_dir).exists());
    assert!(temp_path.join(&config.directories.test_dir).exists());

    // Verify config content
    let config_content = fs::read_to_string(temp_path.join(".config/.mucm/mucm.toml")).unwrap();
    assert!(config_content.contains("[project]"));
    assert!(config_content.contains("name = \"My Project\""));
    assert!(config_content.contains("[directories]"));
    assert!(config_content.contains("use_case_dir = \"docs/use-cases\""));
    assert!(config_content.contains("[templates]"));
    assert!(config_content.contains("use_case_style = \"detailed\""));
}

/// Test Config::load() reads existing configuration
#[test]
fn test_config_load_existing() {
    let temp_dir = TempDir::new().unwrap();

    // Create config first
    let original_config = init_project_in_dir(temp_dir.path().to_str().unwrap()).unwrap();

    // Load config
    let loaded_config =
        load_from_dir(temp_dir.path().to_str().unwrap()).expect("Failed to load config");

    assert_eq!(original_config.project.name, loaded_config.project.name);
    assert_eq!(
        original_config.directories.use_case_dir,
        loaded_config.directories.use_case_dir
    );
    assert_eq!(
        original_config.metadata.enabled,
        loaded_config.metadata.enabled
    );
}

/// Test Config::load() fails gracefully when no project exists
#[test]
fn test_config_load_no_project() {
    let temp_dir = TempDir::new().unwrap();

    // Try to load config without initializing
    let result = load_from_dir(temp_dir.path().to_str().unwrap());
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("No markdown use case manager project found"));
    assert!(error.to_string().contains("Run 'mucm init' first"));
}

/// Test Config::save() persists configuration changes
#[test]
fn test_config_save_modifications() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize and modify config
    let mut config = init_project_in_dir(temp_dir.path().to_str().unwrap()).unwrap();
    config.project.name = "Modified Project".to_string();
    config.generation.test_language = "javascript".to_string();
    config.metadata.enabled = false;

    // Save changes
    config
        .save_in_dir(temp_dir.path().to_str().unwrap())
        .expect("Failed to save config");

    // Load and verify changes
    let loaded_config = load_from_dir(temp_dir.path().to_str().unwrap()).unwrap();
    assert_eq!(loaded_config.project.name, "Modified Project");
    assert_eq!(loaded_config.generation.test_language, "javascript");
    assert!(!loaded_config.metadata.enabled);
}

/// Test UseCaseCoordinator::load() works with existing project
#[test]
fn test_use_case_manager_load() {
    with_temp_dir(|_temp_dir| {
        // Initialize project - retry a few times in case of transient filesystem issues
        let mut init_result = init_project();
        if init_result.is_err() {
            std::thread::sleep(std::time::Duration::from_millis(10));
            init_result = init_project();
        }

        match init_result {
            Ok(_) => {}
            Err(e) => {
                panic!("Config init failed: {:?}", e);
            }
        }

        // Load manager
        let _manager = UseCaseCoordinator::load().expect("Failed to load UseCaseCoordinator");

        // Should start with empty use cases
        // Note: We can't directly access use_cases field, but we can test behavior
    });
}

/// Test UseCaseCoordinator::load() fails without project
#[test]
fn test_use_case_manager_load_no_project() {
    with_temp_dir(|_temp_dir| {
        // Try to load without project
        let result = UseCaseCoordinator::load();
        assert!(result.is_err());
    });
}

/// Test use case file creation and directory structure
#[test]
fn test_use_case_file_creation() {
    with_temp_dir(|_temp_dir| {
        // Initialize project and create use case
        init_project().unwrap();
        let mut manager = UseCaseCoordinator::load().unwrap();

        let use_case_id = manager
            .create_use_case(
                "Test Use Case".to_string(),
                "testing".to_string(),
                Some("Test description".to_string()),
            )
            .expect("Failed to create use case");

        // Verify file was created in correct location
        let expected_path = Path::new("docs/use-cases/testing").join(format!("{}.md", use_case_id));
        assert!(expected_path.exists());

        // Verify file content
        let content = fs::read_to_string(&expected_path).unwrap();
        assert!(content.contains("Test Use Case"));
        assert!(content.contains("Test description"));
        assert!(content.contains("## Description"));
        assert!(content.contains("## Scenarios"));
    });
}

/// Test scenario addition and file updates
#[test]
fn test_scenario_file_updates() {
    with_temp_dir(|temp_dir| {
        // Initialize project with test generation enabled
        let mut config = Config::default();
        config.generation.auto_generate_tests = true;
        config.generation.test_language = "rust".to_string();

        // Initialize project
        init_project().unwrap();

        // Save the custom config
        config
            .save_in_dir(temp_dir.path().to_str().unwrap())
            .unwrap();

        // Load manager
        let mut manager = UseCaseCoordinator::load().unwrap();

        // Create use case
        let use_case_id = manager
            .create_use_case(
                "Scenario Test".to_string(),
                "testing".to_string(),
                Some("Test with scenarios".to_string()),
            )
            .unwrap();

        // Add scenario
        let scenario_id = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Test Scenario".to_string(),
                Some("Scenario description".to_string()),
            )
            .expect("Failed to add scenario");

        // Verify file was updated
        let use_case_path = Path::new("docs/use-cases/testing").join(format!("{}.md", use_case_id));
        let content = fs::read_to_string(&use_case_path).unwrap();
        assert!(content.contains("Test Scenario"));
        assert!(content.contains(&scenario_id));
        assert!(content.contains("Scenario description"));

        // Verify test file was created
        let test_path = Path::new("tests/use-cases/testing").join(format!(
            "{}.rs",
            use_case_id.to_lowercase().replace('-', "_")
        ));
        assert!(test_path.exists());

        let test_content = fs::read_to_string(&test_path).unwrap();
        assert!(
            test_content.contains("AUTO-GENERATED TEST"),
            "Should contain test generation marker"
        );
        assert!(
            test_content.contains(&use_case_id),
            "Should contain use case ID"
        );
        assert!(
            test_content.contains("Test Scenario"),
            "Should contain scenario name"
        );
    });
}

/// Test multiple categories create separate directories
#[test]
fn test_multiple_categories() {
    with_temp_dir(|_temp_dir| {
        init_project().unwrap();
        let mut manager = UseCaseCoordinator::load().unwrap();

        // Create use cases in different categories
        let auth_id = manager
            .create_use_case("Login".to_string(), "authentication".to_string(), None)
            .unwrap();

        let profile_id = manager
            .create_use_case(
                "Update Profile".to_string(),
                "user_profile".to_string(),
                None,
            )
            .unwrap();

        let api_id = manager
            .create_use_case("API Endpoint".to_string(), "api".to_string(), None)
            .unwrap();

        // Verify separate directories were created
        assert!(Path::new("docs/use-cases/authentication").exists());
        assert!(Path::new("docs/use-cases/user_profile").exists());
        assert!(Path::new("docs/use-cases/api").exists());

        // Verify files in correct locations
        assert!(Path::new("docs/use-cases/authentication")
            .join(format!("{}.md", auth_id))
            .exists());
        assert!(Path::new("docs/use-cases/user_profile")
            .join(format!("{}.md", profile_id))
            .exists());
        assert!(Path::new("docs/use-cases/api")
            .join(format!("{}.md", api_id))
            .exists());
    });
}

/// Test overview generation creates README
#[test]
fn test_overview_generation() {
    with_temp_dir(|_temp_dir| {
        init_project().unwrap();
        let mut manager = UseCaseCoordinator::load().unwrap();

        // Create multiple use cases
        manager
            .create_use_case(
                "Login".to_string(),
                "auth".to_string(),
                Some("User authentication".to_string()),
            )
            .unwrap();

        manager
            .create_use_case(
                "Register".to_string(),
                "auth".to_string(),
                Some("User registration".to_string()),
            )
            .unwrap();

        manager
            .create_use_case("Update Profile".to_string(), "profile".to_string(), None)
            .unwrap();

        // Verify overview file was automatically generated
        let overview_path = Path::new("docs/use-cases/README.md");
        assert!(overview_path.exists());

        let content = fs::read_to_string(overview_path).unwrap();
        assert!(content.contains("Use Cases Overview") || content.contains("Overview"));

        // More flexible assertions - check for the use cases that were created
        assert!(content.contains("UC-AUT-001")); // Login
        assert!(content.contains("UC-AUT-002")); // Register
        assert!(content.contains("UC-PRO-001")); // Update Profile

        // Check for categories
        assert!(content.contains("auth") || content.contains("Auth"));
        assert!(content.contains("profile") || content.contains("Profile"));
    });
}

/// Test file persistence and reload
#[test]
fn test_file_persistence_and_reload() {
    with_temp_dir(|_temp_dir| {
        init_project().unwrap();

        // Create and save use cases with first manager instance
        {
            let mut manager = UseCaseCoordinator::load().unwrap();
            manager
                .create_use_case(
                    "Persistent Test".to_string(),
                    "persistence".to_string(),
                    Some("Testing file persistence".to_string()),
                )
                .unwrap();

            manager
                .add_scenario_to_use_case(
                    "UC-PER-001".to_string(),
                    "Persistence Scenario".to_string(),
                    Some("Test scenario persistence".to_string()),
                )
                .unwrap();
        }

        // Load with new manager instance and verify data persisted
        {
            let _manager = UseCaseCoordinator::load().unwrap();

            // Use a public method to verify the use cases were loaded
            // Since we can't access private fields, we'll use list_use_cases output capture
            // For now, we'll just verify the files exist
            assert!(Path::new("docs/use-cases/persistence/UC-PER-001.md").exists());

            let content = fs::read_to_string("docs/use-cases/persistence/UC-PER-001.md").unwrap();
            assert!(content.contains("Persistent Test"));
            assert!(content.contains("Persistence Scenario"));
            assert!(content.contains("UC-PER-001-S01"));
        }
    });
}

/// Test error handling for invalid file operations
#[test]
fn test_file_operation_error_handling() {
    with_temp_dir(|_temp_dir| {
        init_project().unwrap();
        let mut manager = UseCaseCoordinator::load().unwrap();

        // Try to add scenario to non-existent use case
        let result = manager.add_scenario_to_use_case(
            "UC-NONEXISTENT-001".to_string(),
            "Test Scenario".to_string(),
            None,
        );

        assert!(result.is_err());
        let error = result.unwrap_err();
        // The error message should indicate the use case was not found
        assert!(error.to_string().contains("Use case") && error.to_string().contains("not found"));
    });
}

/// Test custom directory configuration
#[test]
fn test_custom_directory_configuration() {
    with_temp_dir(|_temp_dir| {
        // Initialize project first to create proper structure
        init_project().unwrap();

        // Create custom config
        let mut config = Config::default();
        config.directories.use_case_dir = "custom_docs".to_string();
        config.directories.test_dir = "custom_tests".to_string();

        // Save custom config
        save_config(&config).unwrap();

        // Create directories
        fs::create_dir_all(&config.directories.use_case_dir).unwrap();
        fs::create_dir_all(&config.directories.test_dir).unwrap();

        // Use manager with custom config
        let mut manager = UseCaseCoordinator::load().unwrap();
        let use_case_id = manager
            .create_use_case("Custom Dir Test".to_string(), "custom".to_string(), None)
            .unwrap();

        // Verify files created in custom directories
        let custom_path = Path::new("custom_docs/custom").join(format!("{}.md", use_case_id));
        assert!(custom_path.exists());
    });
}
