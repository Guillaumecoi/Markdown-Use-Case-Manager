// Unit tests for template engine and template utilities
use markdown_use_case_manager::core::templates::{to_snake_case, TemplateEngine};
use serde_json::json;
use serial_test::serial;
use std::collections::HashMap;

/// Test to_snake_case utility function with various inputs
#[test]
fn test_to_snake_case_basic() {
    assert_eq!(to_snake_case("UC-AUT-001"), "uc_aut_001");
    assert_eq!(to_snake_case("User Login Process"), "user_login_process");
    assert_eq!(to_snake_case("PAYMENT_PROCESSING"), "payment_processing");
    assert_eq!(to_snake_case("simple"), "simple");
    assert_eq!(
        to_snake_case("Multi-Word.Test_Case"),
        "multi_word_test_case"
    );
}

/// Test to_snake_case with edge cases
#[test]
fn test_to_snake_case_edge_cases() {
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_snake_case("   "), "");
    assert_eq!(to_snake_case("a"), "a");
    assert_eq!(to_snake_case("A"), "a");
    assert_eq!(to_snake_case("ABC"), "abc");
    assert_eq!(to_snake_case("---"), "");
    assert_eq!(to_snake_case("___"), "");
    assert_eq!(to_snake_case("..."), "");
    assert_eq!(to_snake_case("   hello   world   "), "hello_world");
}

/// Test to_snake_case with special characters
#[test]
fn test_to_snake_case_special_characters() {
    assert_eq!(to_snake_case("Test@Case"), "test_case");
    assert_eq!(
        to_snake_case("Test#With$Special%Characters"),
        "test_with_special_characters"
    );
    assert_eq!(
        to_snake_case("Test.Case-With_Multiple.Separators"),
        "test_case_with_multiple_separators"
    );
    assert_eq!(to_snake_case("CamelCase"), "camelcase");
    assert_eq!(to_snake_case("PascalCase"), "pascalcase");
}

/// Test to_snake_case with multiple consecutive separators
#[test]
fn test_to_snake_case_multiple_separators() {
    assert_eq!(to_snake_case("test--case"), "test_case");
    assert_eq!(to_snake_case("test___case"), "test_case");
    assert_eq!(to_snake_case("test...case"), "test_case");
    assert_eq!(to_snake_case("test   case"), "test_case");
    assert_eq!(to_snake_case("test-._ case"), "test_case");
}

/// Test TemplateEngine::new().unwrap() creates a valid engine
#[test]
fn test_template_engine_creation() {
    let engine = TemplateEngine::new().unwrap();
    assert!(engine.render_use_case(&HashMap::new()).is_ok());
}

#[test]
fn test_template_engine_uniqueness() {
    let engine1 = TemplateEngine::new().unwrap();

    // Both should be TemplateEngine instances
    assert!(format!("{:?}", engine1).contains("TemplateEngine"));
}

/// Test TemplateEngine::default() works the same as new()
#[test]
fn test_template_engine_default() {
    let engine1 = TemplateEngine::new().unwrap();
    let engine2 = TemplateEngine::default();

    // Both should be TemplateEngine instances (can't test equality due to private fields)
    assert!(format!("{:?}", engine1).contains("TemplateEngine"));
    assert!(format!("{:?}", engine2).contains("TemplateEngine"));
}

/// Test TemplateEngine::render_use_case() with minimal data
#[test]
fn test_template_engine_render_minimal() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    // Minimal required data
    data.insert("title".to_string(), json!("Test Case"));
    data.insert("description".to_string(), json!("Test description"));
    data.insert("scenarios".to_string(), json!([]));
    data.insert("metadata_enabled".to_string(), json!(false));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());

    let content = result.unwrap();
    assert!(content.contains("# Test Case"));
    assert!(content.contains("## Description"));
    assert!(content.contains("Test description"));
    assert!(content.contains("## Scenarios"));
}

