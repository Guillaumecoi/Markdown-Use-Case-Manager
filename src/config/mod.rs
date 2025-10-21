// src/config/mod.rs
use crate::core::languages::LanguageRegistry;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub directories: DirectoryConfig,
    pub templates: TemplateConfig,
    pub generation: GenerationConfig,
    pub metadata: MetadataConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct DirectoryConfig {
    pub use_case_dir: String,
    pub test_dir: String,
    pub persona_dir: String,
    pub template_dir: Option<String>,
    /// Directory for TOML source files (defaults to same as use_case_dir if not specified)
    pub toml_dir: Option<String>,
}

impl DirectoryConfig {
    /// Get the effective TOML directory (falls back to use_case_dir if not specified)
    pub fn get_toml_dir(&self) -> &str {
        self.toml_dir.as_deref().unwrap_or(&self.use_case_dir)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub use_case_template: Option<String>,
    pub test_template: Option<String>,
    /// List of methodologies to import and make available
    pub methodologies: Vec<String>,
    /// Default methodology to use when none specified
    pub default_methodology: Option<String>,
}

/// Per-methodology template configuration
/// This is loaded from .config/.mucm/methodologies/{name}.toml
/// Note: Metadata is configured in the main config, not per-methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyConfig {
    pub template: MethodologyTemplateInfo,
    pub generation: GenerationConfig,
    #[serde(default)]
    pub custom_fields: std::collections::HashMap<String, CustomFieldConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyTemplateInfo {
    pub name: String,
    pub description: String,
    /// Preferred/recommended style for this methodology: "simple", "normal", or "detailed"
    pub preferred_style: String,
}

/// Configuration for custom fields specific to a methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldConfig {
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String, // "string", "array", "number", "boolean"
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub test_language: String,
    pub auto_generate_tests: bool,
    pub overwrite_test_documentation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    /// Enable or disable metadata generation entirely
    pub enabled: bool,

    // Auto-populated fields (true/false to include or not)
    // These fields are automatically filled by the system when creating use cases:
    /// Auto-generated unique identifier
    pub include_id: bool,
    /// Use case title from command line argument
    pub include_title: bool,
    /// Category derived from directory structure
    pub include_category: bool,
    /// Current status (automatically set to "draft")
    pub include_status: bool,
    /// Priority level (automatically set to "medium")
    pub include_priority: bool,
    /// Creation timestamp (automatically set to current time)
    pub include_created: bool,
    /// Last updated timestamp (automatically set to current time)  
    pub include_last_updated: bool,

    // Extended metadata fields (true/false to enable/disable each field)
    /// Prerequisites and dependencies for the use case
    pub include_prerequisites: bool,
    /// Target users and stakeholders
    pub include_personas: bool,
    /// Author of the use case
    pub include_author: bool,
    /// Reviewer of the use case
    pub include_reviewer: bool,
    /// Business value and justification
    pub include_business_value: bool,
    /// Implementation complexity assessment
    pub include_complexity: bool,
    /// Associated epic or project
    pub include_epic: bool,
    /// Acceptance criteria for completion
    pub include_acceptance_criteria: bool,
    /// Assumptions made in the use case
    pub include_assumptions: bool,
    /// Constraints and limitations
    pub include_constraints: bool,
}

impl Config {
    /// Create a Config with methodology-specific recommended settings
    /// This sets the default_methodology in the main config
    pub fn new_with_methodology(methodology: &str) -> Self {
        let mut config = Self::default();
        
        // Just set the default methodology - actual settings come from per-template configs
        config.templates.default_methodology = Some(methodology.to_string());
        
        config
    }

    /// Get methodology-specific recommendations as a human-readable string
    pub fn methodology_recommendations(methodology: &str) -> String {
        match methodology {
            "business" => {
                "Business Methodology Recommendations:
- Focus on business value and stakeholder needs
- Business-oriented language and structure
- Emphasis on ROI and business outcomes
- Best for: Business analysts, product managers, stakeholder documentation".to_string()
            },
            "developer" => {
                "Developer Methodology Recommendations:
- Technical implementation focus
- System behavior and API documentation
- Code-centric perspective
- Best for: Development teams, technical documentation, API design".to_string()
            },
            "feature" => {
                "Feature Methodology Recommendations:
- Feature-oriented documentation
- User story and epic integration
- Agile-friendly structure
- Best for: Product development, agile teams, feature tracking".to_string()
            },
            "testing" => {
                "Testing Methodology Recommendations:
- Test-focused documentation
- Test scenarios and coverage tracking
- Quality assurance emphasis
- Best for: QA teams, test automation, quality metrics".to_string()
            },
            _ => "Unknown methodology. Using developer methodology defaults.".to_string()
        }
    }
    const CONFIG_DIR: &'static str = ".config/.mucm";
    const CONFIG_FILE: &'static str = "mucm.toml";
    const TEMPLATES_DIR: &'static str = "templates";

