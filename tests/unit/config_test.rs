// Unit tests for Config and related configuration functionality
use markdown_use_case_manager::config::{
    Config, DirectoryConfig, GenerationConfig, MetadataConfig, ProjectConfig, TemplateConfig,
};
use std::path::Path;

/// Test Config::default() creates valid default configuration
#[test]
fn test_config_default() {
    let config = Config::default();

    assert_eq!(config.project.name, "My Project");
    assert_eq!(
        config.project.description,
        "A project managed with use case manager"
    );
    assert_eq!(config.directories.use_case_dir, "docs/use-cases");
    assert_eq!(config.directories.test_dir, "tests/use-cases");
    assert!(config.directories.template_dir.is_none());
    assert_eq!(config.generation.test_language, "rust");
    assert!(!config.generation.auto_generate_tests);
    assert!(!config.generation.overwrite_test_documentation);
    assert!(config.metadata.enabled);
    assert!(config.metadata.include_id);
    assert!(!config.metadata.custom_fields.is_empty());
}

/// Test Config::config_path() returns correct path
#[test]
fn test_config_path() {
    let path = Config::config_path();
    assert_eq!(path, Path::new(".config/.mucm/mucm.toml"));
}

/// Test ProjectConfig creation and fields
#[test]
fn test_project_config() {
    let project_config = ProjectConfig {
        name: "Test Project".to_string(),
        description: "A test project for unit testing".to_string(),
    };

    assert_eq!(project_config.name, "Test Project");
    assert_eq!(
        project_config.description,
        "A test project for unit testing"
    );
}

/// Test DirectoryConfig creation and fields
#[test]
fn test_directory_config() {
    let dir_config = DirectoryConfig {
        use_case_dir: "custom/use-cases".to_string(),
        test_dir: "custom/tests".to_string(),
        template_dir: Some("custom/templates".to_string()),
    };

    assert_eq!(dir_config.use_case_dir, "custom/use-cases");
    assert_eq!(dir_config.test_dir, "custom/tests");
    assert_eq!(
        dir_config.template_dir,
        Some("custom/templates".to_string())
    );
}

/// Test TemplateConfig creation and fields
#[test]
fn test_template_config() {
    let template_config = TemplateConfig {
        use_case_template: Some("custom_use_case.hbs".to_string()),
        test_template: Some("custom_test.hbs".to_string()),
        use_case_style: Some("detailed".to_string()),
    };

    assert_eq!(
        template_config.use_case_template,
        Some("custom_use_case.hbs".to_string())
    );
    assert_eq!(
        template_config.test_template,
        Some("custom_test.hbs".to_string())
    );
    assert_eq!(template_config.use_case_style, Some("detailed".to_string()));
}

/// Test GenerationConfig creation and fields
#[test]
fn test_generation_config() {
    let gen_config = GenerationConfig {
        test_language: "javascript".to_string(),
        auto_generate_tests: true,
        overwrite_test_documentation: true,
    };

    assert_eq!(gen_config.test_language, "javascript");
    assert!(gen_config.auto_generate_tests);
    assert!(gen_config.overwrite_test_documentation);
}

/// Test MetadataConfig creation and all fields
#[test]
fn test_metadata_config() {
    let metadata_config = MetadataConfig {
        enabled: true,
        include_id: true,
        include_title: true,
        include_category: true,
        include_status: true,
        include_priority: true,
        include_created: true,
        include_last_updated: true,
        custom_fields: vec!["author".to_string(), "reviewer".to_string()],
    };

    assert!(metadata_config.enabled);
    assert!(metadata_config.include_id);
    assert!(metadata_config.include_title);
    assert!(metadata_config.include_category);
    assert!(metadata_config.include_status);
    assert!(metadata_config.include_priority);
    assert!(metadata_config.include_created);
    assert!(metadata_config.include_last_updated);
    assert_eq!(metadata_config.custom_fields.len(), 2);
    assert!(metadata_config
        .custom_fields
        .contains(&"author".to_string()));
    assert!(metadata_config
        .custom_fields
        .contains(&"reviewer".to_string()));
}

