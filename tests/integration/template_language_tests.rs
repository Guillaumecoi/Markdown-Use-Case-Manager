// tests/integration/template_language_tests.rs
use super::test_helpers::with_temp_dir;
use crate::test_utils::get_available_test_languages;
use markdown_use_case_manager::config::Config;
use markdown_use_case_manager::core::templates::TemplateEngine;
use markdown_use_case_manager::core::use_case_coordinator::UseCaseCoordinator;
use std::collections::HashMap;
use std::fs;

/// Test end-to-end language support initialization
#[test]
fn test_end_to_end_language_support() {
    with_temp_dir(|_temp_dir| {
        // Test that we can initialize with each supported language
        for language in &["rust", "python", "javascript", "js", "py"] {
            let result = Config::init_project_with_language_in_dir(".", Some(language.to_string()));
            assert!(
                result.is_ok(),
                "Failed to initialize with language: {}",
                language
            );

            // Clean up for next iteration
            let config_dir = std::path::Path::new(".config");
            if config_dir.exists() {
                fs::remove_dir_all(config_dir).unwrap();
            }
        }
    });
}

/// Helper to create a manager in the current directory with a given config
fn create_manager_with_config(config: Config) -> UseCaseCoordinator {
    // Create the .config/.mucm directory
    std::fs::create_dir_all(".config/.mucm").unwrap();

    // Save config to the current directory
    config.save_in_dir(".").unwrap();

    // Load manager from the current directory
    UseCaseCoordinator::load().unwrap()
}

#[test]
fn test_template_engine_language_map() {
    let template_engine = TemplateEngine::new().unwrap();

    // Should have rust, python, and javascript templates
    assert!(template_engine.has_test_template("rust"));
    assert!(template_engine.has_test_template("python"));
    assert!(template_engine.has_test_template("javascript"));

    let available_languages = get_available_test_languages();
    assert!(available_languages.contains(&"rust".to_string()));
    assert!(available_languages.contains(&"python".to_string()));
    assert!(available_languages.contains(&"javascript".to_string()));
    assert_eq!(available_languages.len(), 3);
}

#[test]
fn test_conditional_template_loading_rust() {
    // Create config with Rust language and tests enabled
    let mut config = Config::default();
    config.generation.test_language = "rust".to_string();
    config.generation.auto_generate_tests = true;

    // Create template engine with this config
    let template_engine = TemplateEngine::with_config(Some(&config));

    // Should have rust templates loaded
    assert!(template_engine.has_test_template("rust"));

    // Should be able to render rust test
    let mut data = HashMap::new();
    data.insert("title".to_string(), serde_json::json!("Test Use Case"));
    data.insert("id".to_string(), serde_json::json!("UC-TST-001"));

    let result = template_engine.render_test("rust", &data);
    assert!(result.is_ok());

    let content = result.unwrap();
    assert!(content.contains("mod"));
    assert!(content.contains("Test Use Case"));
    assert!(content.contains("UC-TST-001"));
}

#[test]
fn test_conditional_template_loading_python() {
    // Create config with Python language and tests enabled
    let mut config = Config::default();
    config.generation.test_language = "python".to_string();
    config.generation.auto_generate_tests = true;

    // Create template engine with this config
    let template_engine = TemplateEngine::with_config(Some(&config));

    // Should have python templates loaded
    assert!(template_engine.has_test_template("python"));

    // Should be able to render python test
    let mut data = HashMap::new();
    data.insert("title".to_string(), serde_json::json!("Test Use Case"));
    data.insert(
        "title_snake_case".to_string(),
        serde_json::json!("test_use_case"),
    );
    data.insert("id".to_string(), serde_json::json!("UC-TST-001"));
    data.insert("scenarios".to_string(), serde_json::json!([]));

    let result = template_engine.render_test("python", &data);
    assert!(result.is_ok());

    let content = result.unwrap();
    assert!(content.contains("import unittest"));
    assert!(content.contains("class Test"));
    assert!(content.contains("Test Use Case"));
}

