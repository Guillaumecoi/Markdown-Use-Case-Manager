// Tests for the modular language support system

use crate::test_utils::*;
use markdown_use_case_manager::core::languages::LanguageRegistry;

#[test]
fn test_language_registry_creation() {
    let registry = LanguageRegistry::new();
    let available_languages = registry.available_languages();

    // Check that built-in languages are registered
    assert!(available_languages.contains(&"rust".to_string()));
    assert!(available_languages.contains(&"python".to_string()));
    assert!(available_languages.contains(&"javascript".to_string()));
    assert_eq!(available_languages.len(), 3);
}

#[test]
fn test_language_lookup() {
    let registry = LanguageRegistry::new();

    // Test primary names
    assert!(registry.get("rust").is_some());
    assert!(registry.get("python").is_some());
    assert!(registry.get("javascript").is_some());

    // Test aliases
    assert!(registry.get("py").is_some());
    assert!(registry.get("js").is_some());

    // Test unsupported language
    assert!(registry.get("go").is_none());

    // Test is_supported
    assert!(registry.is_supported("rust"));
    assert!(registry.is_supported("py"));
    assert!(!registry.is_supported("go"));
}

#[test]
fn test_template_language_specific_methods() {
    // Test language-specific template methods
    let rust_template = get_rust_test_template();
    assert!(rust_template.contains("fn test_"));

    let python_template = get_python_test_template();
    assert!(python_template.contains("def test_"));

    let js_template = get_javascript_test_template();
    assert!(js_template.contains("describe("));
}

#[test]
fn test_python_language_implementation() {
    let registry = LanguageRegistry::new();
    let python_lang = registry.get("python").unwrap();

    assert_eq!(python_lang.name(), "python");
    assert_eq!(python_lang.file_extension(), "py");
    assert!(python_lang.uses_legacy_directory());
    assert_eq!(python_lang.aliases(), &["py"]);

    // Test that template is loaded
    let template = python_lang.test_template();
    assert!(template.contains("{{title}}"));
    assert!(template.contains("import unittest"));
    assert!(template.contains("def test_{{snake_case_id}}"));
}

#[test]
fn test_javascript_language_implementation() {
    let registry = LanguageRegistry::new();
    let js_lang = registry.get("javascript").unwrap();

    assert_eq!(js_lang.name(), "javascript");
    assert_eq!(js_lang.file_extension(), "js");
    assert!(!js_lang.uses_legacy_directory()); // JavaScript uses new format
    assert_eq!(js_lang.aliases(), &["js"]);

    // Test that template is loaded
    let template = js_lang.test_template();
    assert!(template.contains("{{title}}"));
    assert!(template.contains("describe("));
    assert!(template.contains("test("));
}

#[test]
fn test_language_aliases() {
    let registry = LanguageRegistry::new();

    // Python aliases
    let python_by_name = registry.get("python").unwrap();
    let python_by_alias = registry.get("py").unwrap();
    assert_eq!(python_by_name.name(), python_by_alias.name());

    // JavaScript aliases
    let js_by_name = registry.get("javascript").unwrap();
    let js_by_alias = registry.get("js").unwrap();
    assert_eq!(js_by_name.name(), js_by_alias.name());
}

#[test]
fn test_all_names_method() {
    let registry = LanguageRegistry::new();

    let python_lang = registry.get("python").unwrap();
    let all_names = python_lang.all_names();
    assert!(all_names.contains(&"python"));
    assert!(all_names.contains(&"py"));
    assert_eq!(all_names.len(), 2);

    let rust_lang = registry.get("rust").unwrap();
    let rust_names = rust_lang.all_names();
    assert!(rust_names.contains(&"rust"));
    assert_eq!(rust_names.len(), 1); // No aliases for rust
}

#[test]
fn test_template_content_quality() {
    let registry = LanguageRegistry::new();

    // Test that all templates contain necessary placeholders
    for lang_name in &["rust", "python", "javascript"] {
        let lang = registry.get(lang_name).unwrap();
        let template = lang.test_template();

        // Common placeholders
        assert!(
            template.contains("{{title}}"),
            "Template for {} missing {{title}}",
            lang_name
        );
        assert!(
            template.contains("{{id}}"),
            "Template for {} missing {{id}}",
            lang_name
        );
        assert!(
            template.contains("{{description}}"),
            "Template for {} missing {{description}}",
            lang_name
        );
        assert!(
            template.contains("{{#each scenarios}}"),
            "Template for {} missing scenario iteration",
            lang_name
        );

        // Check for user implementation markers
        assert!(
            template.contains("START USER IMPLEMENTATION"),
            "Template for {} missing start marker",
            lang_name
        );
        assert!(
            template.contains("END USER IMPLEMENTATION"),
            "Template for {} missing end marker",
            lang_name
        );
    }
}

#[test]
fn test_template_engine_integration() {
    // Test that TemplateEngine can get templates through the language registry
    let rust_template = get_test_template_for_language("rust");
    assert!(rust_template.is_some());

    let python_template = get_test_template_for_language("python");
    assert!(python_template.is_some());

    let js_template = get_test_template_for_language("javascript");
    assert!(js_template.is_some());

    let unknown_template = get_test_template_for_language("unknown");
    assert!(unknown_template.is_none());
}

#[test]
fn test_legacy_compatibility() {
    // Test that legacy methods still work
    let rust_template = get_rust_test_template();
    assert!(rust_template.contains("{{title}}"));

    let python_template = get_python_test_template();
    assert!(python_template.contains("{{title}}"));

    let js_template = get_javascript_test_template();
    assert!(js_template.contains("{{title}}"));
}
