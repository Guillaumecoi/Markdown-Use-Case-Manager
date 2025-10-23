// src/core/services/file_service.rs
use crate::config::Config;
use crate::core::models::UseCase;
use crate::core::template_engine::to_snake_case;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Service responsible for all file I/O operations
/// Handles saving and loading of use case files
/// 
/// Architecture:
/// - TOML files (.toml) are the source of truth - users edit these
/// - Markdown files (.md) are generated documentation - regenerated from TOML
#[derive(Clone)]
pub struct FileService {
    config: Config,
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Save a use case to both TOML and Markdown files
    /// 
    /// This creates two files:
    /// - {id}.toml - Source of truth containing all use case data (in toml_dir)
    /// - {id}.md - Generated documentation for human reading (in use_case_dir)
    pub fn save_use_case(&self, use_case: &UseCase, markdown_content: &str) -> Result<()> {
        let category_snake = to_snake_case(&use_case.category);
        
        // Create TOML directory structure (source files)
        let toml_dir = Path::new(self.config.directories.get_toml_dir())
            .join(&category_snake);
        fs::create_dir_all(&toml_dir)?;

        // Create markdown directory structure (generated docs)
        let md_dir = Path::new(&self.config.directories.use_case_dir)
            .join(&category_snake);
        fs::create_dir_all(&md_dir)?;

        // Save TOML file (source of truth)
        let toml_path = toml_dir.join(format!("{}.toml", use_case.id));
        let toml_content = toml::to_string_pretty(use_case)?;
        fs::write(&toml_path, toml_content)?;

        // Save markdown file (generated output)
        let md_path = md_dir.join(format!("{}.md", use_case.id));
        fs::write(&md_path, markdown_content)?;

        Ok(())
    }

    /// Load all use cases from TOML files
    /// 
    /// Scans the TOML directory for .toml files and deserializes them.
    /// Markdown files are ignored during loading - they're just generated output.
    pub fn load_use_cases(&self) -> Result<Vec<UseCase>> {
        let toml_dir = Path::new(self.config.directories.get_toml_dir());
        let mut use_cases = Vec::new();

        if !toml_dir.exists() {
            return Ok(use_cases); // No use cases yet
        }

        for entry in walkdir::WalkDir::new(toml_dir) {
            let entry = entry?;
            
            // Only process .toml files that start with "UC-" (use case ID pattern)
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "toml")
                && entry.path().file_name().is_some_and(|name| {
                    name.to_string_lossy().starts_with("UC-")
                })
            {
                let content = fs::read_to_string(entry.path())?;
                let use_case: UseCase = toml::from_str(&content)?;
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    /// Save a test file
    pub fn save_test_file(
        &self,
        use_case: &UseCase,
        test_content: &str,
        file_extension: &str,
    ) -> Result<()> {
        if use_case.scenarios.is_empty() {
            return Ok(()); // No scenarios = no tests
        }

        let test_dir =
            Path::new(&self.config.directories.test_dir).join(to_snake_case(&use_case.category));
        fs::create_dir_all(&test_dir)?;

        let test_file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = test_dir.join(test_file_name);

        fs::write(&test_path, test_content)?;

        Ok(())
    }

    /// Save overview file
    pub fn save_overview(&self, content: &str) -> Result<()> {
        let overview_path = Path::new(&self.config.directories.use_case_dir).join("README.md");
        fs::write(&overview_path, content)?;
        println!("Generated overview at: {}", overview_path.display());
        Ok(())
    }

    /// Check if a test file exists
    pub fn test_file_exists(&self, use_case: &UseCase, file_extension: &str) -> bool {
        let test_dir =
            Path::new(&self.config.directories.test_dir).join(to_snake_case(&use_case.category));
        let test_file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = test_dir.join(test_file_name);
        test_path.exists()
    }
}
