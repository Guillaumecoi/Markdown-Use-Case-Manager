/// CLI Runner - Core business logic adapter.
///
/// The CliRunner serves as the main interface between the CLI layer and the
/// application's business logic. It provides high-level operations for use case
/// management, project initialization, and methodology handling.
///
/// This runner delegates to specialized controllers:
/// - `ProjectController`: Handles project-level operations (init, config, templates)
/// - `UseCaseController`: Manages individual use cases (CRUD, regeneration)
///
/// The runner maintains lazy-loaded controllers to avoid unnecessary initialization
/// and provides a clean, error-handling facade for CLI command handlers.
use anyhow::Result;

use crate::controller::{DisplayResult, ProjectController, UseCaseController};

/// CLI runner that delegates to controllers
/// This is a thin adapter between CLI interface and business logic
pub struct CliRunner {
    use_case_controller: Option<UseCaseController>,
}

impl CliRunner {
    /// Create a new CLI runner instance with uninitialized controllers.
    pub fn new() -> Self {
        Self {
            use_case_controller: None,
        }
    }

    /// Sanitize an optional string input by trimming whitespace and filtering empty strings.
    ///
    /// Returns None if the input is None or contains only whitespace.
    /// Returns Some(trimmed_string) if the input contains non-whitespace characters.
    fn sanitize_optional_string(input: Option<String>) -> Option<String> {
        input
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
    }

    /// Sanitize a required string input by trimming whitespace.
    ///
    /// Preserves internal whitespace but removes leading/trailing whitespace.
    fn sanitize_required_string(input: String) -> String {
        input.trim().to_string()
    }

    /// Ensure the use case controller is loaded.
    fn ensure_use_case_controller(&mut self) -> Result<&mut UseCaseController> {
        if self.use_case_controller.is_none() {
            self.use_case_controller = Some(UseCaseController::new()?);
        }
        Ok(self
            .use_case_controller
            .as_mut()
            .expect("controller was just initialized"))
    }

    /// Initialize a new use case manager project (configuration phase).
    ///
    /// Creates the initial project structure and configuration files.
    /// This is the first step of initialization - templates are copied later
    /// in `finalize_init()` to allow config review.
    ///
    /// # Arguments
    /// * `language` - Optional programming language for code templates
    /// * `methodology` - Optional default methodology (defaults to "feature")
    ///
    /// # Returns
    /// Returns a DisplayResult with success message.
    pub fn init_project(
        &mut self,
        language: Option<String>,
        methodology: Option<String>,
    ) -> Result<DisplayResult> {
        // Sanitize inputs: trim whitespace and filter out empty strings
        let sanitized_language = Self::sanitize_optional_string(language);
        let sanitized_methodology =
            Self::sanitize_optional_string(methodology).unwrap_or_else(|| "feature".to_string());

        let result = ProjectController::init_project(sanitized_language, sanitized_methodology)?;
        Ok(result)
    }

    /// Finalize project initialization (template copying phase).
    ///
    /// Completes the initialization by copying template files based on the
    /// previously created configuration. This should be called after reviewing
    /// the generated config files.
    ///
    /// # Returns
    /// Returns a DisplayResult with completion message.
    pub fn finalize_init(&mut self) -> Result<DisplayResult> {
        let result = ProjectController::finalize_init()?;
        Ok(result)
    }

    /// Create a new use case using the project's default methodology.
    ///
    /// Generates a new use case with the specified details, using whatever
    /// methodology is configured as default for the project.
    ///
    /// # Arguments
    /// * `title` - The use case title
    /// * `category` - The category for organization
    /// * `description` - Optional detailed description
    ///
    /// # Returns
    /// Returns a DisplayResult with success message.
    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<DisplayResult> {
        let controller = self.ensure_use_case_controller()?;
        controller.create_use_case(
            Self::sanitize_required_string(title),
            Self::sanitize_required_string(category),
            Self::sanitize_optional_string(description),
        )
    }

