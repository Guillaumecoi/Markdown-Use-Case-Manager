// Integration tests for code preservation functionality
use crate::test_helpers::with_temp_dir;
use markdown_use_case_manager::core::templates::TemplateEngine;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_template_preservation_features() {
    with_temp_dir(|_temp_dir| {
        let engine = TemplateEngine::new().unwrap();

        // Create test data that represents a use case with scenarios
        let mut data = HashMap::new();
        data.insert("id".to_string(), json!("UC-001"));
        data.insert("title".to_string(), json!("Test Case"));
        data.insert("test_module_name".to_string(), json!("uc_001"));
        data.insert("generated_at".to_string(), json!("2025-10-01 12:00:00 UTC"));

        let scenarios = json!([
            {
                "id": "SC-001",
                "snake_case_id": "sc_001",
                "title": "Test Scenario",
                "description": "Test scenario description"
            }
        ]);
        data.insert("scenarios".to_string(), scenarios);

        // Test Rust template generation
        let rust_template = engine.render_test("rust", &data).unwrap();

        // Verify it contains our new granular markers
        assert!(rust_template.contains("START USER IMPLEMENTATION"));
        assert!(rust_template.contains("END USER IMPLEMENTATION"));
        assert!(rust_template.contains("panic!(\"Test not implemented yet\");"));

        // Verify no "expected outcome" references
        assert!(!rust_template.to_lowercase().contains("expected outcome"));

        // Test Python template generation
        data.insert("title_snake_case".to_string(), json!("Test_Case"));
        let python_template = engine.render_test("python", &data).unwrap();

        // Verify it contains our new granular markers
        assert!(python_template.contains("START USER IMPLEMENTATION"));
        assert!(python_template.contains("END USER IMPLEMENTATION"));
        assert!(python_template.contains("self.fail(\"Test not implemented yet\")"));

        // Verify no "expected outcome" references
        assert!(!python_template.to_lowercase().contains("expected outcome"));

        // Test that case insensitive language works
        assert!(engine.render_test("RUST", &data).is_ok());
        assert!(engine.render_test("Python", &data).is_ok());

        // Test that unsupported languages fail with proper error
        let unsupported_result = engine.render_test("unsupported_language", &data);
        assert!(unsupported_result.is_err());
        assert!(unsupported_result
            .unwrap_err()
            .to_string()
            .contains("Unsupported language"));
    });
}

#[test]
fn test_marker_granularity() {
    with_temp_dir(|_temp_dir| {
        let engine = TemplateEngine::new().unwrap();

        // Create test data with multiple scenarios
        let mut data = HashMap::new();
        data.insert("id".to_string(), json!("UC-001"));
        data.insert("title".to_string(), json!("Multi Scenario Test"));
        data.insert("test_module_name".to_string(), json!("uc_001"));
        data.insert("generated_at".to_string(), json!("2025-10-01 12:00:00 UTC"));

        let scenarios = json!([
            {
                "id": "SC-001",
                "snake_case_id": "sc_001",
                "title": "First Scenario",
                "description": "First test scenario"
            },
            {
                "id": "SC-002",
                "snake_case_id": "sc_002",
                "title": "Second Scenario",
                "description": "Second test scenario"
            }
        ]);
        data.insert("scenarios".to_string(), scenarios);

        // Test Rust template
        let rust_content = engine.render_test("rust", &data).unwrap();

        // Count markers - should be one pair per scenario plus one for module setup
        let start_markers = rust_content.matches("START USER IMPLEMENTATION").count();
        let end_markers = rust_content.matches("END USER IMPLEMENTATION").count();
        let test_functions = rust_content.matches("fn test_").count();

        // Debug output for troubleshooting
        println!(
            "Rust content START markers: {}, END markers: {}, test functions: {}",
            start_markers, end_markers, test_functions
        );

        // We expect: 1 module-level marker + 1 per scenario = 3 total for 2 scenarios
        assert_eq!(
            start_markers, 3,
            "Should have module marker plus one marker per scenario"
        );
        // Due to template structure, we may have extra END markers from nested text
        assert!(
            end_markers >= 3,
            "Should have at least 3 END markers to match START markers"
        );
        assert_eq!(
            test_functions, 2,
            "Should have test function for each scenario"
        );
        // Verify we have one marker per test function plus the module marker
        assert_eq!(
            start_markers,
            test_functions + 1,
            "Should have module marker plus one per test function"
        );

        // Test Python template
        data.insert("title_snake_case".to_string(), json!("Multi_Scenario_Test"));
        let python_content = engine.render_test("python", &data).unwrap();

        let py_start_markers = python_content.matches("START USER IMPLEMENTATION").count();
        let py_end_markers = python_content.matches("END USER IMPLEMENTATION").count();
        let py_test_methods = python_content.matches("def test_").count();

        println!(
            "Python content START markers: {}, END markers: {}, test methods: {}",
            py_start_markers, py_end_markers, py_test_methods
        );

        // Python template has markers for module, setUp, tearDown, and per-test
        // Expected: 1 module + 1 setUp + 1 tearDown + 1 per test method
        let expected_py_markers = 1 + 1 + 1 + py_test_methods;
        assert_eq!(
            py_start_markers, expected_py_markers,
            "Should have module, setUp, tearDown, and per-test markers"
        );
        assert!(
            py_end_markers >= expected_py_markers,
            "Should have at least matching END markers"
        );
        assert_eq!(
            py_test_methods, 2,
            "Should have test method for each scenario"
        );
    });
}