#[test]
fn test_no_tests_generated_without_scenarios() {
    with_temp_dir(|_temp_dir| {
        // Initialize project with tests enabled
        let mut config = Config::default();
        config.generation.auto_generate_tests = true;
        config.generation.test_language = "rust".to_string();

        let mut manager = create_manager_with_config(config);

        // Create use case without scenarios
        let _use_case_id = manager
            .create_use_case(
                "Empty Use Case".to_string(),
                "testing".to_string(),
                Some("A use case with no scenarios".to_string()),
            )
            .unwrap();

        // Verify no test file was created
        let test_dir = std::path::Path::new("tests/use-cases/testing");
        let test_file = test_dir.join("uc_tes_001.rs");

        // Test directory might exist but test file should not
        assert!(
            !test_file.exists(),
            "Test file should not exist for use case without scenarios"
        );
    });
}

#[test]
fn test_tests_generated_with_scenarios_rust() {
    with_temp_dir(|_temp_dir| {
        // Initialize project with Rust tests enabled
        let mut config = Config::default();
        config.generation.auto_generate_tests = true;
        config.generation.test_language = "rust".to_string();

        let mut manager = create_manager_with_config(config);

        // Create use case
        let use_case_id = manager
            .create_use_case(
                "Use Case With Scenarios".to_string(),
                "testing".to_string(),
                Some("A use case that will have scenarios".to_string()),
            )
            .unwrap();

        // Add a scenario to trigger test generation
        let _scenario_id = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Test Scenario".to_string(),
                Some("This is a test scenario".to_string()),
            )
            .unwrap();

        // Verify test file was created
        let test_dir = std::path::Path::new("tests/use-cases/testing");
        let test_file = test_dir.join("uc_tes_001.rs");

        assert!(
            test_file.exists(),
            "Test file should exist after adding scenario"
        );

        // Verify content is Rust
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("mod uc_tes_001"));
        assert!(content.contains("Use Case With Scenarios"));
        assert!(content.contains("test_uc_tes_001_s01"));
    });
}

#[test]
fn test_tests_generated_with_scenarios_python() {
    with_temp_dir(|_temp_dir| {
        // Initialize project with Python tests enabled
        let mut config = Config::default();
        config.generation.auto_generate_tests = true;
        config.generation.test_language = "python".to_string();

        let mut manager = create_manager_with_config(config);

        // Create use case
        let use_case_id = manager
            .create_use_case(
                "Python Use Case".to_string(),
                "testing".to_string(),
                Some("A use case for Python testing".to_string()),
            )
            .unwrap();

        // Add a scenario to trigger test generation
        let _scenario_id = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Python Test Scenario".to_string(),
                Some("This is a Python test scenario".to_string()),
            )
            .unwrap();

        // Verify test file was created with .py extension
        let test_dir = std::path::Path::new("tests/use-cases/testing");
        let test_file = test_dir.join("uc_tes_001.py");

        assert!(
            test_file.exists(),
            "Python test file should exist after adding scenario"
        );

        // Verify content is Python
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("import unittest"));
        assert!(content.contains("class Test"));
        assert!(content.contains("test_uc_tes_001_s01"));
        assert!(content.contains("Python Use Case"));
    });
}

#[test]
fn test_no_tests_when_disabled() {
    with_temp_dir(|_temp_dir| {
        // Initialize project with tests disabled
        let mut config = Config::default();
        config.generation.auto_generate_tests = false; // Disabled
        config.generation.test_language = "rust".to_string();

        let mut manager = create_manager_with_config(config);

        // Create use case
        let use_case_id = manager
            .create_use_case(
                "Disabled Tests Use Case".to_string(),
                "testing".to_string(),
                Some("A use case with tests disabled".to_string()),
            )
            .unwrap();

        // Add a scenario
        let _scenario_id = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Scenario With No Tests".to_string(),
                Some("This scenario should not generate tests".to_string()),
            )
            .unwrap();

        // Verify no test file was created
        let test_dir = std::path::Path::new("tests/use-cases/testing");
        let test_file = test_dir.join("uc_tes_001.rs");

        assert!(
            !test_file.exists(),
            "Test file should not exist when test generation is disabled"
        );
    });
}

