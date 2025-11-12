//! # Use Case Workflow
//!
//! Interactive use case management for creating and managing use cases.
//! Provides guided workflows for use case operations.

use anyhow::Result;
use inquire::{Select, Text};

use crate::cli::interactive::{runner::InteractiveRunner, ui::UI};

/// Use case workflow handler
pub struct UseCaseWorkflow;

impl UseCaseWorkflow {
    /// Interactive use case creation workflow
    pub fn create_use_case() -> Result<()> {
        UI::show_section_header("Create Use Case", "📝")?;

        let title = Text::new("Use case title:")
            .with_help_message("A clear, descriptive title for the use case")
            .prompt()?;

        let category = Text::new("Category:")
            .with_help_message("Categorize this use case (e.g., 'authentication', 'data processing')")
            .prompt()?;

        let description = Text::new("Description (optional):")
            .with_help_message("Brief description of what this use case accomplishes")
            .prompt_skippable()?;

        // Get available methodologies for selection
        let mut runner = InteractiveRunner::new();
        let methodologies = runner.get_available_methodologies()?;

        let methodology_options = if methodologies.is_empty() {
            vec!["none".to_string()]
        } else {
            let mut options = vec!["none".to_string()];
            options.extend(methodologies.iter().map(|m| m.display_name.clone()));
            options
        };

        let selected_methodology = Select::new("Methodology (optional):", methodology_options)
            .with_help_message("Choose a methodology to structure this use case")
            .prompt()?;

        let methodology = if selected_methodology == "none" {
            None
        } else {
            // Find the methodology name from display name
            methodologies
                .iter()
                .find(|m| m.display_name == selected_methodology)
                .map(|m| m.name.clone())
        };

        // Create the use case
        let result = runner.create_use_case_interactive(
            title,
            category,
            description,
            methodology,
        )?;

        UI::show_success(&result)?;
        UI::pause_for_input()?;

        Ok(())
    }

    /// List all use cases
    pub fn list_use_cases() -> Result<()> {
        UI::show_section_header("Use Cases", "📋")?;

        let mut runner = InteractiveRunner::new();
        runner.list_use_cases()?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Show project status
    pub fn show_status() -> Result<()> {
        UI::show_section_header("Project Status", "📊")?;

        let mut runner = InteractiveRunner::new();
        runner.show_status()?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Interactive use case management menu
    pub fn manage_use_cases() -> Result<()> {
        UI::clear_screen()?;
        UI::show_section_header("Use Case Management", "📝")?;

        loop {
            let options = vec![
                "Create New Use Case",
                "List All Use Cases",
                "Show Project Status",
                "Back to Main Menu",
            ];

            let choice = Select::new("What would you like to do?", options).prompt()?;

            match choice {
                "Create New Use Case" => Self::create_use_case()?,
                "List All Use Cases" => Self::list_use_cases()?,
                "Show Project Status" => Self::show_status()?,
                "Back to Main Menu" => break,
                _ => {}
            }
        }

        Ok(())
    }
}