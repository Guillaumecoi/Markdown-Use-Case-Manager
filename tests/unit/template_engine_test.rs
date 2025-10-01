// Unit tests for template engine and template utilities
use use_case_manager::core::templates::{TemplateEngine, to_snake_case};
use serde_json::json;
use std::collections::HashMap;

/// Test to_snake_case utility function with various inputs
#[test]
fn test_to_snake_case_basic() {
    assert_eq!(to_snake_case("UC-AUT-001"), "uc_aut_001");
    assert_eq!(to_snake_case("User Login Process"), "user_login_process");
    assert_eq!(to_snake_case("PAYMENT_PROCESSING"), "payment_processing");
    assert_eq!(to_snake_case("simple"), "simple");
    assert_eq!(to_snake_case("Multi-Word.Test_Case"), "multi_word_test_case");
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
    assert_eq!(to_snake_case("Test#With$Special%Characters"), "test_with_special_characters");
    assert_eq!(to_snake_case("Test.Case-With_Multiple.Separators"), "test_case_with_multiple_separators");
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

/// Test TemplateEngine::new() creates a valid engine
#[test]
fn test_template_engine_new() {
    let engine = TemplateEngine::new();
    // Just verify it creates without panicking - internal structure is private
    assert!(format!("{:?}", engine).contains("TemplateEngine"));
}

/// Test TemplateEngine::default() works the same as new()
#[test]
fn test_template_engine_default() {
    let engine1 = TemplateEngine::new();
    let engine2 = TemplateEngine::default();
    
    // Both should be TemplateEngine instances (can't test equality due to private fields)
    assert!(format!("{:?}", engine1).contains("TemplateEngine"));
    assert!(format!("{:?}", engine2).contains("TemplateEngine"));
}

/// Test TemplateEngine::render_use_case() with minimal data
#[test]
fn test_template_engine_render_minimal() {
    let engine = TemplateEngine::new();
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
    let engine = TemplateEngine::new();
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
    data.insert("include_tags".to_string(), json!(false));
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
    let engine = TemplateEngine::new();
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
    let engine = TemplateEngine::new();
    let mut data = HashMap::new();
    
    data.insert("title".to_string(), json!("Test With Scenarios"));
    data.insert("description".to_string(), json!("Test with scenario content"));
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
    let engine = TemplateEngine::new();
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
    let engine = TemplateEngine::new();
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
    
    let result = engine.render_rust_test(&data);
    assert!(result.is_ok());
    
    let content = result.unwrap();
    
    // Check for content that exists in the actual template
    assert!(content.contains("Use Case: Test Case (UC-TST-001)"));
    assert!(content.contains("mod uc_tst_001"));
    assert!(content.contains("fn test_sc_001()"));
    assert!(content.contains("Scenario: Test Scenario"));
    assert!(content.contains("Test scenario description"));
}

/// Test TemplateEngine::render_scenario_test() functionality
#[test]
fn test_template_engine_render_scenario_test() {
    let engine = TemplateEngine::new();
    let mut data = HashMap::new();
    
    data.insert("scenario_id".to_string(), json!("SC-001"));
    data.insert("scenario_title".to_string(), json!("Test Scenario"));
    data.insert("scenario_description".to_string(), json!("Scenario description"));
    data.insert("use_case_id".to_string(), json!("UC-001"));
    data.insert("use_case_title".to_string(), json!("Use Case Title"));
    data.insert("test_module_name".to_string(), json!("sc_001"));
    data.insert("generated_at".to_string(), json!("2025-10-01 12:00:00 UTC"));
    
    let result = engine.render_scenario_test(&data);
    assert!(result.is_ok());
    
    let content = result.unwrap();
    
    assert!(content.contains("Generated test file for scenario: Test Scenario"));
    assert!(content.contains("Use Case: Use Case Title (UC-001)"));
    assert!(content.contains("Scenario ID: SC-001"));
    assert!(content.contains("mod sc_001"));
    assert!(content.contains("fn test_sc_001()"));
}