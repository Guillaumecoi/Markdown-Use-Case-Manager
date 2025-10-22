// tests/unit/coordinator_methodology_test.rs
use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
use markdown_use_case_manager::core::templates::TemplateEngine;
use markdown_use_case_manager::config::Config;
use serial_test::serial;
use std::env;
use std::path::PathBuf;
use tempfile::TempDir;

fn with_temp_dir<F, R>(test_fn: F) -> R
where
    F: FnOnce(&PathBuf) -> R,
{
    // IMPORTANT: Get original directory BEFORE creating TempDir to avoid issues
    // if a previous test left us in a deleted directory
    let original_dir = env::current_dir().unwrap_or_else(|_| {
        // If current dir is invalid, use temp dir as fallback
        std::env::temp_dir()
    });
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Change to temp directory
    env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");
    
    // Create necessary config directory structure
    std::fs::create_dir_all(".config/.mucm").expect("Failed to create config directory");
    
    // Run the test and capture result
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        test_fn(&temp_dir.path().to_path_buf())
    }));
    
    // CRITICAL: Restore directory BEFORE TempDir drops to avoid "current dir deleted" issues
    let _ = env::set_current_dir(&original_dir);
    
    // Re-throw any panic that occurred in the test
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

#[test]
#[serial]
fn test_template_engine_methodology_methods() {
    // Test methodology methods using TemplateEngine directly
    // These don't require a full coordinator or project initialization
    let template_engine = TemplateEngine::new().expect("Failed to create template engine");

    // Test available methodologies
    let methodologies = template_engine.available_methodologies();
    assert!(methodologies.contains(&"feature".to_string()));
    assert!(methodologies.contains(&"business".to_string()));
    assert!(methodologies.contains(&"developer".to_string()));
    assert!(methodologies.contains(&"tester".to_string()));

    // Test methodology info
    let simple_info = template_engine.get_methodology_info("feature");
    assert!(simple_info.is_some());
    let (name, description) = simple_info.unwrap();
    assert_eq!(name, "Feature Development");
    assert!(description.contains("development"));

    // Test invalid methodology
    let invalid_info = template_engine.get_methodology_info("nonexistent");
    assert!(invalid_info.is_none());
}

#[test]
#[serial]
fn test_create_use_case_with_methodology() {
    with_temp_dir(|_| {
        // Create test config
        let mut config = Config::default();
        config.project.name = "Test Project".to_string();
        
        // Create necessary directories for the file service
        std::fs::create_dir_all("docs/use-cases").expect("Failed to create docs directory");

        // Create coordinator 
        let mut coordinator = UseCaseCoordinator::new(config).expect("Failed to create coordinator");

        // Create use case with specific methodology
        let result = coordinator.create_use_case_with_methodology(
            "Test Use Case".to_string(),
            "Testing".to_string(),
            Some("A test use case with business methodology".to_string()),
            "business",
        );

        assert!(result.is_ok());
        let use_case_id = result.unwrap();
        assert!(use_case_id.starts_with("UC-TES-")); // "Testing" becomes "TES" in the ID

        // Test with invalid methodology
        let invalid_result = coordinator.create_use_case_with_methodology(
            "Another Test".to_string(),
            "Testing".to_string(),
            None,
            "nonexistent",
        );

        assert!(invalid_result.is_err());
        assert!(invalid_result.unwrap_err().to_string().contains("Unknown methodology"));
    });
}

#[test]
#[serial]
fn test_regenerate_use_case_with_methodology() {
    with_temp_dir(|_| {
        // Create test config
        let mut config = Config::default();
        config.project.name = "Test Project".to_string();
        
        // Create necessary directories for the file service
        std::fs::create_dir_all("docs/use-cases").expect("Failed to create docs directory");

        // Create coordinator
        let mut coordinator = UseCaseCoordinator::new(config).expect("Failed to create coordinator");

        // First create a use case
        let use_case_id = coordinator.create_use_case(
            "Test Use Case".to_string(),
            "Testing".to_string(),
            Some("A test use case".to_string()),
        ).expect("Failed to create use case");

        // Regenerate with methodology
        let result = coordinator.regenerate_use_case_with_methodology(&use_case_id, "feature");
        assert!(result.is_ok());

        // Test with invalid methodology
        let invalid_result = coordinator.regenerate_use_case_with_methodology(&use_case_id, "nonexistent");
        assert!(invalid_result.is_err());
        assert!(invalid_result.unwrap_err().to_string().contains("Unknown methodology"));

        // Test with invalid use case ID  
        let invalid_uc_result = coordinator.regenerate_use_case_with_methodology("INVALID-ID", "feature");
        assert!(invalid_uc_result.is_err());
    });
}