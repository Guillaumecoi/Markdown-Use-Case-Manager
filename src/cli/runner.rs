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

use crate::controller::ProjectController;
use crate::controller::UseCaseController;

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
    /// Returns a success message string.
    pub fn init_project(
        &mut self,
        language: Option<String>,
        methodology: Option<String>,
    ) -> Result<String> {
        let default_methodology = methodology.unwrap_or_else(|| "feature".to_string());
        let result = ProjectController::init_project(language, default_methodology)?;
        Ok(result.message)
    }

    /// Finalize project initialization (template copying phase).
    /// 
    /// Completes the initialization by copying template files based on the
    /// previously created configuration. This should be called after reviewing
    /// the generated config files.
    /// 
    /// # Returns
    /// Returns a success message string.
    pub fn finalize_init(&mut self) -> Result<String> {
        let result = ProjectController::finalize_init()?;
        Ok(result.message)
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
    /// Returns a success message string.
    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.create_use_case(title, category, description)?;
        Ok(result.message)
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
    /// Returns a success message string.
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: String,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.create_use_case_with_methodology(
            title,
            category,
            description,
            methodology,
        )?;
        Ok(result.message)
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

    /// Get all use case IDs for selection prompts.
    /// 
    /// Returns a list of all use case identifiers in the project.
    /// Useful for interactive mode dropdowns or validation.
    /// 
    /// # Returns
    /// Returns a vector of use case ID strings.
    #[allow(dead_code)]
    pub fn get_use_case_ids(&mut self) -> Result<Vec<String>> {
        let controller = self.ensure_use_case_controller()?;
        let options = controller.get_use_case_ids()?;
        Ok(options.items)
    }

    /// Get all categories currently in use.
    /// 
    /// Returns a list of all categories that have use cases assigned to them.
    /// 
    /// # Returns
    /// Returns a vector of category name strings.
    pub fn get_categories(&mut self) -> Result<Vec<String>> {
        let controller = self.ensure_use_case_controller()?;
        let options = controller.get_categories()?;
        Ok(options.items)
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
    /// including its display name and description.
    /// 
    /// # Arguments
    /// * `methodology` - The name of the methodology to query
    /// 
    /// # Returns
    /// Returns a formatted string with methodology details, or a not-found message.
    pub fn get_methodology_info(&mut self, methodology: String) -> Result<String> {
        let methodologies = ProjectController::get_available_methodologies()?;

        match methodologies.iter().find(|m| m.name == methodology) {
            Some(info) => Ok(format!(
                "Methodology: {}\nDisplay Name: {}\nDescription: {}",
                info.name, info.display_name, info.description
            )),
            None => Ok(format!("Methodology '{}' not found.", methodology)),
        }
    }

    /// Get the default methodology from project configuration.
    /// 
    /// Retrieves the methodology configured as default for the project.
    /// 
    /// # Returns
    /// Returns the default methodology name as a string.
    #[allow(dead_code)]
    pub fn get_default_methodology(&mut self) -> Result<String> {
        ProjectController::get_default_methodology()
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
    /// Returns a success message string.
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.regenerate_use_case_with_methodology(use_case_id, methodology)?;
        Ok(result.message)
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
    pub fn regenerate_use_case(&mut self, use_case_id: &str) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.regenerate_use_case(use_case_id)
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
