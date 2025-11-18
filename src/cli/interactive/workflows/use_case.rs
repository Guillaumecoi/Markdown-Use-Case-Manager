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
        UI::show_section_header("Create Multi-View Use Case", "🔄")?;

        let mut runner = InteractiveRunner::new();
        let methodologies = runner.get_installed_methodologies()?;

        if methodologies.is_empty() {
            UI::show_error(
                "No methodologies available. Please configure methodologies in your project.",
            )?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Step 1: Prompt for title and category first
        UI::show_info("\n📋 Required Fields")?;

        let title = Text::new("Title:")
            .with_help_message("A clear, descriptive title for the use case")
            .prompt()?;

        let category = Text::new("Category:")
            .with_help_message("Group this use case (e.g., 'authentication', 'data-processing')")
            .prompt()?;

        let description = Text::new("Description:")
            .with_help_message("Brief description of what this use case accomplishes")
            .prompt_skippable()?;

        // Step 2: Collect multiple views
        UI::show_section_header("Select Views", "👁️")?;
        UI::show_info(
            "Add multiple methodology views. Each view will generate a separate markdown file.",
        )?;

        let mut views: Vec<(String, String)> = Vec::new();

        loop {
            // Display methodologies with their descriptions
            let methodology_options: Vec<String> = methodologies
                .iter()
                .map(|m| format!("{} - {}", m.display_name, m.description))
                .collect();

            let selected_idx = Select::new(
                &format!("Select methodology (view #{}):", views.len() + 1),
                methodology_options.clone(),
            )
            .with_help_message("Choose how you want to structure this view")
            .prompt()?;

            // Find the selected methodology
            let selected_methodology = &methodologies[methodologies
                .iter()
                .position(|m| format!("{} - {}", m.display_name, m.description) == selected_idx)
                .context("Selected methodology not found")?];

            let methodology_name = selected_methodology.name.clone();

            // Get available levels for this methodology
            let available_levels = runner.get_methodology_levels(&methodology_name)?;

            if available_levels.is_empty() {
                UI::show_error(&format!(
                    "No levels available for methodology '{}'",
                    methodology_name
                ))?;
                continue;
            }

            // Display levels with their descriptions
            let level_options: Vec<String> = available_levels
                .iter()
                .map(|level| {
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
                .with_help_message("Choose the detail level for this view")
                .prompt()?;

            // Extract just the level name and convert to lowercase
            let level = selected_level_display
                .split(" - ")
                .next()
                .context("Failed to parse level name")?
                .to_lowercase();

            views.push((methodology_name.clone(), level.clone()));

            UI::show_success(&format!("✓ Added view: {}:{}", methodology_name, level))?;

            // Ask if user wants to add another view
            let add_another = Confirm::new("Add another view?")
                .with_default(false)
                .with_help_message("Each view will generate a separate markdown file")
                .prompt()?;

            if !add_another {
                break;
            }
        }

        if views.is_empty() {
            UI::show_error("No views selected. Use case creation cancelled.")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Always use interactive form for additional fields
        Self::fill_use_case_form(&mut runner, title, category, description, views)?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Interactive form for filling use case fields
    fn fill_use_case_form(
        runner: &mut InteractiveRunner,
        title: String,
        category: String,
        description: Option<String>,
        views: Vec<(String, String)>,
    ) -> Result<()> {
        UI::show_section_header("Additional Fields", "📝")?;

        // Description (if not already provided)
        let final_description = if description.is_some() {
            description
        } else {
            Text::new("Description:")
                .with_help_message("Brief description of what this use case accomplishes")
                .prompt_skippable()?
        };

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

        let result = runner.create_use_case_with_views_and_fields(
            title,
            category,
            final_description,
            views.clone(),
            extra_fields,
        )?;

        UI::show_success(&result)?;

        // Show summary of created views
        UI::show_info("\n📄 Generated files:")?;
        for (methodology, level) in &views {
            println!("   • {}-{}.md", methodology, level);
        }

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
