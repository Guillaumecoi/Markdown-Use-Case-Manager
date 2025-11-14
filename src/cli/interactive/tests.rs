//! Interactive runner tests
//!
//! Tests for the interactive CLI runner, focusing on basic workflows and coordination.
//!
//! ## Running Tests
//!
//! These tests modify global state and should be run with `cargo nextest run` for best results.
//! See the documentation in `controller/tests.rs` for more details.

#[cfg(test)]
mod interactive_runner_tests {
    use crate::cli::interactive::InteractiveRunner;
    use crate::config::{Config, ConfigFileManager};
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    /// Helper to create a test environment with initialized config
    fn setup_test_env() -> (TempDir, InteractiveRunner) {
        let temp_dir = TempDir::new().unwrap();
        env::set_current_dir(&temp_dir).unwrap();

        // Create a basic config
        let config = Config::default();
        ConfigFileManager::save_in_dir(&config, ".").unwrap();

        let runner = InteractiveRunner::new();
        (temp_dir, runner)
    }

    #[test]
    #[serial]
    fn test_new_runner_creation() {
        let runner = InteractiveRunner::new();
        // Should not panic and should create a valid runner
        drop(runner);
    }

    #[test]
    #[serial]
    fn test_get_available_languages() {
        let (_temp_dir, runner) = setup_test_env();

        let result = runner.get_available_languages();
        assert!(result.is_ok());
        let languages = result.unwrap();
        assert!(!languages.is_empty());
    }

    #[test]
    #[serial]
    fn test_get_available_methodologies() {
        let (_temp_dir, runner) = setup_test_env();

        let result = runner.get_available_methodologies();
        assert!(result.is_ok());
        let methodologies = result.unwrap();
        assert!(!methodologies.is_empty());
        
        // Verify methodology info structure
        for methodology in methodologies {
            assert!(!methodology.name.is_empty());
            assert!(!methodology.display_name.is_empty());
        }
    }

    #[test]
    #[serial]
    fn test_create_use_case_interactive() {
        let (_temp_dir, mut runner) = setup_test_env();

        let result = runner.create_use_case_interactive(
            "Test Use Case".to_string(),
            "test".to_string(),
            Some("Test description".to_string()),
            Some("business".to_string()),
        );

        assert!(result.is_ok());
        let message = result.unwrap();
        assert!(message.contains("Created use case"));
    }

    #[test]
    #[serial]
    fn test_create_use_case_without_methodology() {
        let (_temp_dir, mut runner) = setup_test_env();

        // Should use default methodology from config
        let result = runner.create_use_case_interactive(
            "Test Use Case".to_string(),
            "test".to_string(),
            None,
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_use_cases() {
        let (_temp_dir, mut runner) = setup_test_env();

        // Create a use case first
        runner
            .create_use_case_interactive(
                "Test UC".to_string(),
                "test".to_string(),
                None,
                Some("business".to_string()),
            )
            .unwrap();

        // List should not panic
        let result = runner.list_use_cases();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_show_status() {
        let (_temp_dir, mut runner) = setup_test_env();

        // Create some use cases
        runner
            .create_use_case_interactive(
                "Test UC 1".to_string(),
                "test".to_string(),
                None,
                Some("business".to_string()),
            )
            .unwrap();
        runner
            .create_use_case_interactive(
                "Test UC 2".to_string(),
                "test".to_string(),
                None,
                Some("developer".to_string()),
            )
            .unwrap();

        // Show status should not panic
        let result = runner.show_status();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_multiple_use_case_creation() {
        let (_temp_dir, mut runner) = setup_test_env();

        // Create multiple use cases
        for i in 1..=3 {
            let result = runner.create_use_case_interactive(
                format!("Test UC {}", i),
                "test".to_string(),
                Some(format!("Description {}", i)),
                Some("business".to_string()),
            );
            assert!(result.is_ok());
        }

        // Verify we can list them all
        let result = runner.list_use_cases();
        assert!(result.is_ok());
    }

    // TODO: Add tests for initialize_project workflow
    // TODO: Add tests for finalize_initialization workflow
    // TODO: Add tests for error handling in workflows
    // TODO: Add tests for state management across operations
}

#[cfg(test)]
mod workflow_tests {
    use crate::cli::interactive::InteractiveRunner;
    use crate::config::{Config, ConfigFileManager};
    use crate::controller::ProjectController;
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    fn setup_empty_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        temp_dir
    }

    #[test]
    #[serial]
    fn test_initialization_workflow() {
        let _temp_dir = setup_empty_dir();
        let mut runner = InteractiveRunner::new();

        // Step 1: Initialize project
        let result = runner.initialize_project(Some("rust".to_string()), "business".to_string());
        assert!(result.is_ok());
        let message = result.unwrap();
        assert!(message.contains("Configuration file created"));

        // Verify project is now initialized
        assert!(ProjectController::is_initialized());
    }

    #[test]
    #[serial]
    fn test_full_use_case_workflow() {
        let _temp_dir = setup_empty_dir();

        // Setup config
        let config = Config::default();
        ConfigFileManager::save_in_dir(&config, ".").unwrap();

        let mut runner = InteractiveRunner::new();

        // Create a use case
        let result = runner.create_use_case_interactive(
            "Login".to_string(),
            "authentication".to_string(),
            Some("User login workflow".to_string()),
            Some("business".to_string()),
        );
        assert!(result.is_ok());

        // List use cases
        let result = runner.list_use_cases();
        assert!(result.is_ok());

        // Show status
        let result = runner.show_status();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_workflow_with_different_methodologies() {
        let _temp_dir = setup_empty_dir();

        let config = Config::default();
        ConfigFileManager::save_in_dir(&config, ".").unwrap();

        let mut runner = InteractiveRunner::new();

        // Create use cases with different methodologies
        let methodologies = vec!["business", "developer", "feature", "tester"];
        
        for (i, methodology) in methodologies.iter().enumerate() {
            let result = runner.create_use_case_interactive(
                format!("UC {}", i + 1),
                "test".to_string(),
                None,
                Some(methodology.to_string()),
            );
            assert!(result.is_ok(), "Failed for methodology: {}", methodology);
        }
    }

    // TODO: Add tests for menu navigation workflows
    // TODO: Add tests for scenario creation workflows
    // TODO: Add tests for configuration workflows
    // TODO: Add tests for methodology selection workflows
}
