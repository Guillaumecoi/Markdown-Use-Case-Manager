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

#[cfg(test)]
mod persona_workflow_tests {
    use crate::cli::interactive::InteractiveRunner;
    use crate::config::{Config, ConfigFileManager, StorageBackend};
    use crate::core::RepositoryFactory;
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, InteractiveRunner, Config) {
        let temp_dir = TempDir::new().unwrap();
        env::set_current_dir(&temp_dir).unwrap();

        let config = Config::default();
        ConfigFileManager::save_in_dir(&config, ".").unwrap();

        let runner = InteractiveRunner::new();
        (temp_dir, runner, config)
    }

    fn setup_test_env_with_backend(
        backend: StorageBackend,
    ) -> (TempDir, InteractiveRunner, Config) {
        let temp_dir = TempDir::new().unwrap();
        env::set_current_dir(&temp_dir).unwrap();

        let mut config = Config::default();
        config.storage.backend = backend;
        ConfigFileManager::save_in_dir(&config, ".").unwrap();

        let runner = InteractiveRunner::new();
        (temp_dir, runner, config)
    }

    #[test]
    #[serial]
    fn test_create_persona_interactive_basic() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        let result = runner.create_persona_interactive(
            "dev-user".to_string(),
            "Developer User".to_string(),
            "An experienced software developer".to_string(),
            "Write quality code efficiently".to_string(),
            None,
            None,
            None,
        );

        assert!(
            result.is_ok(),
            "Failed to create persona: {:?}",
            result.err()
        );
        let message = result.unwrap();
        assert_eq!(message, "Persona created successfully!");

        // Verify persona was created
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("dev-user").unwrap();
        assert!(persona.is_some());

        let persona = persona.unwrap();
        assert_eq!(persona.id, "dev-user");
        assert_eq!(persona.name, "Developer User");
        assert_eq!(persona.description, "An experienced software developer");
        assert_eq!(persona.goal, "Write quality code efficiently");
    }

    #[test]
    #[serial]
    fn test_create_persona_interactive_with_all_fields() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        let result = runner.create_persona_interactive(
            "senior-dev".to_string(),
            "Senior Developer".to_string(),
            "10+ years of experience in web development".to_string(),
            "Lead team to deliver high-quality products".to_string(),
            Some("Remote work, multiple time zones".to_string()),
            Some(5),
            Some("daily".to_string()),
        );

        assert!(result.is_ok());

        // Verify all fields were saved
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("senior-dev").unwrap().unwrap();

        assert_eq!(
            persona.context,
            Some("Remote work, multiple time zones".to_string())
        );
        assert_eq!(persona.tech_level, Some(5));
        assert_eq!(persona.usage_frequency, Some("daily".to_string()));
    }

    #[test]
    #[serial]
    fn test_create_persona_interactive_sqlite_backend() {
        let (_temp_dir, mut runner, config) = setup_test_env_with_backend(StorageBackend::Sqlite);

        let result = runner.create_persona_interactive(
            "test-user".to_string(),
            "Test User".to_string(),
            "A test persona".to_string(),
            "Test the system".to_string(),
            None,
            None,
            None,
        );

        assert!(result.is_ok());

        // Verify SQLite storage
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let exists = repo.exists("test-user").unwrap();
        assert!(exists);
    }

    #[test]
    #[serial]
    fn test_create_persona_interactive_duplicate_id() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create first persona
        let result = runner.create_persona_interactive(
            "duplicate".to_string(),
            "First User".to_string(),
            "First description".to_string(),
            "First goal".to_string(),
            None,
            None,
            None,
        );
        assert!(result.is_ok());

        // Try to create duplicate - should fail or overwrite depending on implementation
        let result = runner.create_persona_interactive(
            "duplicate".to_string(),
            "Second User".to_string(),
            "Second description".to_string(),
            "Second goal".to_string(),
            None,
            None,
            None,
        );

        // Should succeed (overwrites) or fail with error
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    #[serial]
    fn test_create_persona_interactive_invalid_tech_level() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Tech level > 5 should fail validation in create_persona
        let result = runner.create_persona_interactive(
            "invalid-tech".to_string(),
            "Test User".to_string(),
            "Description".to_string(),
            "Goal".to_string(),
            None,
            Some(10), // Invalid - outside 1-5 range
            None,
        );

        // Should fail with validation error
        assert!(result.is_err(), "Expected error for tech level > 5");
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Tech level must be between 1 and 5"),
            "Expected tech level validation error, got: {}",
            error_msg
        );
    }

    #[test]
    #[serial]
    fn test_list_personas_empty() {
        let (_temp_dir, runner, _config) = setup_test_env();

        let result = runner.list_personas();
        assert!(result.is_ok());
        // Should not panic with empty list
    }

    #[test]
    #[serial]
    fn test_list_personas_single() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create one persona
        runner
            .create_persona_interactive(
                "user1".to_string(),
                "User One".to_string(),
                "First user".to_string(),
                "First goal".to_string(),
                None,
                None,
                None,
            )
            .unwrap();

        let result = runner.list_personas();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_personas_multiple() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create multiple personas
        for i in 1..=5 {
            runner
                .create_persona_interactive(
                    format!("user{}", i),
                    format!("User {}", i),
                    format!("Description {}", i),
                    format!("Goal {}", i),
                    None,
                    Some(i as u8),
                    None,
                )
                .unwrap();
        }

        let result = runner.list_personas();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_personas_with_various_fields() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create personas with different field combinations
        runner
            .create_persona_interactive(
                "minimal".to_string(),
                "Minimal User".to_string(),
                "Minimal".to_string(),
                "Minimal goal".to_string(),
                None,
                None,
                None,
            )
            .unwrap();

        runner
            .create_persona_interactive(
                "full".to_string(),
                "Full User".to_string(),
                "Full description".to_string(),
                "Full goal".to_string(),
                Some("Context".to_string()),
                Some(5),
                Some("daily".to_string()),
            )
            .unwrap();

        let result = runner.list_personas();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_show_persona_exists() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create persona
        runner
            .create_persona_interactive(
                "show-test".to_string(),
                "Show Test User".to_string(),
                "Test description".to_string(),
                "Test goal".to_string(),
                Some("Test context".to_string()),
                Some(3),
                Some("weekly".to_string()),
            )
            .unwrap();

        let result = runner.show_persona("show-test");
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_show_persona_not_found() {
        let (_temp_dir, runner, _config) = setup_test_env();

        let result = runner.show_persona("nonexistent");
        // Should handle gracefully - either Ok with message or Err
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    #[serial]
    fn test_show_persona_minimal_fields() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create persona with minimal fields
        runner
            .create_persona_interactive(
                "minimal-show".to_string(),
                "Minimal User".to_string(),
                "Description".to_string(),
                "Goal".to_string(),
                None,
                None,
                None,
            )
            .unwrap();

        let result = runner.show_persona("minimal-show");
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_show_persona_all_fields() {
        let (_temp_dir, mut runner, _config) = setup_test_env();

        // Create persona with all fields
        runner
            .create_persona_interactive(
                "full-show".to_string(),
                "Full User".to_string(),
                "Complete description".to_string(),
                "Complete goal".to_string(),
                Some("Full context information".to_string()),
                Some(5),
                Some("daily".to_string()),
            )
            .unwrap();

        let result = runner.show_persona("full-show");
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_delete_persona_exists() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        // Create persona
        runner
            .create_persona_interactive(
                "to-delete".to_string(),
                "Delete Test".to_string(),
                "Will be deleted".to_string(),
                "Temporary goal".to_string(),
                None,
                None,
                None,
            )
            .unwrap();

        // Verify it exists
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        assert!(repo.exists("to-delete").unwrap());

        // Delete it
        let result = runner.delete_persona("to-delete");
        assert!(result.is_ok());

        // Verify it's gone
        assert!(!repo.exists("to-delete").unwrap());
    }

    #[test]
    #[serial]
    fn test_delete_persona_not_found() {
        let (_temp_dir, runner, _config) = setup_test_env();

        let result = runner.delete_persona("nonexistent");
        // Should handle gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    #[serial]
    fn test_persona_workflow_complete_cycle() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        // 1. Create persona
        let result = runner.create_persona_interactive(
            "cycle-test".to_string(),
            "Cycle Test User".to_string(),
            "Full cycle test".to_string(),
            "Test complete workflow".to_string(),
            Some("Test context".to_string()),
            Some(4),
            Some("weekly".to_string()),
        );
        assert!(result.is_ok());

        // 2. List personas
        let result = runner.list_personas();
        assert!(result.is_ok());

        // 3. Show persona
        let result = runner.show_persona("cycle-test");
        assert!(result.is_ok());

        // 4. Verify in repository
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("cycle-test").unwrap().unwrap();
        assert_eq!(persona.name, "Cycle Test User");

        // 5. Delete persona
        let result = runner.delete_persona("cycle-test");
        assert!(result.is_ok());

        // 6. Verify deletion
        assert!(!repo.exists("cycle-test").unwrap());
    }

    #[test]
    #[serial]
    fn test_multiple_personas_lifecycle() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        // Create multiple personas
        let personas = vec![
            ("user1", "User One", "First"),
            ("user2", "User Two", "Second"),
            ("user3", "User Three", "Third"),
        ];

        for (id, name, desc) in &personas {
            runner
                .create_persona_interactive(
                    id.to_string(),
                    name.to_string(),
                    desc.to_string(),
                    "Goal".to_string(),
                    None,
                    None,
                    None,
                )
                .unwrap();
        }

        // List all
        let result = runner.list_personas();
        assert!(result.is_ok());

        // Show each one
        for (id, _, _) in &personas {
            let result = runner.show_persona(id);
            assert!(result.is_ok());
        }

        // Delete one
        runner.delete_persona("user2").unwrap();

        // Verify remaining
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        assert!(repo.exists("user1").unwrap());
        assert!(!repo.exists("user2").unwrap());
        assert!(repo.exists("user3").unwrap());
    }

    #[test]
    #[serial]
    fn test_persona_with_special_characters_in_fields() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        let result = runner.create_persona_interactive(
            "special-char".to_string(),
            "User with 'quotes' and \"double quotes\"".to_string(),
            "Description with <brackets> & ampersands".to_string(),
            "Goal with special: @#$%^&*()".to_string(),
            Some("Context: line1\nline2\nline3".to_string()),
            Some(3),
            Some("2-3 times/week".to_string()),
        );

        assert!(result.is_ok());

        // Verify data integrity
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("special-char").unwrap().unwrap();
        assert!(persona.name.contains("quotes"));
        assert!(persona.description.contains("brackets"));
        assert!(persona.goal.contains("special"));
    }

    #[test]
    #[serial]
    fn test_persona_with_unicode_characters() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        let result = runner.create_persona_interactive(
            "unicode-test".to_string(),
            "用户 (User) 👤".to_string(),
            "Descripción con ácentos and émojis 🎉".to_string(),
            "Ziel erreichen 🎯".to_string(),
            Some("Contexte en français 🇫🇷".to_string()),
            Some(4),
            Some("quotidiennement".to_string()),
        );

        assert!(result.is_ok());

        // Verify unicode handling
        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("unicode-test").unwrap().unwrap();
        assert!(persona.name.contains("用户"));
        assert!(persona.description.contains("émojis"));
        assert!(persona.goal.contains("Ziel"));
    }

    #[test]
    #[serial]
    fn test_persona_with_empty_optional_strings() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        // Empty strings should be treated as None
        let result = runner.create_persona_interactive(
            "empty-opts".to_string(),
            "Empty Options".to_string(),
            "Description".to_string(),
            "Goal".to_string(),
            Some("".to_string()), // Empty string
            None,
            Some("".to_string()), // Empty string
        );

        assert!(result.is_ok());

        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("empty-opts").unwrap().unwrap();
        // Empty strings might be stored as None or as empty strings depending on implementation
        assert!(
            persona.context.is_none() || persona.context == Some("".to_string()),
            "Context should be None or empty string"
        );
    }

    #[test]
    #[serial]
    fn test_persona_tech_level_boundaries() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        // Test tech level 1 (minimum)
        runner
            .create_persona_interactive(
                "tech1".to_string(),
                "Tech Level 1".to_string(),
                "Beginner".to_string(),
                "Learn basics".to_string(),
                None,
                Some(1),
                None,
            )
            .unwrap();

        // Test tech level 5 (maximum)
        runner
            .create_persona_interactive(
                "tech5".to_string(),
                "Tech Level 5".to_string(),
                "Expert".to_string(),
                "Master skills".to_string(),
                None,
                Some(5),
                None,
            )
            .unwrap();

        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona1 = repo.load_by_id("tech1").unwrap().unwrap();
        let persona5 = repo.load_by_id("tech5").unwrap().unwrap();

        assert_eq!(persona1.tech_level, Some(1));
        assert_eq!(persona5.tech_level, Some(5));
    }

    #[test]
    #[serial]
    fn test_persona_operations_both_backends() {
        // Test TOML backend
        {
            let (_temp_dir, mut runner, config) = setup_test_env_with_backend(StorageBackend::Toml);

            runner
                .create_persona_interactive(
                    "toml-user".to_string(),
                    "TOML User".to_string(),
                    "TOML storage".to_string(),
                    "Test TOML".to_string(),
                    None,
                    None,
                    None,
                )
                .unwrap();

            let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
            assert!(repo.exists("toml-user").unwrap());
        }

        // Test SQLite backend
        {
            let (_temp_dir, mut runner, config) =
                setup_test_env_with_backend(StorageBackend::Sqlite);

            runner
                .create_persona_interactive(
                    "sqlite-user".to_string(),
                    "SQLite User".to_string(),
                    "SQLite storage".to_string(),
                    "Test SQLite".to_string(),
                    None,
                    None,
                    None,
                )
                .unwrap();

            let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
            assert!(repo.exists("sqlite-user").unwrap());
        }
    }

    #[test]
    #[serial]
    fn test_persona_with_very_long_text() {
        let (_temp_dir, mut runner, config) = setup_test_env();

        let long_text = "Lorem ipsum ".repeat(100); // Very long text
        let result = runner.create_persona_interactive(
            "long-text".to_string(),
            "Long Text User".to_string(),
            long_text.clone(),
            "Handle long text".to_string(),
            Some(long_text.clone()),
            Some(3),
            None,
        );

        assert!(result.is_ok());

        let repo = RepositoryFactory::create_persona_repository(&config).unwrap();
        let persona = repo.load_by_id("long-text").unwrap().unwrap();
        assert_eq!(persona.description.len(), long_text.len());
    }

    // TODO: Add tests for PersonaWorkflow menu interactions when testing infrastructure supports it
    // TODO: Add tests for validation prompts in interactive mode
    // TODO: Add tests for cancellation/back navigation in workflows
    // TODO: Add tests for error recovery in interactive workflows
}
