// src/config/mod.rs
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
    const CONFIG_DIR: &'static str = ".config/ucm";
    const CONFIG_FILE: &'static str = "ucm.toml";
    const TEMPLATES_DIR: &'static str = "templates";
    
    pub fn config_path() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::CONFIG_FILE)
    }
    
    pub fn templates_dir() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR)
    }
    
    pub fn init_project() -> Result<Self> {
        Self::init_project_in_dir(".")
    }
    
    pub fn init_project_in_dir(base_dir: &str) -> Result<Self> {
        let base_path = Path::new(base_dir);
        let config_dir = base_path.join(Self::CONFIG_DIR);
        
        // Create .config/ucm directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context("Failed to create .config/ucm directory")?;
        }
        
        let config = Self::default();
        config.save_in_dir(base_dir)?;
        
        // Copy templates to .config/ucm/templates/
        Self::copy_templates_to_config_in_dir(base_dir)?;
        
        // Create default directories
        let use_case_dir = base_path.join(&config.directories.use_case_dir);
        let test_dir = base_path.join(&config.directories.test_dir);
        
        fs::create_dir_all(&use_case_dir)
            .context("Failed to create use case directory")?;
        fs::create_dir_all(&test_dir)
            .context("Failed to create test directory")?;

        Ok(config)
    }
    
    fn copy_templates_to_config_in_dir(base_dir: &str) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_templates_dir = base_path.join(Self::CONFIG_DIR).join(Self::TEMPLATES_DIR);
        
        // Create templates directory in config
        fs::create_dir_all(&config_templates_dir)
            .context("Failed to create config templates directory")?;
        
        // Get built-in templates from TemplateEngine
        use crate::core::templates::TemplateEngine;
        
        // Define template files and their content
        let templates = [
            ("use_case_simple.hbs", TemplateEngine::get_use_case_simple_template()),
            ("use_case_detailed.hbs", TemplateEngine::get_use_case_detailed_template()),
            ("overview.hbs", TemplateEngine::get_overview_template()),
        ];
        
        // Copy core templates
        for (template_name, template_content) in templates {
            let template_path = config_templates_dir.join(template_name);
            
            let mut file = std::fs::File::create(&template_path)
                .context("Failed to create template file")?;
            
            file.write_all(template_content.as_bytes())
                .context("Failed to write template content")?;
            
            let template_file = template_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");
            println!("Created template: {}", template_file);
        }
        
        // Create language-specific template directories and files
        Self::copy_language_templates(&config_templates_dir)?;

        Ok(())
    }
    
    /// Copy language-specific templates to config directory
    fn copy_language_templates(config_templates_dir: &Path) -> Result<()> {
        use crate::core::templates::TemplateEngine;
        
        // Create Rust templates directory
        let rust_dir = config_templates_dir.join("rust");
        fs::create_dir_all(&rust_dir)
            .context("Failed to create rust templates directory")?;
            
        let rust_templates = [
            ("test.hbs", TemplateEngine::get_rust_test_template()),
        ];
        
        for (template_name, template_content) in rust_templates {
            let template_path = rust_dir.join(template_name);
            let mut file = std::fs::File::create(&template_path)
                .context("Failed to create rust template file")?;
            file.write_all(template_content.as_bytes())
                .context("Failed to write rust template content")?;
            println!("Created template: rust/{}", template_name);
        }
        
        // Create Python templates directory
        let python_dir = config_templates_dir.join("python");
        fs::create_dir_all(&python_dir)
            .context("Failed to create python templates directory")?;
            
        let python_templates = [
            ("test.hbs", TemplateEngine::get_python_test_template()),
        ];
        
        for (template_name, template_content) in python_templates {
            let template_path = python_dir.join(template_name);
            let mut file = std::fs::File::create(&template_path)
                .context("Failed to create python template file")?;
            file.write_all(template_content.as_bytes())
                .context("Failed to write python template content")?;
            println!("Created template: python/{}", template_name);
        }
        
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        
        if !config_path.exists() {
            anyhow::bail!("No use case manager project found. Run 'ucm init' first.");
        }
        
        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    pub fn load_from_dir(base_dir: &str) -> Result<Self> {
        let base_path = Path::new(base_dir);
        let config_path = base_path.join(Self::CONFIG_DIR).join("ucm.toml");
        
        if !config_path.exists() {
            anyhow::bail!("No use case manager project found. Run 'ucm init' first.");
        }
        
        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    pub fn save_in_dir(&self, base_dir: &str) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_path = base_path.join(Self::CONFIG_DIR).join("ucm.toml");
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .context("Failed to write config file")?;
        
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
                auto_generate_tests: false,  // Changed default
                overwrite_test_documentation: false,  // Changed default
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