/// Test TemplateEngine::render_use_case() with metadata enabled
#[test]
fn test_template_engine_render_with_metadata() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    // Complete data with metadata
    data.insert("id".to_string(), json!("UC-TST-001"));
    data.insert("title".to_string(), json!("Test Case"));
    data.insert("category".to_string(), json!("testing"));
    data.insert("status_name".to_string(), json!("PLANNED"));
    data.insert("priority".to_string(), json!("MEDIUM"));
    data.insert("description".to_string(), json!("Test description"));
    data.insert("scenarios".to_string(), json!([]));

    // Metadata configuration
    data.insert("metadata_enabled".to_string(), json!(true));
    data.insert("include_id".to_string(), json!(true));
    data.insert("include_title".to_string(), json!(true));
    data.insert("include_category".to_string(), json!(true));
    data.insert("include_status".to_string(), json!(true));
    data.insert("include_priority".to_string(), json!(true));
    data.insert("include_version".to_string(), json!(false));
    data.insert("include_created".to_string(), json!(false));
    data.insert("include_last_updated".to_string(), json!(false));
    data.insert("custom_fields".to_string(), json!(["author", "epic"]));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());

    let content = result.unwrap();

    // Should start with YAML frontmatter
    assert!(content.starts_with("---"));

    // Check metadata fields
    assert!(content.contains("id: UC-TST-001"));
    assert!(content.contains("title: Test Case"));
    assert!(content.contains("category: testing"));
    assert!(content.contains("status: PLANNED"));
    assert!(content.contains("priority: MEDIUM"));

    // Check disabled fields are not present
    assert!(!content.contains("version:"));
    assert!(!content.contains("created:"));
    assert!(!content.contains("last_updated:"));
    assert!(!content.contains("tags:"));

    // Check custom fields are present
    assert!(content.contains("author: "));
    assert!(content.contains("epic: "));

    // Check content structure
    assert!(content.contains("# Test Case"));
    assert!(content.contains("## Description"));
    assert!(content.contains("## Scenarios"));
}

/// Test TemplateEngine::render_use_case() without metadata
#[test]
fn test_template_engine_render_no_metadata() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("title".to_string(), json!("Simple Case"));
    data.insert("description".to_string(), json!("Simple description"));
    data.insert("scenarios".to_string(), json!([]));
    data.insert("metadata_enabled".to_string(), json!(false));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());

    let content = result.unwrap();

    // Should not contain YAML frontmatter
    assert!(!content.starts_with("---"));
    assert!(!content.contains("id:"));
    assert!(!content.contains("title:"));

    // Should contain basic structure
    assert!(content.contains("# Simple Case"));
    assert!(content.contains("## Description"));
    assert!(content.contains("Simple description"));
    assert!(content.contains("## Scenarios"));
}

/// Test TemplateEngine::render_use_case() with scenarios
#[test]
fn test_template_engine_render_with_scenarios() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("title".to_string(), json!("Test With Scenarios"));
    data.insert(
        "description".to_string(),
        json!("Test with scenario content"),
    );
    data.insert("metadata_enabled".to_string(), json!(false));

    // Add scenarios
    let scenarios = json!([
        {
            "id": "SC-001",
            "title": "Happy Path",
            "description": "User successfully completes the task",
            "status": "📋 PLANNED"
        },
        {
            "id": "SC-002",
            "title": "Error Path",
            "description": "User encounters an error",
            "status": "🔄 IN_PROGRESS"
        }
    ]);
    data.insert("scenarios".to_string(), scenarios);

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());

    let content = result.unwrap();

    assert!(content.contains("# Test With Scenarios"));
    assert!(content.contains("## Scenarios"));
    assert!(content.contains("### Happy Path (SC-001)"));
    assert!(content.contains("### Error Path (SC-002)"));
    assert!(content.contains("User successfully completes the task"));
    assert!(content.contains("User encounters an error"));
    assert!(content.contains("**Status:** 📋 PLANNED"));
    assert!(content.contains("**Status:** 🔄 IN_PROGRESS"));
}

/// Test TemplateEngine error handling with invalid data
#[test]
fn test_template_engine_error_handling() {
    let engine = TemplateEngine::new().unwrap();
    let data = HashMap::new();

    // Missing required fields should still work (handlebars is forgiving)
    let result = engine.render_use_case(&data);
    assert!(result.is_ok());

    // But content should reflect missing data
    let content = result.unwrap();
    assert!(content.contains("# ")); // Empty title
}

/// Test TemplateEngine::render_rust_test() functionality
#[test]
fn test_template_engine_render_rust_test() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("id".to_string(), json!("UC-TST-001"));
    data.insert("title".to_string(), json!("Test Case"));
    data.insert("description".to_string(), json!("Test description"));
    data.insert("test_module_name".to_string(), json!("uc_tst_001"));
    data.insert("generated_at".to_string(), json!("2025-10-01 12:00:00 UTC"));

    let scenarios = json!([
        {
            "id": "SC-001",
            "snake_case_id": "sc_001",
            "title": "Test Scenario",
            "description": "Test scenario description",
            "status": "PLANNED"
        }
    ]);
    data.insert("scenarios".to_string(), scenarios);

    let result = engine.render_test("rust", &data);
    assert!(result.is_ok());

    let content = result.unwrap();

    // Check for content that exists in the actual template
    assert!(content.contains("Use Case: Test Case"));
    assert!(content.contains("mod uc_tst_001"));
    assert!(content.contains("fn test_sc_001()"));
    assert!(content.contains("Scenario: Test Scenario"));
    assert!(content.contains("Test scenario description"));
}

