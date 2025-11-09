use anyhow::Result;

use crate::controller::ProjectController;
use crate::controller::UseCaseController;

/// CLI runner that delegates to controllers
/// This is a thin adapter between CLI interface and business logic
pub struct CliRunner {
    use_case_controller: Option<UseCaseController>,
}

impl CliRunner {
    /// Create a new CLI runner
    pub fn new() -> Self {
        Self {
            use_case_controller: None,
        }
    }

    /// Load or initialize the use case controller
    fn ensure_use_case_controller(&mut self) -> Result<&mut UseCaseController> {
        if self.use_case_controller.is_none() {
            self.use_case_controller = Some(UseCaseController::new()?);
        }
        Ok(self
            .use_case_controller
            .as_mut()
            .expect("controller was just initialized"))
    }

    /// Initialize a new use case manager project (Step 1: Create config only)
    #[allow(clippy::unused_self)]
    pub fn init_project(
        &mut self,
        language: Option<String>,
        methodology: Option<String>,
    ) -> Result<String> {
        let default_methodology = methodology.unwrap_or_else(|| "feature".to_string());
        let result = ProjectController::init_project(language, default_methodology)?;
        Ok(result.message)
    }

    /// Finalize initialization (Step 2: Copy templates after config review)
    #[allow(clippy::unused_self)]
    pub fn finalize_init(&mut self) -> Result<String> {
        let result = ProjectController::finalize_init()?;
        Ok(result.message)
    }

    /// Create a new use case (uses default methodology from config)
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

    /// Create a new use case with specific methodology
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

    /// List all use cases
    pub fn list_use_cases(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.list_use_cases()
    }

    /// Show project status
    pub fn show_status(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.show_status()
    }

    /// Get all use case IDs for selection
    /// Get all use case IDs for selection prompts
    /// TODO: Use this in interactive mode for use case selection dropdowns
    #[allow(dead_code)]
    pub fn get_use_case_ids(&mut self) -> Result<Vec<String>> {
        let controller = self.ensure_use_case_controller()?;
        let options = controller.get_use_case_ids()?;
        Ok(options.items)
    }

    /// Get all categories in use
    pub fn get_categories(&mut self) -> Result<Vec<String>> {
        let controller = self.ensure_use_case_controller()?;
        let options = controller.get_categories()?;
        Ok(options.items)
    }

    /// Show available languages
    pub fn show_languages() -> Result<String> {
        ProjectController::show_languages()
    }

    /// List available methodologies
    #[allow(clippy::unused_self)]
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

    /// Get information about a specific methodology
    #[allow(clippy::unused_self)]
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

    /// Get default methodology from config
    #[allow(clippy::unused_self)]
    /// Get the default methodology from config
    /// TODO: Use this in interactive mode to pre-fill methodology in interactive create workflow
    #[allow(dead_code)]
    pub fn get_default_methodology(&mut self) -> Result<String> {
        ProjectController::get_default_methodology()
    }

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.regenerate_use_case_with_methodology(use_case_id, methodology)?;
        Ok(result.message)
    }

    /// Regenerate markdown for a single use case from its TOML file
    pub fn regenerate_use_case(&mut self, use_case_id: &str) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.regenerate_use_case(use_case_id)
    }

    /// Regenerate markdown for all use cases from their TOML files
    pub fn regenerate_all_use_cases(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.regenerate_all_use_cases()
    }
}
