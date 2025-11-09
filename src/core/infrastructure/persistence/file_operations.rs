// File operation utilities for persistence layer
use crate::config::Config;
use crate::core::domain::entities::UseCase;
use crate::core::infrastructure::template_engine::to_snake_case;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Helper functions for file operations
pub struct FileOperations {
    config: Config,
}

impl FileOperations {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Save a test file
    /// This should create test files in the configured test_dir with proper structure
    pub fn save_test_file(
        &self,
        use_case: &UseCase,
        test_content: &str,
        file_extension: &str,
    ) -> Result<()> {
        // Create test directory with category subdirectory if it doesn't exist
        let test_dir = Path::new(&self.config.directories.test_dir);
        let category_dir = test_dir.join(to_snake_case(&use_case.category));
        fs::create_dir_all(&category_dir)?;

        // Generate filename: snake_case of use case ID with extension
        let file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = category_dir.join(file_name);

        // Write the test file
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

    /// Check if a test file exists for a given use case
    pub fn test_file_exists(&self, use_case: &UseCase, file_extension: &str) -> bool {
        let test_dir =
            Path::new(&self.config.directories.test_dir).join(to_snake_case(&use_case.category));
        let test_file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = test_dir.join(test_file_name);
        test_path.exists()
    }
}