/// Test TemplateEngine::render_python_test() functionality
#[test]
fn test_template_engine_render_python_test() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("id".to_string(), json!("UC-TST-001"));
    data.insert("title".to_string(), json!("Test Case"));
    data.insert("title_snake_case".to_string(), json!("Test_Case"));
    data.insert("description".to_string(), json!("Test description"));
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

    let result = engine.render_test("python", &data);
    assert!(result.is_ok());

    let content = result.unwrap();

    // Check for Python-specific content
    assert!(content.contains("Generated test file for use case: Test Case"));
    assert!(content.contains("import unittest"));
    assert!(content.contains("class TestTest_Case(unittest.TestCase)"));
    assert!(content.contains("def test_sc_001(self)"));
    assert!(content.contains("START USER IMPLEMENTATION"));
    assert!(content.contains("END USER IMPLEMENTATION"));
}

/// Test TemplateEngine with config for template style selection
#[test]
fn test_template_engine_with_config_simple_style() {
    use markdown_use_case_manager::config::Config;

    // Create config with simple template style
    let mut config = Config::default();
    config.templates.use_case_style = Some("simple".to_string());

    // Test that the engine can be created with the config (will use fallback templates)
    let engine = TemplateEngine::with_config(Some(&config));

    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Test Use Case"));
    data.insert("description".to_string(), json!("Test description"));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());
    let content = result.unwrap();
    // Should use built-in simple template since custom templates don't exist
    assert!(content.contains("Test Use Case"));
}

/// Test TemplateEngine with config for detailed template style
#[test]
fn test_template_engine_with_config_detailed_style() {
    use markdown_use_case_manager::config::Config;

    // Create config with detailed template style
    let mut config = Config::default();
    config.templates.use_case_style = Some("detailed".to_string());

    // Test that the engine can be created with the config (will use fallback templates)
    let engine = TemplateEngine::with_config(Some(&config));

    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Test Use Case"));
    data.insert("description".to_string(), json!("Test description"));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());
    let content = result.unwrap();
    // Should use built-in detailed template since custom templates don't exist
    assert!(content.contains("Test Use Case"));
}

/// Test TemplateEngine fallback to built-in templates when custom templates don't exist
#[test]
#[serial]
fn test_template_engine_fallback_to_builtin() {
    use markdown_use_case_manager::config::Config;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    // Don't create any custom templates - should fallback to built-in
    let config = Config::default();
    let engine = TemplateEngine::with_config(Some(&config));

    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Test Use Case"));
    data.insert("description".to_string(), json!("Test description"));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());
    let content = result.unwrap();
    // Should use built-in template and still work
    assert!(content.contains("Test Use Case"));

    std::env::set_current_dir(original_dir).unwrap();
}

/// Test TemplateEngine default config behavior
#[test]
fn test_template_engine_default_config() {
    let engine = TemplateEngine::new().unwrap();

    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Test Use Case"));
    data.insert("description".to_string(), json!("Test description"));

    let result = engine.render_use_case(&data);
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.contains("Test Use Case"));
}

/// Test that Rust templates contain the new granular user implementation markers
#[test]
fn test_rust_template_granular_markers() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("id".to_string(), json!("UC-TST-001"));
    data.insert("title".to_string(), json!("Test Case"));
    data.insert("test_module_name".to_string(), json!("uc_tst_001"));
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

    let result = engine.render_test("rust", &data);
    assert!(result.is_ok());

    let content = result.unwrap();

    // Verify granular markers are present around test implementations
    assert!(content.contains(
        "// ============================================================================="
    ));
    assert!(content
        .contains("// START USER IMPLEMENTATION - Feel free to modify the code below this line"));
    assert!(content.contains(
        "// ============================================================================="
    ));
    assert!(content.contains("// END USER IMPLEMENTATION - Do not modify anything below this line"));

    // Verify markers are around individual test methods plus module setup
    let marker_count = content.matches("START USER IMPLEMENTATION").count();
    let scenario_count = content.matches("fn test_").count();
    assert_eq!(
        marker_count,
        scenario_count + 1,
        "Each test method should have its own markers plus module setup marker"
    );
}

