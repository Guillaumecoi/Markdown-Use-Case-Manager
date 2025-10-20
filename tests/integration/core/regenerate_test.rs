// Integration tests for regenerate command
#[cfg(test)]
mod regenerate_tests {
    use serial_test::serial;
    use std::env;
    use std::fs;
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
    fn test_regenerate_single_use_case() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::config::Config;
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize project and create a use case
            Config::init_project_with_language(Some("rust".to_string())).unwrap();
            let mut coordinator = UseCaseCoordinator::load().unwrap();
            
            let use_case_id = coordinator.create_use_case(
                "Test Use Case".to_string(),
                "Testing".to_string(),
                Some("Initial description".to_string()),
            ).unwrap();
            
                        // Manually edit the TOML file to add a new scenario
            let toml_path = format!("use-cases-data/testing/{}.toml", use_case_id);
            let toml_content = fs::read_to_string(&toml_path).unwrap();
            let updated_toml = toml_content.replace(
                "Initial description",
                "Updated description from TOML edit"
            );
            fs::write(&toml_path, updated_toml).unwrap();
            
            // Reload coordinator to pick up TOML changes
            let coordinator = UseCaseCoordinator::load().unwrap();
            
            // Regenerate the use case
            coordinator.regenerate_markdown(&use_case_id).unwrap();
            
