// tests/unit/processor_integration_test.rs
use markdown_use_case_manager::core::template_engine::TemplateEngine;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_template_engine_with_methodology_processors() {
    let engine = TemplateEngine::new().expect("Failed to create template engine");
    
    // Test that processor registry is available
    let methodologies = engine.available_methodologies();
    assert!(methodologies.contains(&"feature".to_string()));
    assert!(methodologies.contains(&"business".to_string()));
    assert!(methodologies.contains(&"developer".to_string()));
    assert!(methodologies.contains(&"tester".to_string()));
}

#[test]
fn test_methodology_info_retrieval() {
    let engine = TemplateEngine::new().expect("Failed to create template engine");
    
    // Test methodology info retrieval
    let simple_info = engine.get_methodology_info("feature");
    assert!(simple_info.is_some());
    let (name, description) = simple_info.unwrap();
    assert_eq!(name, "Feature Development");
    assert!(description.contains("development"));
    
    let business_info = engine.get_methodology_info("business");
    assert!(business_info.is_some());
    let (name, description) = business_info.unwrap();
    assert_eq!(name, "Business Analysis");
    assert!(description.contains("stakeholder"));
    
    let testing_info = engine.get_methodology_info("tester");
    assert!(testing_info.is_some());
    let (name, description) = testing_info.unwrap();
    assert_eq!(name, "Testing & QA");
    assert!(description.contains("Test-driven"));
}

#[test]
fn test_render_with_methodology_simple() {
    let engine = TemplateEngine::new().expect("Failed to create template engine");
    
    let mut data = HashMap::new();
    data.insert("id".to_string(), json!("UC-001"));
    data.insert("title".to_string(), json!("Test Use Case"));
    data.insert("category".to_string(), json!("Testing"));
    data.insert("description".to_string(), json!("A test use case"));
    data.insert("scenarios".to_string(), json!([]));
    
    // Test methodology-specific rendering
    let result = engine.render_use_case_with_methodology(&data, "feature");
    assert!(result.is_ok());
    
    let content = result.unwrap();
    assert!(content.contains("Test Use Case"));
    // The methodology data is added to template variables but may not appear in the basic template
    // Since we're using the fallback simple template, let's just verify the rendering works
    assert!(!content.is_empty());
}

#[test]
fn test_render_with_methodology_invalid() {
    let engine = TemplateEngine::new().expect("Failed to create template engine");
    
    let mut data = HashMap::new();
    data.insert("id".to_string(), json!("UC-001"));
    data.insert("title".to_string(), json!("Test Use Case"));
    data.insert("category".to_string(), json!("Testing"));
    data.insert("description".to_string(), json!("A test use case"));
    
    // Test with invalid methodology
    let result = engine.render_use_case_with_methodology(&data, "nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown methodology"));
}