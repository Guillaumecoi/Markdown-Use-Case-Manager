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

    /// Create a persona interactively
    pub fn create_persona_interactive(
        &mut self,
        id: String,
        name: String,
        description: String,
        goal: String,
        context: Option<String>,
        tech_level: Option<u8>,
        usage_frequency: Option<String>,
    ) -> Result<String> {
        use crate::cli::args::PersonaCommands;
        use crate::cli::standard::handle_persona_command;
        use crate::config::Config;

        let config = Config::load()?;
        let command = PersonaCommands::Create {
            id,
            name,
            description,
            goal,
            context,
            tech_level,
            usage_frequency,
        };

        handle_persona_command(command, &config)?;
        Ok("Persona created successfully!".to_string())
    }

    /// List all personas
    pub fn list_personas(&self) -> Result<()> {
        use crate::cli::args::PersonaCommands;
        use crate::cli::standard::handle_persona_command;
        use crate::config::Config;

        let config = Config::load()?;
        let command = PersonaCommands::List;
        handle_persona_command(command, &config)
    }

    /// Show persona details
    pub fn show_persona(&self, id: &str) -> Result<()> {
        use crate::cli::args::PersonaCommands;
        use crate::cli::standard::handle_persona_command;
        use crate::config::Config;

        let config = Config::load()?;
        let command = PersonaCommands::Show { id: id.to_string() };
        handle_persona_command(command, &config)
    }

    /// Delete a persona
    pub fn delete_persona(&self, id: &str) -> Result<String> {
        use crate::cli::args::PersonaCommands;
        use crate::cli::standard::handle_persona_command;
        use crate::config::Config;

        let config = Config::load()?;
        let command = PersonaCommands::Delete { id: id.to_string() };
        handle_persona_command(command, &config)?;
        Ok(format!("Persona '{}' deleted successfully!", id))
    }
}

// Re-export for convenience
pub use crate::controller::MethodologyInfo;
