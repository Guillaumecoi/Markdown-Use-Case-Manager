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
use crate::core::{FieldCollection, MethodologyFieldCollector};

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

    /// Sanitize an optional string input by trimming whitespace and filtering empty strings.
    ///
    /// Returns None if the input is None or contains only whitespace.
    /// Returns Some(trimmed_string) if the input contains non-whitespace characters.
    fn sanitize_optional_string(input: Option<String>) -> Option<String> {
        input
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
    }

    /// Sanitize a string input by trimming whitespace.
    ///
    /// Returns None if the input contains only whitespace, Some(trimmed_string) otherwise.
    fn sanitize_string(input: String) -> Option<String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
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
    /// Get all available methodologies from source templates (for initialization)
    pub fn get_available_methodologies(&self) -> Result<Vec<MethodologyInfo>> {
        ProjectController::get_available_methodologies()
    }

    /// Collect methodology-specific field definitions for the given views
    pub fn collect_methodology_fields(
        &self,
        views: &[(String, String)],
    ) -> Result<FieldCollection> {
        let collector = MethodologyFieldCollector::new()?;
        collector.collect_fields_for_views(views)
    }

    /// Get installed/configured methodologies in the project (for creating use cases)
    pub fn get_installed_methodologies(&self) -> Result<Vec<MethodologyInfo>> {
        ProjectController::get_installed_methodologies()
    }

    /// Get available levels for a specific methodology
    pub fn get_methodology_levels(
        &self,
        methodology_name: &str,
    ) -> Result<Vec<crate::core::DocumentationLevel>> {
        crate::controller::ProjectController::get_methodology_levels(methodology_name)
    }

    /// Initialize project for interactive mode.
    pub fn initialize_project(
        &mut self,
        language: Option<String>,
        methodologies: Vec<String>,
        storage: String,
        use_case_dir: String,
        test_dir: String,
        persona_dir: String,
        data_dir: String,
    ) -> Result<String> {
        // Sanitize inputs: trim whitespace and filter out empty strings
        let sanitized_language = Self::sanitize_optional_string(language);
        let sanitized_methodologies: Vec<String> = methodologies
            .into_iter()
            .filter_map(Self::sanitize_string)
            .collect();

        // Use first methodology as default, or "feature" if none provided
        let default_methodology = sanitized_methodologies
            .first()
            .cloned()
            .unwrap_or_else(|| "feature".to_string());

        // Call the complete initialization method
        let result = crate::controller::ProjectController::init_project(
            sanitized_language,
            Some(sanitized_methodologies),
            Some(storage),
            Some(default_methodology),
            Some(use_case_dir),
            Some(test_dir),
            Some(persona_dir),
            Some(data_dir),
        )?;

        Ok(result.message)
    }

    /// Create a multi-view use case
    #[allow(dead_code)]
    pub fn create_use_case_with_views(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        views: Vec<(String, String)>, // Vec of (methodology, level) pairs
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;

        // Format views as "methodology:level,methodology:level"
        let views_string = views
            .iter()
            .map(|(methodology, level)| format!("{}:{}", methodology, level))
            .collect::<Vec<_>>()
            .join(",");

        let result = controller.create_use_case(
            title,
            category,
            description,
            None,
            Some(views_string),
            None,
            None,
        )?;
        Ok(result.message)
    }

    /// Create use case with multiple views and additional fields
    pub fn create_use_case_with_views_and_fields(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        priority: String,
        views: Vec<(String, String)>, // Vec of (methodology, level) pairs
        extra_fields: std::collections::HashMap<String, String>,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;

        // Format views as "methodology:level,methodology:level"
        let views_string = views
            .iter()
            .map(|(methodology, level)| format!("{}:{}", methodology, level))
            .collect::<Vec<_>>()
            .join(",");

        let result = controller.create_use_case(
            title,
            category,
            description,
            None,
            Some(views_string),
            Some(priority),
            Some(extra_fields),
        )?;
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
    pub fn create_persona_interactive(&mut self, id: String, name: String) -> Result<String> {
        use crate::cli::args::PersonaCommands;
        use crate::cli::standard::handle_persona_command;
        use crate::config::Config;

        let config = Config::load()?;
        let command = PersonaCommands::Create { id, name };

        handle_persona_command(command, &config)?;
        Ok("Persona created successfully!".to_string())
    }

    /// Create a persona with additional fields
    // pub fn create_persona_with_fields(
    //     &mut self,
    //     id: String,
    //     name: String,
    //     extra_fields: std::collections::HashMap<String, String>,
    // ) -> Result<String> {
    //     use crate::cli::standard::create_persona_with_fields;
    //     use crate::config::Config;

    //     let config = Config::load()?;
    //     create_persona_with_fields(&config, id.clone(), name.clone(), extra_fields)?;
    //     Ok(format!("Created persona with custom fields: {} ({})", name, id))
    // }

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

    // ========== Use Case Editing Methods ==========

    /// Get list of all use case IDs for selection
    pub fn get_use_case_ids(&mut self) -> Result<Vec<String>> {
        let controller = self.ensure_use_case_controller()?;
        let use_cases = controller.get_all_use_cases()?;
        Ok(use_cases.iter().map(|uc| uc.id.clone()).collect())
    }

    /// Get use case details for editing
    pub fn get_use_case_details(&mut self, use_case_id: &str) -> Result<crate::core::UseCase> {
        let controller = self.ensure_use_case_controller()?;
        let use_case = controller.get_use_case(use_case_id)?;
        // Clone to return owned value
        Ok(use_case.clone())
    }

    /// Update basic use case fields
    pub fn update_use_case(
        &mut self,
        use_case_id: String,
        title: Option<String>,
        category: Option<String>,
        description: Option<String>,
        priority: Option<String>,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.update_use_case(
            use_case_id.clone(),
            title,
            category,
            description,
            priority,
        )?;
        Ok(result.message)
    }

    /// Update methodology-specific fields for a use case
    pub fn update_methodology_fields(
        &mut self,
        use_case_id: &str,
        methodology: &str,
        fields: std::collections::HashMap<String, String>,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.update_use_case_methodology_fields(
            use_case_id.to_string(),
            methodology.to_string(),
            fields,
        )?;
        Ok(result.message)
    }

    /// Add a new view to a use case
    pub fn add_view_to_use_case(
        &mut self,
        use_case_id: &str,
        methodology: &str,
        level: &str,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.add_view(
            use_case_id.to_string(),
            methodology.to_string(),
            level.to_string(),
        )?;
        Ok(result.message)
    }

    /// Remove a view from a use case
    pub fn remove_view_from_use_case(
        &mut self,
        use_case_id: &str,
        methodology: &str,
    ) -> Result<String> {
        let controller = self.ensure_use_case_controller()?;
        let result = controller.remove_view(use_case_id.to_string(), methodology.to_string())?;
        Ok(result.message)
    }

    /// Get current methodology field values for a use case
    pub fn get_methodology_field_values(
        &mut self,
        use_case_id: &str,
        methodology: &str,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>> {
        let use_case = self.get_use_case_details(use_case_id)?;
        Ok(use_case
            .methodology_fields
            .get(methodology)
            .cloned()
            .unwrap_or_default())
    }
}

// Re-export for convenience
pub use crate::controller::MethodologyInfo;