            // Verify markdown was updated
            let md_path = format!("docs/use-cases/testing/{}.md", use_case_id);
            let md_content = fs::read_to_string(&md_path).unwrap();
            assert!(md_content.contains("Updated description from TOML edit"));
        });
    }

    #[test]
    #[serial]
    fn test_regenerate_all_use_cases() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::config::Config;
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize project and create multiple use cases
            Config::init_project_with_language(Some("rust".to_string())).unwrap();
            let mut coordinator = UseCaseCoordinator::load().unwrap();
            
            let uc1 = coordinator.create_use_case(
                "Use Case 1".to_string(),
                "Testing".to_string(),
                Some("Description 1".to_string()),
            ).unwrap();
            
            let uc2 = coordinator.create_use_case(
                "Use Case 2".to_string(),
                "Testing".to_string(),
                Some("Description 2".to_string()),
            ).unwrap();
            
            // Edit both TOML files
            for use_case_id in &[&uc1, &uc2] {
                let toml_path = format!("use-cases-data/testing/{}.toml", use_case_id);
                let toml_content = fs::read_to_string(&toml_path).unwrap();
                let updated_toml = toml_content.replace(
                    "priority = \"Medium\"",
                    "priority = \"High\""
                );
                fs::write(&toml_path, updated_toml).unwrap();
            }
            
            // Reload and regenerate all
            let coordinator = UseCaseCoordinator::load().unwrap();
            coordinator.regenerate_all_markdown().unwrap();
            
            // Verify both markdown files were updated
            for use_case_id in &[&uc1, &uc2] {
                let md_path = format!("docs/use-cases/testing/{}.md", use_case_id);
                let md_content = fs::read_to_string(&md_path).unwrap();
                assert!(md_content.contains("HIGH") || md_content.contains("High"));
            }
        });
    }

    #[test]
    #[serial]
    fn test_regenerate_preserves_toml_changes() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::config::Config;
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize and create use case
            Config::init_project_with_language(Some("rust".to_string())).unwrap();
            let mut coordinator = UseCaseCoordinator::load().unwrap();
            
            let use_case_id = coordinator.create_use_case(
                "Complex Use Case".to_string(),
                "Testing".to_string(),
                None,
            ).unwrap();
            
            // Edit TOML with rich metadata
            let toml_path = format!("use-cases-data/testing/{}.toml", use_case_id);
            let toml_content = format!(
                r#"id = "{}"
title = "Complex Use Case"
category = "Testing"
description = "Detailed description added manually"
priority = "High"
scenarios = []
prerequisites = ["Setup database", "Install dependencies"]
personas = ["Developer", "Tester"]
acceptance_criteria = [
  "All tests pass",
  "Code coverage > 80%",
  "Performance benchmarks met"
]
assumptions = ["Test environment available"]
constraints = ["Must complete within sprint"]

[metadata]
created_at = "2025-10-20T10:00:00.000000000Z"
updated_at = "2025-10-20T11:00:00.000000000Z"
"#,
                use_case_id
            );
            fs::write(&toml_path, toml_content).unwrap();
            
            // Reload and regenerate
            let coordinator = UseCaseCoordinator::load().unwrap();
            coordinator.regenerate_markdown(&use_case_id).unwrap();
            
            // Verify markdown reflects TOML changes
            let md_path = format!("docs/use-cases/testing/{}.md", use_case_id);
            let md_content = fs::read_to_string(&md_path).unwrap();
            
            assert!(md_content.contains("Detailed description added manually"));
            assert!(md_content.contains("HIGH") || md_content.contains("High"));
            
            // Verify TOML data is preserved (though formatting may change during save)
            let toml_after = fs::read_to_string(&toml_path).unwrap();
            // Check that the data is still there (TOML may be reformatted)
            assert!(toml_after.contains("Setup database"));
            assert!(toml_after.contains("Install dependencies"));
            assert!(toml_after.contains("Developer"));
            assert!(toml_after.contains("Tester"));
            assert!(toml_after.contains("All tests pass"));
        });
    }

    #[test]
    #[serial]
    fn test_regenerate_nonexistent_use_case() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::config::Config;
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize empty project
            Config::init_project_with_language(Some("rust".to_string())).unwrap();
            let coordinator = UseCaseCoordinator::load().unwrap();
            
            // Try to regenerate non-existent use case
            let result = coordinator.regenerate_markdown("UC-NONEXISTENT-001");
            
            assert!(result.is_err());
            if let Err(error) = result {
                let error_msg = error.to_string();
                assert!(error_msg.contains("not found") || error_msg.contains("UC-NONEXISTENT-001"));
            }
        });
    }

    #[test]
    #[serial]
    fn test_regenerate_updates_overview() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::config::Config;
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize and create use case
            Config::init_project_with_language(Some("rust".to_string())).unwrap();
            let mut coordinator = UseCaseCoordinator::load().unwrap();
            
            let use_case_id = coordinator.create_use_case(
                "Feature X".to_string(),
                "Features".to_string(),
                Some("Original description".to_string()),
            ).unwrap();
            
            // Check initial overview
            let overview_path = "docs/use-cases/README.md";
            let overview_before = fs::read_to_string(overview_path).unwrap();
            assert!(overview_before.contains("Feature X"));
            
            // Edit TOML to change title
            let toml_path = format!("use-cases-data/features/{}.toml", use_case_id);
            let toml_content = fs::read_to_string(&toml_path).unwrap();
            let updated_toml = toml_content.replace(
                "title = \"Feature X\"",
                "title = \"Feature X Enhanced\""
            );
            fs::write(&toml_path, updated_toml).unwrap();
            
            // Regenerate all (which includes overview)
            let coordinator = UseCaseCoordinator::load().unwrap();
            coordinator.regenerate_all_markdown().unwrap();
            
            // Verify overview was updated
            let overview_after = fs::read_to_string(overview_path).unwrap();
            assert!(overview_after.contains("Feature X Enhanced"));
        });
    }

    #[test]
    #[serial]
    fn test_toml_is_source_md_is_output() {
        with_temp_dir(|_| {
            use markdown_use_case_manager::config::Config;
            use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
            
            // Initialize and create use case
            Config::init_project_with_language(Some("rust".to_string())).unwrap();
            let mut coordinator = UseCaseCoordinator::load().unwrap();
            
            let use_case_id = coordinator.create_use_case(
                "Source Test".to_string(),
                "Testing".to_string(),
                Some("Initial".to_string()),
            ).unwrap();
            
            let toml_path = format!("use-cases-data/testing/{}.toml", use_case_id);
            let md_path = format!("docs/use-cases/testing/{}.md", use_case_id);
            
            // Get timestamps
            let toml_meta = fs::metadata(&toml_path).unwrap();
            let md_meta = fs::metadata(&md_path).unwrap();
            
            // Both should exist
            assert!(toml_meta.is_file());
            assert!(md_meta.is_file());
            
            // Edit TOML
            std::thread::sleep(std::time::Duration::from_millis(100)); // Ensure different timestamp
            let toml_content = fs::read_to_string(&toml_path).unwrap();
            let updated_toml = toml_content.replace("Initial", "Modified");
            fs::write(&toml_path, updated_toml).unwrap();
            
            // Regenerate
            let coordinator = UseCaseCoordinator::load().unwrap();
            coordinator.regenerate_markdown(&use_case_id).unwrap();
            
            // MD should be updated
            let md_content = fs::read_to_string(&md_path).unwrap();
            assert!(md_content.contains("Modified"));
            
            // TOML metadata shows it's newer or same age as MD (source drives output)
            let toml_modified = fs::metadata(&toml_path).unwrap().modified().unwrap();
            let md_modified = fs::metadata(&md_path).unwrap().modified().unwrap();
            
            // MD should be regenerated (newer than or equal to TOML after regen)
            assert!(md_modified >= toml_modified);
        });
    }
}
