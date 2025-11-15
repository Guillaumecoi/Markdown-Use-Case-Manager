//! Test generator for use case test documentation.
//!
//! Handles generation of test files from use cases using language-specific templates.

use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::config::Config;
use crate::core::file_operations::FileOperations;
use crate::core::{to_snake_case, TemplateEngine, UseCase};
use crate::presentation::UseCaseFormatter;

/// Generator for use case test documentation.
pub struct TestGenerator {
    config: Config,
    file_operations: FileOperations,
    template_engine: TemplateEngine,
}

impl TestGenerator {
    /// Creates a new test generator with the given configuration.
    pub fn new(config: Config) -> Self {
        let file_operations = FileOperations::new(config.clone());
        let template_engine = TemplateEngine::with_config(Some(&config));
        Self {
            config,
            file_operations,
            template_engine,
        }
    }

    /// Generates and saves a test file for the given use case.
    ///
    /// Returns `Ok(())` if the file was generated or skipped (when file exists and overwrite is disabled).
    pub fn generate(&self, use_case: &UseCase) -> Result<()> {
        // Skip test generation if test_language is "none"
        if self.config.generation.test_language == "none" {
            return Ok(());
        }

        // Check if test file already exists and overwrite is disabled
        let file_extension = self.get_file_extension();
        if self
            .file_operations
            .test_file_exists(use_case, &file_extension)
            && !self.config.generation.overwrite_test_documentation
        {
            // Use the formatter to display the skipped message
            UseCaseFormatter::display_test_skipped();
            return Ok(());
        }

        // Generate test content using template
        let test_content = self.generate_content(use_case)?;

        // Save the test file
        self.file_operations
            .save_test_file(use_case, &test_content, &file_extension)?;

        // Get the test file path for display
        let test_file_path = self.get_file_path(use_case)?;

        // Use the formatter to display the generated message
        UseCaseFormatter::display_test_generated(
            &use_case.id,
            &test_file_path.display().to_string(),
        );

        Ok(())
    }

    /// Generates test content for a use case without saving to file.
    fn generate_content(&self, use_case: &UseCase) -> Result<String> {
        // Convert UseCase to JSON for template engine
        let use_case_json = serde_json::to_value(use_case)?;
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json)?;

        // Merge extra fields into top-level HashMap
        if let Some(Value::Object(extra_map)) = data.remove("extra") {
            for (key, value) in extra_map {
                data.insert(key, value);
            }
        }

        // Add generated timestamp
        data.insert(
            "generated_at".to_string(),
            json!(chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string()),
        );

        // Add snake_case version of title for class names
        if let Some(Value::String(title)) = data.get("title") {
            data.insert("title_snake_case".to_string(), json!(to_snake_case(title)));
        }

        // Render using test template for the configured language
        self.template_engine
            .render_test(&self.config.generation.test_language, &data)
    }

    /// Gets the file extension for test files based on the configured language.
    fn get_file_extension(&self) -> String {
        match self.config.generation.test_language.as_str() {
            "python" => "py".to_string(),
            "javascript" => "js".to_string(),
            "rust" => "rs".to_string(),
            "none" => "txt".to_string(), // fallback for none
            _ => "txt".to_string(),      // fallback for unknown
        }
    }

    /// Gets the full file path for a use case's test file.
    fn get_file_path(&self, use_case: &UseCase) -> Result<std::path::PathBuf> {
        let test_dir = std::path::Path::new(&self.config.directories.test_dir);
        let category_dir = test_dir.join(to_snake_case(&use_case.category));
        let file_extension = self.get_file_extension();
        let file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        Ok(category_dir.join(file_name))
    }
}