    /// Create a new use case with a specific methodology.
    ///
    /// Generates a new use case with the specified details, overriding the
    /// project's default methodology with the provided one.
    ///
    /// # Arguments
    /// * `title` - The use case title
    /// * `category` - The category for organization
    /// * `description` - Optional detailed description
    /// * `methodology` - The methodology to use for documentation generation
    ///
    /// # Returns
    /// Returns a DisplayResult with success message.
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: String,
    ) -> Result<DisplayResult> {
        let controller = self.ensure_use_case_controller()?;
        controller.create_use_case_with_methodology(
            Self::sanitize_required_string(title),
            Self::sanitize_required_string(category),
            Self::sanitize_optional_string(description),
            Self::sanitize_required_string(methodology),
        )
    }

    /// List all use cases in the project.
    ///
    /// Displays information about all existing use cases, including their
    /// titles, categories, and current status.
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an error if listing fails.
    pub fn list_use_cases(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.list_use_cases()
    }

    /// Display the current project status.
    ///
    /// Shows information about the project's initialization state,
    /// configuration, and available use cases.
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an error if status retrieval fails.
    pub fn show_status(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.show_status()
    }

    /// Display available programming languages.
    ///
    /// Shows the list of supported programming languages for code templates.
    ///
    /// # Returns
    /// Returns a formatted string listing available languages.
    pub fn show_languages() -> Result<String> {
        ProjectController::show_languages()
    }

    /// List all available methodologies.
    ///
    /// Retrieves and formats information about all supported documentation
    /// methodologies, including their names and descriptions.
    ///
    /// # Returns
    /// Returns a formatted string with methodology information.
    pub fn list_methodologies(&mut self) -> Result<String> {
        let methodologies = ProjectController::get_available_methodologies()?;

        if methodologies.is_empty() {
            return Ok("No methodologies available.".to_string());
        }

        let mut result = String::from("Available methodologies:\n");
        for info in methodologies {
            result.push_str(&format!("  - {}: {}\n", info.name, info.description));
        }

        Ok(result)
    }

    /// Get detailed information about a specific methodology.
    ///
    /// Retrieves comprehensive information about the requested methodology,
    /// including its display name, description, when to use it, and key features.
    ///
    /// # Arguments
    /// * `methodology` - The name of the methodology to query
    ///
    /// # Returns
    /// Returns a formatted string with methodology details, or a not-found message.
    pub fn get_methodology_info(&mut self, methodology: String) -> Result<String> {
        use crate::config::Config;
        use crate::core::{Methodology, MethodologyRegistry};

        let sanitized_methodology = Self::sanitize_required_string(methodology);

        // Always load methodology metadata (info.toml) from source templates
        let templates_dir = Config::get_metadata_load_dir()?;
        let registry = MethodologyRegistry::new_dynamic(&templates_dir)?;

        match registry.get(&sanitized_methodology) {
            Some(methodology) => {
                let mut result = format!(
                    "=== {} ===\n\n{}\n\n",
                    methodology.title(),
                    methodology.description()
                );

                result.push_str("When to Use:\n");
                for item in methodology.when_to_use() {
                    result.push_str(&format!("  • {}\n", item));
                }

                result.push_str("\nKey Features:\n");
                for item in methodology.key_features() {
                    result.push_str(&format!("  • {}\n", item));
                }

                result.push_str("\nAvailable Levels:\n");
                for level in methodology.levels() {
                    result.push_str(&format!(
                        "  • {} ({}): {}\n",
                        level.name, level.filename, level.description
                    ));
                }

                result.push_str(&format!(
                    "\nPreferred Style: {}\n",
                    methodology.preferred_style()
                ));

                Ok(result)
            }
            None => Ok(format!(
                "Methodology '{}' not found.",
                sanitized_methodology
            )),
        }
    }

    /// Regenerate a use case with a different methodology.
    ///
    /// Updates an existing use case to use a new methodology, regenerating
    /// its documentation accordingly.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to regenerate
    /// * `methodology` - The new methodology to apply
    ///
    /// # Returns
    /// Returns a DisplayResult with success message.
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<DisplayResult> {
        let controller = self.ensure_use_case_controller()?;
        controller.regenerate_use_case_with_methodology(
            Self::sanitize_required_string(use_case_id),
            Self::sanitize_required_string(methodology),
        )
    }

    /// Regenerate documentation for a single use case.
    ///
    /// Regenerates the markdown documentation for the specified use case
    /// using its current methodology.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to regenerate
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an error if regeneration fails.
    pub fn regenerate_use_case(&mut self, use_case_id: String) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.regenerate_use_case(&Self::sanitize_required_string(use_case_id))
    }

    /// Regenerate documentation for all use cases.
    ///
    /// Regenerates markdown documentation for all use cases in the project
    /// using their current methodologies.
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an error if any regeneration fails.
    pub fn regenerate_all_use_cases(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.regenerate_all_use_cases()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test sanitization of optional strings
    #[test]
    fn test_sanitize_optional_string() {
        // Test None input
        assert_eq!(CliRunner::sanitize_optional_string(None), None);

        // Test empty string
        assert_eq!(
            CliRunner::sanitize_optional_string(Some("".to_string())),
            None
        );

        // Test whitespace-only string
        assert_eq!(
            CliRunner::sanitize_optional_string(Some("   ".to_string())),
            None
        );

        // Test string with leading/trailing whitespace
        assert_eq!(
            CliRunner::sanitize_optional_string(Some("  hello  ".to_string())),
            Some("hello".to_string())
        );

        // Test string with internal whitespace (should be preserved)
        assert_eq!(
            CliRunner::sanitize_optional_string(Some("  hello world  ".to_string())),
            Some("hello world".to_string())
        );

        // Test string with no whitespace
        assert_eq!(
            CliRunner::sanitize_optional_string(Some("hello".to_string())),
            Some("hello".to_string())
        );
    }

    /// Test sanitization of required strings
    #[test]
    fn test_sanitize_required_string() {
        // Test string with leading/trailing whitespace
        assert_eq!(
            CliRunner::sanitize_required_string("  hello  ".to_string()),
            "hello".to_string()
        );

        // Test string with internal whitespace (should be preserved)
        assert_eq!(
            CliRunner::sanitize_required_string("  hello world  ".to_string()),
            "hello world".to_string()
        );

        // Test string with no whitespace
        assert_eq!(
            CliRunner::sanitize_required_string("hello".to_string()),
            "hello".to_string()
        );

        // Test empty string
        assert_eq!(
            CliRunner::sanitize_required_string("".to_string()),
            "".to_string()
        );

        // Test whitespace-only string
        assert_eq!(
            CliRunner::sanitize_required_string("   ".to_string()),
            "".to_string()
        );
    }
}