    pub fn config_path() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::CONFIG_FILE)
    }

    /// Get list of available programming languages from source templates and local config
    pub fn get_available_languages() -> Result<Vec<String>> {
        let mut languages = Vec::new();

        // Start with built-in language registry
        let language_registry = LanguageRegistry::new();
        languages.extend(language_registry.available_languages());

        // Look for user-defined languages in current directory
        let config_dir = Path::new(Self::CONFIG_DIR);
        let templates_dir = config_dir.join(Self::TEMPLATES_DIR);

        if templates_dir.exists() {
            for entry in fs::read_dir(&templates_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();

                    // Check for "lang-{language}" pattern (preferred)
                    if let Some(lang) = dir_name.strip_prefix("lang-") {
                        if !languages.contains(&lang.to_string()) {
                            languages.push(lang.to_string());
                        }
                    }
                }
            }
        }

        languages.sort();
        Ok(languages)
    }

    /// Save config file only (without copying templates or creating directories)
    /// Used in the first step of two-step initialization
    pub fn save_config_only(config: &Config) -> Result<()> {
        let base_path = Path::new(".");
        let config_dir = base_path.join(Self::CONFIG_DIR);
        
        // Create .config/.mucm directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create .config/.mucm directory")?;
        }
        
        config.save_in_dir(".")?;
        Ok(())
    }

    /// Check if templates have already been copied to .config/.mucm/templates/
    pub fn check_templates_exist() -> bool {
        let base_path = Path::new(".");
        let templates_dir = base_path.join(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR);
        templates_dir.exists() && templates_dir.is_dir()
    }

    /// Copy templates to .config/.mucm/templates/ with language (wrapper for _in_dir version)
    pub fn copy_templates_to_config_with_language(language: Option<String>) -> Result<()> {
        Self::copy_templates_to_config_with_language_in_dir(".", language)
    }

    fn copy_templates_to_config_with_language_in_dir(
        base_dir: &str,
        _language: Option<String>,  // Not currently used - we copy all languages now
    ) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_templates_dir = base_path.join(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR);
        let config_methodologies_dir = base_path.join(Self::CONFIG_DIR).join("methodologies");

        // Create directories
        fs::create_dir_all(&config_templates_dir)
            .context("Failed to create config templates directory")?;
        fs::create_dir_all(&config_methodologies_dir)
            .context("Failed to create config methodologies directory")?;

        // Load the config from base_dir to see which methodologies to import
        let config_path = base_path.join(Self::CONFIG_DIR).join("mucm.toml");
        if !config_path.exists() {
            anyhow::bail!("Config file not found at {:?} - run 'mucm init' first", config_path);
        }
        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        // Look for templates directory - first check if we're in a dev environment
        let mut source_templates_dir = None;
        
        // Try current directory first
        let local_templates = Path::new("templates");
        if local_templates.exists() {
            source_templates_dir = Some(local_templates.to_path_buf());
        } else {
            // Try CARGO_MANIFEST_DIR (set during cargo test and build)
            if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                let cargo_templates = Path::new(&manifest_dir).join("templates");
                if cargo_templates.exists() {
                    source_templates_dir = Some(cargo_templates);
                }
            }
            
            // If still not found, try to find templates relative to the executable
            if source_templates_dir.is_none() {
                if let Ok(exe_path) = std::env::current_exe() {
                    if let Some(exe_dir) = exe_path.parent() {
                        // Check ../../templates (when running from target/release/)
                        let dev_templates = exe_dir.parent().and_then(|p| p.parent()).map(|p| p.join("templates"));
                        if let Some(dev_templates) = dev_templates {
                            if dev_templates.exists() {
                                source_templates_dir = Some(dev_templates);
                            }
                        }
                    }
                }
            }
        }
        
        let Some(source_templates_dir) = source_templates_dir else {
            anyhow::bail!(
                "Source templates directory not found. \
                 Looked for 'templates/' directory. \
                 This directory should contain methodologies/ and languages/ subdirectories."
            );
        };

        // Copy only the selected methodologies
        let source_methodologies = source_templates_dir.join("methodologies");
        if source_methodologies.exists() {
            for methodology in &config.templates.methodologies {
                let source_method_dir = source_methodologies.join(methodology);
                if !source_method_dir.exists() {
                    anyhow::bail!(
                        "Methodology '{}' not found in templates/methodologies/. \
                         Available methodologies should be in templates/methodologies/{{name}}/ directories.",
                        methodology
                    );
                }

                // Copy methodology templates to templates/{methodology}/
                let target_method_templates = config_templates_dir.join(methodology);
                Self::copy_dir_recursive(&source_method_dir, &target_method_templates)?;

                // Copy methodology config.toml to methodologies/{methodology}.toml
                let source_config = source_method_dir.join("config.toml");
                if source_config.exists() {
                    let target_config = config_methodologies_dir.join(format!("{}.toml", methodology));
                    fs::copy(&source_config, &target_config)?;
                    println!("✓ Copied methodology: {}", methodology);
                } else {
                    anyhow::bail!(
                        "Methodology '{}' is missing config.toml file at {:?}",
                        methodology,
                        source_config
                    );
                }
            }
        } else {
            anyhow::bail!(
                "Source methodologies directory not found at {:?}",
                source_methodologies
            );
        }

        // Copy language templates based on configured language
        let source_languages = source_templates_dir.join("languages");
        if source_languages.exists() {
            let source_lang_dir = source_languages.join(&config.generation.test_language);
            if source_lang_dir.exists() {
                let target_languages = config_templates_dir.join("languages");
                let target_lang_dir = target_languages.join(&config.generation.test_language);
                Self::copy_dir_recursive(&source_lang_dir, &target_lang_dir)?;
                println!("✓ Copied language templates: {}", config.generation.test_language);
            } else {
                println!("⚠ Language '{}' not found in templates/languages/, skipping", config.generation.test_language);
            }
        }

        Ok(())
    }

    /// Recursively copy a directory and all its contents
    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }

    /// Find the .config/.mucm directory by walking up the directory tree
    fn find_config_dir() -> Result<PathBuf> {
        let mut current_dir = std::env::current_dir()?;
        
        loop {
            let config_dir = current_dir.join(Self::CONFIG_DIR);
            if config_dir.exists() && config_dir.is_dir() {
                return Ok(config_dir);
            }
            
            // Try parent directory
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                anyhow::bail!(
                    "No .config/.mucm directory found. Run 'mucm init' first to initialize a project."
                );
            }
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            anyhow::bail!("No markdown use case manager project found. Run 'mucm init' first.");
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save_in_dir(&self, base_dir: &str) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_path = base_path.join(Self::CONFIG_DIR).join("mucm.toml");
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Get list of available methodologies (those with config files)
    pub fn list_available_methodologies() -> Result<Vec<String>> {
        let methodologies_dir = Self::find_config_dir()?
            .join("methodologies");

        if !methodologies_dir.exists() {
            return Ok(Vec::new());
        }

        let mut methodologies = Vec::new();
        for entry in fs::read_dir(&methodologies_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Files are named {methodology}.toml
                    methodologies.push(name.to_string());
                }
            }
        }

        methodologies.sort();
        Ok(methodologies)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            project: ProjectConfig {
                name: "My Project".to_string(),
                description: "A project managed with use case manager".to_string(),
            },
            directories: DirectoryConfig {
                use_case_dir: "docs/use-cases".to_string(),
                test_dir: "tests/use-cases".to_string(),
                persona_dir: "docs/personas".to_string(),
                template_dir: None,
                toml_dir: Some("use-cases-data".to_string()), // Default: keep source data separate
            },
            templates: TemplateConfig {
                use_case_template: None,
                test_template: None,
                methodologies: vec!["developer".to_string(), "feature".to_string()],
                default_methodology: Some("developer".to_string()),
            },
            generation: GenerationConfig {
                test_language: "rust".to_string(),
                auto_generate_tests: false,
                overwrite_test_documentation: false,
            },
            metadata: MetadataConfig {
                enabled: true,
                include_id: true,
                include_title: true,
                include_category: true,
                include_status: true,
                include_priority: true,
                include_created: true,
                include_last_updated: true,
                include_prerequisites: true,
                include_personas: true,
                include_author: true,
                include_reviewer: true,
                include_business_value: true,
                include_complexity: true,
                include_epic: true,
                include_acceptance_criteria: true,
                include_assumptions: true,
                include_constraints: true,
            },
        }
    }
}
