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
pub struct DirectoryConfig {
    pub use_case_dir: String,
    pub test_dir: String,
    pub template_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub use_case_template: Option<String>,
    pub test_template: Option<String>,
    pub use_case_style: Option<String>, // "simple" or "detailed"
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
    /// Tags array (automatically set to empty)
    pub include_tags: bool,
    /// Test file path (automatically set when tests are generated)
    pub include_test_file: bool,

    // Custom fields that user fills manually (always empty by default)
    // Add field names here that you want to appear in the metadata for manual completion
    // Examples: "author", "reviewer", "business_value", "complexity", "epic", "prerequisites", "test_file"
    pub custom_fields: Vec<String>,
}

impl Config {
    const CONFIG_DIR: &'static str = ".config/.mucm";
    const CONFIG_FILE: &'static str = "mucm.toml";
    const TEMPLATES_DIR: &'static str = "templates";
    const LANGUAGE_PREFIX: &'static str = "lang-";
    const SOURCE_TEMPLATES_DIR: &'static str = "templates"; // Source templates in the project

    pub fn config_path() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::CONFIG_FILE)
    }

    pub fn templates_dir() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR)
    }

    /// Get list of available programming languages from source templates and local config
    pub fn get_available_languages() -> Result<Vec<String>> {
        let mut languages = Vec::new();
        
        // Get built-in languages from the registry
        let language_registry = LanguageRegistry::new();
        let builtin_languages = language_registry.available_languages();

        // First, check source templates directory (built-in languages)
        let source_templates_dir = Path::new(Self::SOURCE_TEMPLATES_DIR);
        if source_templates_dir.exists() {
            if let Ok(entries) = fs::read_dir(source_templates_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if let Some(lang_name) = name.strip_prefix(Self::LANGUAGE_PREFIX) {
                                if !lang_name.is_empty() {
                                    languages.push(lang_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

                // Then check config templates directory
        let current_templates_dir = Self::templates_dir();
        if current_templates_dir.exists() {
            if let Ok(entries) = fs::read_dir(&current_templates_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if let Some(lang_name) = name.strip_prefix(Self::LANGUAGE_PREFIX) {
                                if !lang_name.is_empty() && !languages.contains(&lang_name.to_string()) {
                                    languages.push(lang_name.to_string());
                                }
                            } else if language_registry.is_supported(name) {
                                // Support any language supported by the registry for backward compatibility
                                if !languages.contains(&name.to_string()) {
                                    languages.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Add built-in languages if none found or to supplement discovered ones
        languages.extend(builtin_languages);

        languages.sort();
        languages.dedup();
        Ok(languages)
    }

    /// Get available languages from a specific directory (used for cross-directory checks)
    pub fn get_available_languages_from_dir(base_dir: &str) -> Result<Vec<String>> {
        let base_path = Path::new(base_dir);
        let templates_dir = base_path.join(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR);
        let mut languages = Vec::new();

        if templates_dir.exists() {
            if let Ok(entries) = fs::read_dir(&templates_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if let Some(lang_name) = name.strip_prefix(Self::LANGUAGE_PREFIX) {
                                if !lang_name.is_empty() {
                                    languages.push(lang_name.to_string());
                                }
                            } else if matches!(name, "rust" | "python") {
                                // Support legacy folders for backward compatibility
                                languages.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Always include built-in languages
        for builtin in ["rust", "python"] {
            if !languages.contains(&builtin.to_string()) {
                languages.push(builtin.to_string());
            }
        }

        languages.sort();
        languages.dedup();
        Ok(languages)
    }

    /// Check if a language is supported (either built-in or available in templates)
    pub fn is_language_supported(language: &str) -> Result<bool> {
        let available = Self::get_available_languages()?;
        Ok(available.contains(&language.to_string()))
    }

    pub fn init_project() -> Result<Self> {
        Self::init_project_in_dir(".")
    }

    pub fn init_project_with_language(language: Option<String>) -> Result<Self> {
        Self::init_project_with_language_in_dir(".", language)
    }

    pub fn init_project_in_dir(base_dir: &str) -> Result<Self> {
        Self::init_project_with_language_in_dir(base_dir, None)
    }

    pub fn init_project_with_language_in_dir(base_dir: &str, language: Option<String>) -> Result<Self> {
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

        fs::create_dir_all(&use_case_dir).context("Failed to create use case directory")?;
        fs::create_dir_all(&test_dir).context("Failed to create test directory")?;

        Ok(config)
    }

    fn copy_templates_to_config_with_language_in_dir(base_dir: &str, language: Option<String>) -> Result<()> {
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
    fn copy_language_templates_selective(config_templates_dir: &Path, language: Option<String>) -> Result<()> {
        let Some(lang) = language else {
            return Ok(()); // No language specified, don't copy any language templates
        };

        // First try to copy from source templates (built-in)
        let source_lang_dir = Path::new(Self::SOURCE_TEMPLATES_DIR)
            .join(format!("{}{}", Self::LANGUAGE_PREFIX, &lang));
        
        if source_lang_dir.exists() {
            // Copy from source templates
            let target_lang_dir = if matches!(lang.as_str(), "rust" | "python") {
                // Use legacy format for backward compatibility
                config_templates_dir.join(&lang)
            } else {
                // Use new format for other languages
                config_templates_dir.join(format!("{}{}", Self::LANGUAGE_PREFIX, &lang))
            };

            fs::create_dir_all(&target_lang_dir).context("Failed to create language templates directory")?;

            // Copy all files from source to target
            if let Ok(entries) = fs::read_dir(&source_lang_dir) {
                for entry in entries.flatten() {
                    let source_file = entry.path();
                    if source_file.is_file() {
                        if let Some(filename) = source_file.file_name() {
                            let target_file = target_lang_dir.join(filename);
                            fs::copy(&source_file, &target_file)
                                .context("Failed to copy template file")?;
                            
                            let dir_name = target_lang_dir.file_name()
                                .and_then(|n| n.to_str()).unwrap_or("unknown");
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
                let target_lang_dir = if language_impl.uses_legacy_directory() {
                    // Use legacy format for backward compatibility
                    config_templates_dir.join(language_impl.legacy_directory())
                } else {
                    // Use new format for other languages
                    config_templates_dir.join(format!("{}{}", Self::LANGUAGE_PREFIX, &lang))
                };
                
                fs::create_dir_all(&target_lang_dir).context("Failed to create language templates directory")?;

                let test_template_content = language_impl.test_template();
                let template_path = target_lang_dir.join("test.hbs");
                let mut file = std::fs::File::create(&template_path)
                    .context("Failed to create language template file")?;
                file.write_all(test_template_content.as_bytes())
                    .context("Failed to write language template content")?;
                
                let dir_name = target_lang_dir.file_name()
                    .and_then(|n| n.to_str()).unwrap_or("unknown");
                println!("Created template: {}/test.hbs", dir_name);
            } else {
                // For unsupported languages, create a placeholder directory
                let lang_dir = config_templates_dir.join(format!("{}{}", Self::LANGUAGE_PREFIX, lang));
                fs::create_dir_all(&lang_dir).context("Failed to create language templates directory")?;
                
                // Create a basic test template placeholder
                let test_template_path = lang_dir.join("test.hbs");
                let placeholder_content = format!("// Test template for {} - customize as needed\n// Use case: {{{{title}}}}\n", lang);
                
                let mut file = std::fs::File::create(&test_template_path)
                    .context("Failed to create language template file")?;
                file.write_all(placeholder_content.as_bytes())
                    .context("Failed to write language template content")?;
                println!("Created template: {}/test.hbs", lang_dir.file_name().unwrap().to_str().unwrap());
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

    pub fn load_from_dir(base_dir: &str) -> Result<Self> {
        let base_path = Path::new(base_dir);
        let config_path = base_path.join(Self::CONFIG_DIR).join("mucm.toml");

        if !config_path.exists() {
            anyhow::bail!("No markdown use case manager project found. Run 'mucm init' first.");
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
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
        Self {
            project: ProjectConfig {
                name: "My Project".to_string(),
                description: "A project managed with use case manager".to_string(),
            },
            directories: DirectoryConfig {
                use_case_dir: "docs/use-cases".to_string(),
                test_dir: "tests/use-cases".to_string(),
                template_dir: None,
            },
            templates: TemplateConfig {
                use_case_template: None,
                test_template: None,
                use_case_style: Some("detailed".to_string()),
            },
            generation: GenerationConfig {
                test_language: "rust".to_string(),
                auto_generate_tests: false,          // Changed default
                overwrite_test_documentation: false, // Changed default
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
                include_tags: false,
                include_test_file: true,
                custom_fields: vec![
                    "author".to_string(),
                    "reviewer".to_string(),
                    "business_value".to_string(),
                    "complexity".to_string(),
                    "epic".to_string(),
                    "prerequisites".to_string(),
                ],
            },
        }
    }
}