/// Test Config serialization and deserialization
#[test]
fn test_config_serialization() {
    let config = Config::default();

    // Test TOML serialization (the primary format used by the application)
    let toml_str = toml::to_string(&config).expect("Failed to serialize to TOML");
    let deserialized: Config = toml::from_str(&toml_str).expect("Failed to deserialize from TOML");

    assert_eq!(config.project.name, deserialized.project.name);
    assert_eq!(
        config.directories.use_case_dir,
        deserialized.directories.use_case_dir
    );
    assert_eq!(
        config.generation.test_language,
        deserialized.generation.test_language
    );
    assert_eq!(config.metadata.enabled, deserialized.metadata.enabled);
}

/// Test Config clone functionality
#[test]
fn test_config_clone() {
    let config = Config::default();
    let cloned = config.clone();

    assert_eq!(config.project.name, cloned.project.name);
    assert_eq!(config.project.description, cloned.project.description);
    assert_eq!(
        config.directories.use_case_dir,
        cloned.directories.use_case_dir
    );
    assert_eq!(config.directories.test_dir, cloned.directories.test_dir);
    assert_eq!(
        config.generation.test_language,
        cloned.generation.test_language
    );
    assert_eq!(config.metadata.enabled, cloned.metadata.enabled);
}

/// Test Config debug formatting
#[test]
fn test_config_debug() {
    let config = Config::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("Config"));
    assert!(debug_str.contains("project"));
    assert!(debug_str.contains("directories"));
    assert!(debug_str.contains("templates"));
    assert!(debug_str.contains("generation"));
    assert!(debug_str.contains("metadata"));
}

/// Test MetadataConfig with disabled metadata
#[test]
fn test_metadata_config_disabled() {
    let metadata_config = MetadataConfig {
        enabled: false,
        include_id: false,
        include_title: false,
        include_category: false,
        include_status: false,
        include_priority: false,
        include_created: false,
        include_last_updated: false,
        custom_fields: vec![],
    };

    assert!(!metadata_config.enabled);
    assert!(!metadata_config.include_id);
    assert!(!metadata_config.include_title);
    assert!(metadata_config.custom_fields.is_empty());
}

/// Test Config with custom values
#[test]
fn test_config_custom_values() {
    let config = Config {
        project: ProjectConfig {
            name: "Custom Project".to_string(),
            description: "A custom project configuration".to_string(),
        },
        directories: DirectoryConfig {
            use_case_dir: "src/docs".to_string(),
            test_dir: "src/tests".to_string(),
            template_dir: Some("src/templates".to_string()),
        },
        templates: TemplateConfig {
            use_case_template: Some("my_template.hbs".to_string()),
            test_template: None,
            use_case_style: Some("simple".to_string()),
        },
        generation: GenerationConfig {
            test_language: "python".to_string(),
            auto_generate_tests: true,
            overwrite_test_documentation: false,
        },
        metadata: MetadataConfig {
            enabled: true,
            include_id: true,
            include_title: true,
            include_category: false,
            include_status: true,
            include_priority: false,
            include_created: false,
            include_last_updated: true,
            custom_fields: vec![
                "epic".to_string(),
                "team".to_string(),
                "priority".to_string(),
            ],
        },
    };

    assert_eq!(config.project.name, "Custom Project");
    assert_eq!(config.directories.use_case_dir, "src/docs");
    assert_eq!(
        config.directories.template_dir,
        Some("src/templates".to_string())
    );
    assert_eq!(
        config.templates.use_case_template,
        Some("my_template.hbs".to_string())
    );
    assert!(config.templates.test_template.is_none());
    assert_eq!(config.generation.test_language, "python");
    assert!(config.generation.auto_generate_tests);
    assert!(!config.generation.overwrite_test_documentation);
    assert!(!config.metadata.include_category);
    assert_eq!(config.metadata.custom_fields.len(), 3);
}

/// Test Config field access and modification
#[test]
fn test_config_field_access() {
    let mut config = Config::default();

    // Test field access
    assert_eq!(config.project.name, "My Project");

    // Test field modification
    config.project.name = "Modified Project".to_string();
    assert_eq!(config.project.name, "Modified Project");

    config.generation.auto_generate_tests = true;
    assert!(config.generation.auto_generate_tests);

    config.metadata.custom_fields.push("new_field".to_string());
    assert!(config
        .metadata
        .custom_fields
        .contains(&"new_field".to_string()));
}
