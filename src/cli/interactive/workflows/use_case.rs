//! # Use Case Workflow
//!
//! Interactive use case management for creating and managing use cases.
//! Provides guided workflows for use case operations.

use anyhow::{Context, Result};
use inquire::{Confirm, Select, Text};
use std::collections::HashMap;

use crate::cli::interactive::{runner::InteractiveRunner, ui::UI};

/// Use case workflow handler
pub struct UseCaseWorkflow;

impl UseCaseWorkflow {
    /// Interactive use case creation workflow
    pub fn create_use_case() -> Result<()> {
        UI::show_section_header("Create Use Case", "📝")?;

        // Step 1: Select methodology from installed/configured ones in the project
        let mut runner = InteractiveRunner::new();
        let methodologies = runner.get_installed_methodologies()?;

        if methodologies.is_empty() {
            UI::show_error(
                "No methodologies available. Please configure methodologies in your project.",
            )?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Display methodologies with their descriptions
        let methodology_options: Vec<String> = methodologies
            .iter()
            .map(|m| format!("{} - {}", m.display_name, m.description))
            .collect();

        let selected_idx = Select::new("Select methodology:", methodology_options)
            .with_help_message("Choose how you want to structure this use case")
            .prompt()?;

        // Find the selected methodology
        let selected_methodology = &methodologies[methodologies
            .iter()
            .position(|m| format!("{} - {}", m.display_name, m.description) == selected_idx)
            .context("Selected methodology not found")?];

        let methodology_name = selected_methodology.name.clone();

        // Step 2: Get available levels for this methodology
        let available_levels = runner.get_methodology_levels(&methodology_name)?;

        if available_levels.is_empty() {
            UI::show_error(&format!(
                "No levels available for methodology '{}'",
                methodology_name
            ))?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Show info about available levels
        UI::show_info(&format!(
            "\n💡 Methodology '{}' has {} level(s) available",
            methodology_name,
            available_levels.len()
        ))?;

        // Display levels with their descriptions
        let level_options: Vec<String> = available_levels
            .iter()
            .map(|level| {
                // Capitalize level name for display
                let display_name = level
                    .name
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().next().unwrap()
                        } else {
                            c
                        }
                    })
                    .collect::<String>();
                format!("{} - {}", display_name, level.description)
            })
            .collect();

        let selected_level_display = Select::new("Select level:", level_options)
            .with_help_message("Choose the detail level for your use case documentation")
            .prompt()?;

        // Extract just the level name (before " - ") and convert to lowercase
        let level = selected_level_display
            .split(" - ")
            .next()
            .context("Failed to parse level name")?
            .to_lowercase();

        // Step 3: Prompt for required fields
        UI::show_info("\n📋 Required Fields")?;

        let title = Text::new("Title:")
            .with_help_message("A clear, descriptive title for the use case")
            .prompt()?;

        let category = Text::new("Category:")
            .with_help_message("Group this use case (e.g., 'authentication', 'data-processing')")
            .prompt()?;

        // Step 4: Ask if user wants to fill in form or edit file directly
        let use_form = Confirm::new("Fill in additional fields using interactive form?")
            .with_default(true)
            .with_help_message("No = create use case and edit TOML/SQL file directly")
            .prompt()?;

        if use_form {
            // Step 5: Interactive form for additional fields
            Self::fill_use_case_form(&mut runner, title, category, methodology_name, level)?;
        } else {
            // Create with minimal info and let user edit file
            let result = runner.create_use_case_interactive(
                title.clone(),
                category,
                None,
                Some(methodology_name.clone()),
            )?;

            UI::show_success(&result)?;
            UI::show_info(&format!(
                "\n💡 Edit the TOML file in the data directory to add more fields.\n   Level: {}",
                level
            ))?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Interactive form for filling use case fields
    fn fill_use_case_form(
        runner: &mut InteractiveRunner,
        title: String,
        category: String,
        methodology: String,
        _level: String,
    ) -> Result<()> {
        UI::show_section_header("Additional Fields", "📝")?;

        // Description
        let description = Text::new("Description:")
            .with_help_message("Brief description of what this use case accomplishes")
            .prompt_skippable()?;

        // Priority
        let priority_options = vec!["Low", "Medium", "High", "Critical"];
        let priority = Select::new("Priority:", priority_options)
            .with_help_message("Priority level for this use case")
            .prompt()?;

        // Status
        let status_options = vec!["Draft", "In Review", "Approved", "Implemented"];
        let status = Select::new("Status:", status_options)
            .with_help_message("Current status of this use case")
            .prompt()?;

        // Author (optional)
        let author = Text::new("Author (optional):")
            .with_help_message("Person who created this use case")
            .prompt_skippable()?;

        // Reviewer (optional)
        let reviewer = Text::new("Reviewer (optional):")
            .with_help_message("Person responsible for reviewing this use case")
            .prompt_skippable()?;

        // Create the use case with additional fields
        let mut extra_fields = HashMap::new();
        extra_fields.insert("priority".to_string(), priority.to_lowercase());
        extra_fields.insert(
            "status".to_string(),
            status.to_lowercase().replace(" ", "_"),
        );

        if let Some(auth) = author {
            if !auth.is_empty() {
                extra_fields.insert("author".to_string(), auth);
            }
        }

        if let Some(rev) = reviewer {
            if !rev.is_empty() {
                extra_fields.insert("reviewer".to_string(), rev);
            }
        }

        let result = runner.create_use_case_with_fields(
            title,
            category,
            description,
            Some(methodology),
            extra_fields,
        )?;

        UI::show_success(&result)?;
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
