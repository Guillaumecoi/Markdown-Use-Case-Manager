// Integration tests for template management system
use use_case_manager::config::Config;
use tempfile::TempDir;
use std::fs;
use std::path::Path;

/// Test that templates_dir() returns correct path
#[test]
fn test_templates_dir_path() {
    let templates_dir = Config::templates_dir();
    assert_eq!(templates_dir, Path::new(".config/ucm/templates"));
}

/// Test config structure includes template settings
#[test]
fn test_config_template_settings() {
    let config = Config::default();
    assert!(config.templates.use_case_style.is_some());
    assert_eq!(config.templates.use_case_style.unwrap(), "detailed");
}

/// Test that init_project creates template directory structure
#[test]
fn test_init_creates_template_directory() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize project in temp directory (no directory change needed)
    let result = Config::init_project_in_dir(temp_dir.path().to_str().unwrap());
    assert!(result.is_ok());
    
    // Verify template directory structure was created
    let templates_dir = temp_dir.path().join(".config/ucm/templates");
    assert!(templates_dir.exists());
}

/// Test template copying functionality - uses built-in templates
#[test]
fn test_template_copying_with_source() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize project in temp directory
    Config::init_project_in_dir(temp_dir.path().to_str().unwrap()).expect("Failed to initialize project");
    
    // Verify core templates were created
    let config_templates_dir = temp_dir.path().join(".config/ucm/templates");
    assert!(config_templates_dir.exists());
    assert!(config_templates_dir.join("use_case_simple.hbs").exists());
    assert!(config_templates_dir.join("use_case_detailed.hbs").exists());
    assert!(config_templates_dir.join("overview.hbs").exists());
    
    // Verify language-specific template directories were created
    assert!(config_templates_dir.join("rust").exists());
    assert!(config_templates_dir.join("python").exists());
    assert!(config_templates_dir.join("rust/test.hbs").exists());
    assert!(config_templates_dir.join("python/test.hbs").exists());
    
    // Verify content was written correctly (should contain built-in template content, not empty)
    let content = fs::read_to_string(config_templates_dir.join("use_case_simple.hbs")).unwrap();
    assert!(!content.is_empty());
    assert!(content.contains("{{title}}"));
}