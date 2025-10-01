// src/config/mod.rs
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
    const CONFIG_DIR: &'static str = ".config";
    const CONFIG_FILE: &'static str = "ucm.toml";
    
    pub fn config_path() -> PathBuf {
        Path::new(Self::CONFIG_DIR).join(Self::CONFIG_FILE)
    }
    
    pub fn init_project() -> Result<Self> {
        let config_dir = Path::new(Self::CONFIG_DIR);
        
        // Create .config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .context("Failed to create .config directory")?;
        }
        
        let config = Self::default();
        config.save()?;
        
        // Create default directories
        fs::create_dir_all(&config.directories.use_case_dir)
            .context("Failed to create use case directory")?;
        fs::create_dir_all(&config.directories.test_dir)
            .context("Failed to create test directory")?;
        
        Ok(config)
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
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
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