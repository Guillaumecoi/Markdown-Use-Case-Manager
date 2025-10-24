// Unit tests for Config and related configuration functionality
use markdown_use_case_manager::config::{
    Config, DirectoryConfig, GenerationConfig, MetadataConfig, ProjectConfig, TemplateConfig,
};
use std::collections::HashMap;
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
    assert_eq!(config.directories.persona_dir, "docs/personas");
    assert!(config.directories.template_dir.is_none());
    assert_eq!(config.generation.test_language, "python");
    assert!(!config.generation.auto_generate_tests);
    assert!(!config.generation.overwrite_test_documentation);
    assert!(config.metadata.created);
    assert!(config.metadata.last_updated);
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
        persona_dir: "custom/personas".to_string(),
        template_dir: Some("custom/templates".to_string()),
        toml_dir: None,
    };

    assert_eq!(dir_config.use_case_dir, "custom/use-cases");
    assert_eq!(dir_config.test_dir, "custom/tests");
    assert_eq!(
        dir_config.template_dir,
        Some("custom/templates".to_string())
    );
}

/// Test TemplateConfig creation and fields - simplified structure
#[test]
fn test_template_config() {
    let template_config = TemplateConfig {
        methodologies: vec!["developer".to_string(), "feature".to_string()],
        default_methodology: "developer".to_string(),
        test_language: "python".to_string(),
    };

    assert_eq!(template_config.methodologies.len(), 2);
    assert_eq!(template_config.default_methodology, "developer");
    assert_eq!(template_config.test_language, "python");
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

/// Test MetadataConfig creation - simplified to only created/last_updated timestamps
#[test]
fn test_metadata_config() {
    let metadata_config = MetadataConfig {
        created: true,
        last_updated: true,
    };

    assert!(metadata_config.created);
    assert!(metadata_config.last_updated);
    
    // Test with timestamps disabled
    let metadata_disabled = MetadataConfig {
        created: false,
        last_updated: false,
    };
    
    assert!(!metadata_disabled.created);
    assert!(!metadata_disabled.last_updated);
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
    assert_eq!(config.metadata.created, deserialized.metadata.created);
    assert_eq!(config.metadata.last_updated, deserialized.metadata.last_updated);
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
    assert_eq!(config.metadata.created, cloned.metadata.created);
    assert_eq!(config.metadata.last_updated, cloned.metadata.last_updated);
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
            persona_dir: "src/personas".to_string(),
            template_dir: Some("src/templates".to_string()),
            toml_dir: None,
        },
        templates: TemplateConfig {
            methodologies: vec!["business".to_string()],
            default_methodology: "business".to_string(),
            test_language: "python".to_string(),
        },
        base_fields: HashMap::new(),
        generation: GenerationConfig {
            test_language: "python".to_string(),
            auto_generate_tests: true,
            overwrite_test_documentation: false,
        },
        metadata: MetadataConfig {
            created: true,
            last_updated: true,
        },
    };

    assert_eq!(config.project.name, "Custom Project");
    assert_eq!(config.directories.use_case_dir, "src/docs");
    assert_eq!(
        config.directories.template_dir,
        Some("src/templates".to_string())
    );
    assert_eq!(config.templates.test_language, "python");
    assert_eq!(config.templates.default_methodology, "business");
    assert_eq!(config.generation.test_language, "python");
    assert!(config.generation.auto_generate_tests);
    assert!(!config.generation.overwrite_test_documentation);
    assert!(config.metadata.created);
    assert!(config.metadata.last_updated);
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

    config.metadata.created = false;
    assert!(!config.metadata.created);
}
