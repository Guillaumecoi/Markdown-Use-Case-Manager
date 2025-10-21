// Error handling integration tests
#[cfg(test)]
mod error_handling_tests {
    use serial_test::serial;
    use std::env;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn with_temp_dir<F, R>(test_fn: F) -> R
    where
        F: FnOnce(&PathBuf) -> R,
    {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let original_dir = env::current_dir().expect("Failed to get current directory");
        
        env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            test_fn(&temp_dir.path().to_path_buf())
        }));
        
        let _ = env::set_current_dir(original_dir);
        
        match result {
            Ok(value) => value,
            Err(panic) => std::panic::resume_unwind(panic),
        }
    }

    #[test]
    #[serial]
    fn test_command_without_init_fails_gracefully() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Attempting to load coordinator without init should fail gracefully
            let result = UseCaseCoordinator::load();
            
            assert!(result.is_err());
            if let Err(error) = result {
                assert!(error.to_string().contains("use case manager project"));
            }
        });
    }

    #[test]
    #[serial]
    fn test_invalid_status_update_fails_gracefully() {
        with_temp_dir(|_| {
            
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize project
            crate::test_utils::init_project_with_language(Some("rust".to_string()))
                .expect("Failed to initialize project");
            
            let mut coordinator = UseCaseCoordinator::load()
                .expect("Failed to load coordinator");
            
            // Create a use case
            let use_case_id = coordinator
                .create_use_case(
                    "Test Case".to_string(),
                    "Testing".to_string(),
                    Some("Test description".to_string()),
                )
                .expect("Failed to create use case");
            
            // Add a scenario
            let scenario_id = coordinator
                .add_scenario_to_use_case(
                    use_case_id,
                    "Test Scenario".to_string(),
                    Some("Test scenario description".to_string()),
                )
                .expect("Failed to add scenario");
            
            // Try to update with invalid status
            let result = coordinator.update_scenario_status(
                scenario_id,
                "invalid_status".to_string(),
            );
            
            assert!(result.is_err());
            let error = result.unwrap_err();
            assert!(error.to_string().contains("Invalid status"));
        });
    }

    #[test]
    #[serial]
    fn test_nonexistent_use_case_scenario_fails() {
        with_temp_dir(|_| {
            
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            crate::test_utils::init_project_with_language(Some("rust".to_string()))
                .expect("Failed to initialize project");
            
            let mut coordinator = UseCaseCoordinator::load()
                .expect("Failed to load coordinator");
            
            // Try to add scenario to non-existent use case
            let result = coordinator.add_scenario_to_use_case(
                "UC-NONEXISTENT-999".to_string(),
                "Test Scenario".to_string(),
                None,
            );
            
            assert!(result.is_err());
            let error = result.unwrap_err();
            assert!(error.to_string().contains("not found"));
        });
    }

    #[test]
    #[serial]
    fn test_duplicate_init_is_idempotent() {
        with_temp_dir(|_| {
            
            
            // First init should succeed
            let result1 = crate::test_utils::init_project_with_language(Some("rust".to_string()));
            assert!(result1.is_ok());
            
            // Second init should also succeed (idempotent operation)
            // It will just update templates and config
            let result2 = crate::test_utils::init_project_with_language(Some("rust".to_string()));
            assert!(result2.is_ok());
        });
    }

    #[test]
    #[serial]
    fn test_unsupported_language_fails_gracefully() {
        with_temp_dir(|_| {
            
            
            let result = crate::test_utils::init_project_with_language(
                Some("totally_fake_language_xyz".to_string())
            );
            
            assert!(result.is_err());
            let error = result.unwrap_err();
            assert!(error.to_string().contains("Unsupported language"));
        });
    }
}