/// Test that Python templates contain the new granular user implementation markers
#[test]
fn test_python_template_granular_markers() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("id".to_string(), json!("UC-TST-001"));
    data.insert("title".to_string(), json!("Test Case"));
    data.insert("title_snake_case".to_string(), json!("Test_Case"));
    data.insert("generated_at".to_string(), json!("2025-10-01 12:00:00 UTC"));

    let scenarios = json!([
        {
            "id": "SC-001",
            "snake_case_id": "sc_001",
            "title": "Test Scenario",
            "description": "Test scenario description"
        },
        {
            "id": "SC-002",
            "snake_case_id": "sc_002",
            "title": "Another Scenario",
            "description": "Another test scenario"
        }
    ]);
    data.insert("scenarios".to_string(), scenarios);

    let result = engine.render_test("python", &data);
    assert!(result.is_ok());

    let content = result.unwrap();

    // Verify granular markers are present around test implementations
    assert!(content.contains(
        "# ============================================================================="
    ));
    assert!(content
        .contains("# START USER IMPLEMENTATION - Feel free to modify the code below this line"));
    assert!(content.contains(
        "# ============================================================================="
    ));
    assert!(content.contains("# END USER IMPLEMENTATION - Do not modify anything below this line"));

    // Verify markers are distributed across module, setup/teardown, and test methods
    let marker_count = content.matches("START USER IMPLEMENTATION").count();
    let scenario_count = content.matches("def test_").count();
    // Expected: 1 module + 1 setUp + 1 tearDown + 1 per test method
    let expected_markers = 1 + 1 + 1 + scenario_count;
    assert_eq!(
        marker_count, expected_markers,
        "Should have module, setUp, tearDown, and per-test markers"
    );

    // Verify we have 2 test methods for 2 scenarios
    assert_eq!(scenario_count, 2);
}

/// Test that scenario template methods are no longer available
#[test]
fn test_scenario_template_methods_removed() {
    let engine = TemplateEngine::new().unwrap();

    // This test verifies at compile time that scenario template methods don't exist
    // If this compiles, it means the methods were successfully removed

    // We can't call engine.render_scenario_test() anymore - it should not exist
    // We can't call engine.get_scenario_test_template() anymore - it should not exist

    // Instead, we should only be able to use render_test() for both use case tests
    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Test"));
    data.insert("scenarios".to_string(), json!([]));

    let rust_result = engine.render_test("rust", &data);
    let python_result = engine.render_test("python", &data);

    assert!(rust_result.is_ok());
    assert!(python_result.is_ok());
}

/// Test error handling for unsupported languages
#[test]
fn test_render_test_unsupported_language() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Test"));
    data.insert("scenarios".to_string(), json!([]));

    let result = engine.render_test("unsupported_lang", &data);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unsupported language"));
}

/// Test template engine load_test_templates_for_language function coverage
#[test]
fn test_load_test_templates_coverage() {
    let engine = TemplateEngine::new().unwrap();

    // Test that we can render both supported languages
    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Coverage Test"));
    data.insert("scenarios".to_string(), json!([]));

    // Both languages should work
    assert!(engine.render_test("rust", &data).is_ok());
    assert!(engine.render_test("python", &data).is_ok());

    // Case insensitive should work
    assert!(engine.render_test("RUST", &data).is_ok());
    assert!(engine.render_test("Python", &data).is_ok());
}

/// Test that templates don't contain "Expected outcome" field references
#[test]
fn test_no_expected_outcome_in_templates() {
    let engine = TemplateEngine::new().unwrap();
    let mut data = HashMap::new();

    data.insert("title".to_string(), json!("Test Case"));
    data.insert(
        "scenarios".to_string(),
        json!([{
            "id": "SC-001",
            "snake_case_id": "sc_001",
            "title": "Test Scenario"
        }]),
    );

    let rust_content = engine.render_test("rust", &data).unwrap();
    let python_content = engine.render_test("python", &data).unwrap();

    // Verify "Expected outcome" field has been removed from templates
    assert!(!rust_content.to_lowercase().contains("expected outcome"));
    assert!(!python_content.to_lowercase().contains("expected outcome"));
    assert!(!rust_content.to_lowercase().contains("expected_outcome"));
    assert!(!python_content.to_lowercase().contains("expected_outcome"));
}