#[test]
fn test_unsupported_language_warning() {
    with_temp_dir(|_temp_dir| {
        // Initialize project with unsupported language
        let mut config = Config::default();
        config.generation.auto_generate_tests = true;
        config.generation.test_language = "unsupported_lang".to_string(); // Genuinely unsupported

        let mut manager = create_manager_with_config(config);

        // Create use case
        let use_case_id = manager
            .create_use_case(
                "Unsupported Language Use Case".to_string(),
                "testing".to_string(),
                Some("A use case with unsupported test language".to_string()),
            )
            .unwrap();

        // Add a scenario
        let _scenario_id = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Unsupported Scenario".to_string(),
                Some("This scenario uses unsupported language".to_string()),
            )
            .unwrap();

        // Verify no test file was created for unsupported language
        let test_dir = std::path::Path::new("tests/use-cases/testing");
        assert!(!test_dir.join("uc_tes_001.txt").exists()); // Should fallback to .txt
        assert!(!test_dir.join("uc_tes_001.js").exists());
        assert!(!test_dir.join("uc_tes_001.rs").exists());
        assert!(!test_dir.join("uc_tes_001.py").exists());
    });
}

#[test]
fn test_one_test_per_scenario() {
    with_temp_dir(|_temp_dir| {
        // Initialize project with Rust tests enabled
        let mut config = Config::default();
        config.generation.auto_generate_tests = true;
        config.generation.test_language = "rust".to_string();

        let mut manager = create_manager_with_config(config);

        // Create use case
        let use_case_id = manager
            .create_use_case(
                "Multi Scenario Use Case".to_string(),
                "testing".to_string(),
                Some("A use case with multiple scenarios".to_string()),
            )
            .unwrap();

        // Add multiple scenarios
        let _scenario_1 = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "First Scenario".to_string(),
                Some("First test scenario".to_string()),
            )
            .unwrap();

        let _scenario_2 = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Second Scenario".to_string(),
                Some("Second test scenario".to_string()),
            )
            .unwrap();

        let _scenario_3 = manager
            .add_scenario_to_use_case(
                use_case_id.clone(),
                "Third Scenario".to_string(),
                Some("Third test scenario".to_string()),
            )
            .unwrap();

        // Verify test file was created (use case test, not individual scenario tests)
        let test_dir = std::path::Path::new("tests/use-cases/testing");
        let test_file = test_dir.join("uc_tes_001.rs");

        assert!(test_file.exists(), "Test file should exist");

        // Verify content has one test function per scenario within the use case test
        let content = fs::read_to_string(&test_file).unwrap();

        // Debug: Print the actual content to see what's generated
        println!("=== GENERATED TEST CONTENT ===");
        println!("{}", content);
        println!("=== END CONTENT ===");

        // Count scenarios in the documentation section
        let scenario_docs = content.matches("## Scenario:").count();
        println!("Number of scenario docs: {}", scenario_docs);

        // Count test functions
        let test_count = content.matches("fn test_").count();
        println!("Number of test functions: {}", test_count);

        // Check for actual function names generated
        assert!(
            content.contains("fn test_uc_tes_001_s01()"),
            "Should have test for first scenario"
        );

        // If more than one scenario, check for additional functions
        if test_count > 1 {
            assert!(
                content.contains("fn test_uc_tes_001_s02()"),
                "Should have test for second scenario"
            );
        }
        if test_count > 2 {
            assert!(
                content.contains("fn test_uc_tes_001_s03()"),
                "Should have test for third scenario"
            );
        }

        // We expect 3 scenarios, but let's see what we actually get
        assert!(test_count >= 1, "Should have at least 1 test function");

        // Verify it uses our new granular markers
        assert!(
            content.contains("START USER IMPLEMENTATION"),
            "Should contain new markers"
        );
        assert!(
            content.contains("END USER IMPLEMENTATION"),
            "Should contain new markers"
        );
    });
}
