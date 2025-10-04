// src/config/mod.rs
use crate::core::languages::LanguageRegistry;
use crate::core::templates::TemplateEngine;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub use_case_template: Option<String>,
    pub test_template: Option<String>,
    pub use_case_style: Option<String>, // "simple" or "detailed"
    pub methodology: Option<String>, // "simple", "cockburn", "unified_process", "bdd_gherkin"
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
    pub fn new_with_methodology(methodology: &str) -> Self {
        let mut config = Self::default();
        
        match methodology {
            "simple" => {
                config.templates.methodology = Some("simple".to_string());
                config.templates.use_case_style = Some("simple".to_string());
                config.metadata.enabled = false; // Minimal metadata for simple approach
                config.generation.auto_generate_tests = false;
            },
            "cockburn" => {
                config.templates.methodology = Some("cockburn".to_string());
                config.metadata.enabled = true;
                config.metadata.include_personas = true;
                config.metadata.include_prerequisites = true;
                config.metadata.include_business_value = true;
                config.metadata.include_acceptance_criteria = true;
                config.generation.auto_generate_tests = true; // Goal-oriented testing
            },
            "unified_process" | "rup" => {
                config.templates.methodology = Some("unified_process".to_string());
                config.metadata.enabled = true;
                config.metadata.include_author = true;
                config.metadata.include_personas = true;
                config.metadata.include_prerequisites = true;
                config.metadata.include_acceptance_criteria = true;
                config.metadata.include_constraints = true;
                config.generation.auto_generate_tests = true;
                config.generation.overwrite_test_documentation = true; // Formal documentation
            },
            "bdd_gherkin" | "bdd" => {
                config.templates.methodology = Some("bdd_gherkin".to_string());
                config.metadata.enabled = true;
                config.metadata.include_acceptance_criteria = true;
                config.metadata.include_personas = true;
                config.generation.auto_generate_tests = true; // BDD focuses on automated testing
                config.generation.test_language = "rust".to_string(); // Could be configured based on project
            },
            _ => {
                // Default to simple for unknown methodologies
                config.templates.methodology = Some("simple".to_string());
            }
        }
        
        config
    }

    /// Initialize project with a pre-configured Config instance
    pub fn init_project_with_config(config: Config) -> Result<Config> {
        // Ensure we're not already in a project
        if Self::load().is_ok() {
            anyhow::bail!("A use case manager project already exists in this directory or a parent directory");
        }

        // Create .config/.mucm directory if it doesn't exist
        let base_path = Path::new(".");
        let config_dir = base_path.join(Self::CONFIG_DIR);
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create .config/.mucm directory")?;
        }

        // Save the configuration first
        config.save_in_dir(".")?;

        // Copy templates to .config/.mucm/templates/
        Self::copy_templates_to_config_with_language_in_dir(".", Some(config.generation.test_language.clone()))?;

        // Create default directories
        let base_path = Path::new(".");
        let use_case_dir = base_path.join(&config.directories.use_case_dir);
        let test_dir = base_path.join(&config.directories.test_dir);
        let persona_dir = base_path.join(&config.directories.persona_dir);

        fs::create_dir_all(&use_case_dir).context("Failed to create use case directory")?;
        fs::create_dir_all(&test_dir).context("Failed to create test directory")?;
        fs::create_dir_all(&persona_dir).context("Failed to create persona directory")?;
        
        Ok(config)
    }

    /// Get methodology-specific recommendations as a human-readable string
    pub fn methodology_recommendations(methodology: &str) -> String {
        match methodology {
            "simple" => {
                "Simple Methodology Recommendations:
- Minimal metadata enabled
- Quick documentation focus
- No automatic test generation
- Best for: Small teams, rapid prototyping, informal documentation".to_string()
            },
            "cockburn" => {
                "Cockburn Goal-Oriented Methodology Recommendations:
- Full metadata including personas and business value
- Prerequisites and acceptance criteria tracking
- Automatic test generation enabled
- Best for: Complex business domains, stakeholder-heavy projects".to_string()
            },
            "unified_process" | "rup" => {
                "Rational Unified Process (RUP) Methodology Recommendations:
- Comprehensive metadata with versioning and authorship
- Formal documentation with prerequisites and acceptance criteria
- Test generation with documentation overwrite enabled
- Best for: Enterprise projects, regulated industries, formal processes".to_string()
            },
            "bdd_gherkin" | "bdd" => {
                "Behavior-Driven Development (BDD) Methodology Recommendations:
- Acceptance criteria and persona-focused metadata
- Automatic test generation strongly enabled
- Collaborative documentation approach
- Best for: Agile teams, automated testing focus, customer collaboration".to_string()
            },
            _ => "Unknown methodology. Using simple methodology defaults.".to_string()
        }
    }
    const CONFIG_DIR: &'static str = ".config/.mucm";
    const CONFIG_FILE: &'static str = "mucm.toml";
    const TEMPLATES_DIR: &'static str = "templates";
    const LANGUAGE_PREFIX: &'static str = "lang-";
    const SOURCE_TEMPLATES_DIR: &'static str = "templates"; // Source templates in the project

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

    pub fn init_project_with_language(language: Option<String>) -> Result<Self> {
        Self::init_project_with_language_in_dir(".", language)
    }

    pub fn init_project_with_language_in_dir(
        base_dir: &str,
        language: Option<String>,
    ) -> Result<Self> {
        let base_path = Path::new(base_dir);
        let config_dir = base_path.join(Self::CONFIG_DIR);

        // Validate language if provided - check both current directory and built-ins
        if let Some(ref lang) = language {
            let language_registry = LanguageRegistry::new();

            // First check if the language is supported by the built-in registry
            if language_registry.is_supported(lang) {
                // Language is supported, continue
            } else {
                // Check available languages from current working directory as fallback
                let available_languages = Self::get_available_languages()?;
                if !available_languages.contains(lang) {
                    anyhow::bail!("Unsupported language '{}'. Supported languages: {}. Add templates to .config/.mucm/templates/lang-{}/ to support this language.", 
                                lang, available_languages.join(", "), lang);
                }
            }
        }

        // Create .config/.mucm directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create .config/.mucm directory")?;
        }

        let mut config = Self::default();

        // Set the test language if provided
        if let Some(ref lang) = language {
            config.generation.test_language = lang.clone();
        }

        config.save_in_dir(base_dir)?;

        // Copy templates to .config/.mucm/templates/
        Self::copy_templates_to_config_with_language_in_dir(base_dir, language)?;

        // Create default directories
        let use_case_dir = base_path.join(&config.directories.use_case_dir);
        let test_dir = base_path.join(&config.directories.test_dir);
        let persona_dir = base_path.join(&config.directories.persona_dir);

        fs::create_dir_all(&use_case_dir).context("Failed to create use case directory")?;
        fs::create_dir_all(&test_dir).context("Failed to create test directory")?;
        fs::create_dir_all(&persona_dir).context("Failed to create persona directory")?;

        Ok(config)
    }

    fn copy_templates_to_config_with_language_in_dir(
        base_dir: &str,
        language: Option<String>,
    ) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_templates_dir = base_path.join(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR);

        // Create templates directory in config
        fs::create_dir_all(&config_templates_dir)
            .context("Failed to create config templates directory")?;

        // Define template files and their content
        let templates = [
            (
                "use_case_simple.hbs",
                TemplateEngine::get_use_case_simple_template(),
            ),
            (
                "use_case_detailed.hbs",
                TemplateEngine::get_use_case_detailed_template(),
            ),
            ("overview.hbs", TemplateEngine::get_overview_template()),
        ];

        // Copy core templates
        for (template_name, template_content) in templates {
            let template_path = config_templates_dir.join(template_name);

            let mut file =
                std::fs::File::create(&template_path).context("Failed to create template file")?;

            file.write_all(template_content.as_bytes())
                .context("Failed to write template content")?;

            let template_file = template_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");
            println!("Created template: {}", template_file);
        }

        // Create language-specific template directories and files
        Self::copy_language_templates_selective(&config_templates_dir, language)?;

        Ok(())
    }

    /// Copy language-specific templates to config directory with optional language filter
    fn copy_language_templates_selective(
        config_templates_dir: &Path,
        language: Option<String>,
    ) -> Result<()> {
        let Some(lang) = language else {
            return Ok(()); // No language specified, don't copy any language templates
        };

        // First try to copy from source templates (built-in)
        let source_lang_dir = Path::new(Self::SOURCE_TEMPLATES_DIR).join(format!(
            "{}{}",
            Self::LANGUAGE_PREFIX,
            &lang
        ));

        if source_lang_dir.exists() {
            // Copy from source templates
            let target_lang_dir =
                config_templates_dir.join(format!("{}{}", Self::LANGUAGE_PREFIX, &lang));

            fs::create_dir_all(&target_lang_dir)
                .context("Failed to create language templates directory")?;

            // Copy all files from source to target
            if let Ok(entries) = fs::read_dir(&source_lang_dir) {
                for entry in entries.flatten() {
                    let source_file = entry.path();
                    if source_file.is_file() {
                        if let Some(filename) = source_file.file_name() {
                            let target_file = target_lang_dir.join(filename);
                            fs::copy(&source_file, &target_file)
                                .context("Failed to copy template file")?;

                            let dir_name = target_lang_dir
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown");
                            let file_name = filename.to_str().unwrap_or("unknown");
                            println!("Created template: {}/{}", dir_name, file_name);
                        }
                    }
                }
            }
        } else {
            // Fallback to built-in template generation using the language registry
            let language_registry = LanguageRegistry::new();
            if let Some(language_impl) = language_registry.get(&lang) {
                let target_lang_dir =
                    config_templates_dir.join(format!("{}{}", Self::LANGUAGE_PREFIX, &lang));

                fs::create_dir_all(&target_lang_dir)
                    .context("Failed to create language templates directory")?;

                let test_template_content = language_impl.test_template();
                let template_path = target_lang_dir.join("test.hbs");
                let mut file = std::fs::File::create(&template_path)
                    .context("Failed to create language template file")?;
                file.write_all(test_template_content.as_bytes())
                    .context("Failed to write language template content")?;

                let dir_name = target_lang_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                println!("Created template: {}/test.hbs", dir_name);
            } else {
                // For unsupported languages, create a placeholder directory
                let lang_dir =
                    config_templates_dir.join(format!("{}{}", Self::LANGUAGE_PREFIX, lang));
                fs::create_dir_all(&lang_dir)
                    .context("Failed to create language templates directory")?;

                // Create a basic test template placeholder
                let test_template_path = lang_dir.join("test.hbs");
                let placeholder_content = format!(
                    "// Test template for {} - customize as needed\n// Use case: {{{{title}}}}\n",
                    lang
                );

                let mut file = std::fs::File::create(&test_template_path)
                    .context("Failed to create language template file")?;
                file.write_all(placeholder_content.as_bytes())
                    .context("Failed to write language template content")?;
                println!(
                    "Created template: {}/test.hbs",
                    lang_dir.file_name().unwrap().to_str().unwrap()
                );
            }
        }

        Ok(())
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
            },
            templates: TemplateConfig {
                use_case_template: None,
                test_template: None,
                use_case_style: Some("detailed".to_string()),
                methodology: Some("simple".to_string()),
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
