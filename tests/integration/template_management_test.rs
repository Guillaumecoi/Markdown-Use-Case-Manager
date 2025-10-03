// Integration tests for template management system
// tests/integration/template_management_test.rs
use super::test_helpers::with_temp_dir;
use crate::test_utils::{init_project_in_dir, templates_dir};
use markdown_use_case_manager::config::Config;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Test that templates_dir() returns correct path
#[test]
fn test_templates_dir_path() {
    let templates_dir = templates_dir();
    assert_eq!(templates_dir, Path::new(".config/.mucm/templates"));
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
    let result = init_project_in_dir(temp_dir.path().to_str().unwrap());
    assert!(result.is_ok());

    // Verify template directory structure was created
    let templates_dir = temp_dir.path().join(".config/.mucm/templates");
    assert!(templates_dir.exists());
}

/// Test template copying functionality - uses built-in templates
#[test]
fn test_template_copying_with_source() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project in temp directory
    init_project_in_dir(temp_dir.path().to_str().unwrap()).expect("Failed to initialize project");

    // Verify core templates were created
    let config_templates_dir = temp_dir.path().join(".config/.mucm/templates");
    assert!(config_templates_dir.exists());
    assert!(config_templates_dir.join("use_case_simple.hbs").exists());
    assert!(config_templates_dir.join("use_case_detailed.hbs").exists());
    assert!(config_templates_dir.join("overview.hbs").exists());

    // Verify language-specific template directories were NOT created by default
    assert!(!config_templates_dir.join("rust").exists());
    assert!(!config_templates_dir.join("python").exists());

    // Verify content was written correctly (should contain built-in template content, not empty)
    let content = fs::read_to_string(config_templates_dir.join("use_case_simple.hbs")).unwrap();
    assert!(!content.is_empty());
    assert!(content.contains("{{title}}"));
}

/// Test init with rust language creates rust templates only
#[test]
fn test_init_with_rust_language() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project with rust language
    let result = Config::init_project_with_language_in_dir(
        temp_dir.path().to_str().unwrap(),
        Some("rust".to_string()),
    );
    assert!(result.is_ok());

    // Verify core templates were created
    let config_templates_dir = temp_dir.path().join(".config/.mucm/templates");
    assert!(config_templates_dir.exists());
    assert!(config_templates_dir.join("use_case_simple.hbs").exists());
    assert!(config_templates_dir.join("use_case_detailed.hbs").exists());
    assert!(config_templates_dir.join("overview.hbs").exists());

    // Verify only rust language templates were created
    assert!(config_templates_dir.join("rust").exists());
    assert!(config_templates_dir.join("rust/test.hbs").exists());
    assert!(!config_templates_dir.join("python").exists());

    // Verify config was updated with rust language
    let config = result.unwrap();
    assert_eq!(config.generation.test_language, "rust");
}

/// Test init with python language creates python templates only
#[test]
fn test_init_with_python_language() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project with python language
    let result = Config::init_project_with_language_in_dir(
        temp_dir.path().to_str().unwrap(),
        Some("python".to_string()),
    );
    assert!(result.is_ok());

    // Verify core templates were created
    let config_templates_dir = temp_dir.path().join(".config/.mucm/templates");
    assert!(config_templates_dir.exists());
    assert!(config_templates_dir.join("use_case_simple.hbs").exists());
    assert!(config_templates_dir.join("use_case_detailed.hbs").exists());
    assert!(config_templates_dir.join("overview.hbs").exists());

    // Verify only python language templates were created
    assert!(config_templates_dir.join("python").exists());
    assert!(config_templates_dir.join("python/test.hbs").exists());
    assert!(!config_templates_dir.join("rust").exists());

    // Verify config was updated with python language
    let config = result.unwrap();
    assert_eq!(config.generation.test_language, "python");
}

/// Test init with invalid language returns error
#[test]
fn test_init_with_invalid_language() {
    let temp_dir = TempDir::new().unwrap();

    // Try to initialize project with invalid language
    let result = Config::init_project_with_language_in_dir(
        temp_dir.path().to_str().unwrap(),
        Some("invalidlang".to_string()),
    );

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unsupported language 'invalidlang'"));
    assert!(error_msg.contains("Add templates to .config/.mucm/templates/lang-invalidlang/"));
}

/// Test getting available languages with built-in defaults
#[test]
fn test_get_available_languages_defaults() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    // Should return built-in languages when no templates exist
    let languages = Config::get_available_languages().unwrap();
    assert!(languages.contains(&"rust".to_string()));
    assert!(languages.contains(&"python".to_string()));

    std::env::set_current_dir(original_dir).unwrap();
}

/// Test language detection with legacy folders
#[test]
fn test_language_detection_legacy_folders() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    // Create legacy language folders (these should still be detected in config)
    let templates_dir = temp_dir.path().join(".config/.mucm/templates");
    fs::create_dir_all(&templates_dir).unwrap();
    fs::create_dir_all(templates_dir.join("rust")).unwrap();
    fs::create_dir_all(templates_dir.join("python")).unwrap();

    let languages = Config::get_available_languages().unwrap();
    // Should detect the legacy folders
    assert!(languages.contains(&"rust".to_string()));
    assert!(languages.contains(&"python".to_string()));

    std::env::set_current_dir(original_dir).unwrap();
}

/// Test language detection with new prefixed folders
#[test]
fn test_language_detection_prefixed_folders() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    // Create new prefixed language folders
    let templates_dir = temp_dir.path().join(".config/.mucm/templates");
    fs::create_dir_all(&templates_dir).unwrap();
    fs::create_dir_all(templates_dir.join("lang-javascript")).unwrap();
    fs::create_dir_all(templates_dir.join("lang-go")).unwrap();
    fs::create_dir_all(templates_dir.join("lang-java")).unwrap();

    let languages = Config::get_available_languages().unwrap();
    assert!(languages.contains(&"javascript".to_string()));
    assert!(languages.contains(&"go".to_string()));
    assert!(languages.contains(&"java".to_string()));

    std::env::set_current_dir(original_dir).unwrap();
}

/// Test mixed legacy and prefixed language folders
#[test]
fn test_language_detection_mixed_folders() {
    with_temp_dir(|temp_dir| {
        // Create both legacy and prefixed folders
        let templates_dir = temp_dir.path().join(".config/.mucm/templates");
        fs::create_dir_all(&templates_dir).unwrap();
        fs::create_dir_all(templates_dir.join("rust")).unwrap(); // Legacy
        fs::create_dir_all(templates_dir.join("lang-go")).unwrap(); // New

        let languages = Config::get_available_languages().unwrap();
        // Should detect local config additions
        assert!(languages.contains(&"rust".to_string())); // Legacy folder
        assert!(languages.contains(&"go".to_string())); // From local config
                                                        // Should not contain duplicates
        assert_eq!(languages.iter().filter(|&l| l == "rust").count(), 1);
    });
}
