//! # Interactive Runner
//!
//! Business logic layer for interactive CLI workflows.
//! Similar to CliRunner but handles interactive user workflows,
//! coordinating between presentation layer and controllers.
//!
//! ## Responsibilities
//!
//! - Interactive workflow orchestration (initialization, menu navigation)
//! - User input validation and processing
//! - Coordination between UI presentation and business logic
//! - State management for interactive sessions
//!
//! ## Architecture
//!
//! The InteractiveRunner serves as the business logic coordinator for
//! interactive CLI operations, delegating to controllers for core business
//! logic while handling the interactive workflow state and user experience.

use anyhow::Result;

use crate::controller::{ProjectController, UseCaseController};

/// Interactive runner that coordinates interactive CLI workflows
pub struct InteractiveRunner {
    use_case_controller: Option<UseCaseController>,
}

impl InteractiveRunner {
    /// Create a new interactive runner instance
    pub fn new() -> Self {
        Self {
            use_case_controller: None,
        }
    }

    /// Ensure the use case controller is loaded
    fn ensure_use_case_controller(&mut self) -> Result<&mut UseCaseController> {
        if self.use_case_controller.is_none() {
            self.use_case_controller = Some(UseCaseController::new()?);
        }
        Ok(self
            .use_case_controller
            .as_mut()
            .expect("controller was just initialized"))
    }

    /// Get available programming languages for selection
    pub fn get_available_languages(&self) -> Result<Vec<String>> {
        let selection_options = ProjectController::get_available_languages()?;
        Ok(selection_options.items)
    }

    /// Get available methodologies with descriptions
    pub fn get_available_methodologies(&self) -> Result<Vec<MethodologyInfo>> {
        ProjectController::get_available_methodologies()
    }

    /// Initialize project with selected options
    pub fn initialize_project(
        &mut self,
        language: Option<String>,
        default_methodology: String,
    ) -> Result<String> {
        // This delegates to CliRunner for consistency, but we could call controller directly
        // For now, we'll keep the existing pattern
        use crate::cli::standard::CliRunner;
        let mut runner = CliRunner::new();
        let result = runner.init_project(language, Some(default_methodology))?;
        Ok(result.message)
    }

    /// Finalize project initialization
    pub fn finalize_initialization(&mut self) -> Result<String> {
        use crate::cli::standard::CliRunner;
        let mut runner = CliRunner::new();
        let result = runner.finalize_init()?;
        Ok(result.message)
    }

    /// Create a use case interactively
    pub fn create_use_case_interactive(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: Option<String>,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = if let Some(method) = methodology {
            controller.create_use_case_with_methodology(title, category, description, method)?
        } else {
            controller.create_use_case(title, category, description)?
        };
        Ok(result.message)
    }

    /// List use cases
    pub fn list_use_cases(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.list_use_cases()
    }

    /// Show project status
    pub fn show_status(&mut self) -> Result<()> {
        let controller = self.ensure_use_case_controller()?;
        controller.show_status()
    }
}

// Re-export for convenience
pub use crate::controller::MethodologyInfo;