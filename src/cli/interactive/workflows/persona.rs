//! # Persona Workflow
//!
//! Interactive persona management for creating and managing personas.
//! Provides guided workflows for persona operations.

use anyhow::Result;
use inquire::{Select, Text};

use crate::cli::interactive::{runner::InteractiveRunner, ui::UI};

/// Persona workflow handler
pub struct PersonaWorkflow;

impl PersonaWorkflow {
    /// Interactive persona creation workflow
    pub fn create_persona() -> Result<()> {
        UI::show_section_header("Create Persona", "👤")?;

        let id = Text::new("Persona ID:")
            .with_help_message("Unique identifier (e.g., 'admin', 'customer', 'developer')")
            .prompt()?;

        let name = Text::new("Persona name:")
            .with_help_message("Display name (e.g., 'System Administrator', 'End User')")
            .prompt()?;

        let description = Text::new("Description:")
            .with_help_message("Brief description of this persona")
            .prompt()?;

        let goal = Text::new("Primary goal:")
            .with_help_message("What is this persona trying to achieve?")
            .prompt()?;

        let context = Text::new("Context (optional):")
            .with_help_message("Background information, work environment, etc.")
            .prompt_skippable()?;

        let tech_level_input = Text::new("Technical proficiency (1-5, optional):")
            .with_help_message("1=Beginner, 2=Basic, 3=Intermediate, 4=Advanced, 5=Expert")
            .prompt_skippable()?;

        let tech_level = if let Some(level_str) = tech_level_input {
            match level_str.trim().parse::<u8>() {
                Ok(level) if level >= 1 && level <= 5 => Some(level),
                _ => {
                    UI::show_error("Invalid tech level. Must be between 1 and 5. Skipping.")?;
                    None
                }
            }
        } else {
            None
        };

        let usage_frequency = Text::new("Usage frequency (optional):")
            .with_help_message("e.g., 'daily', 'weekly', 'occasional'")
            .prompt_skippable()?;

        // Create the persona
        let mut runner = InteractiveRunner::new();
        let result = runner.create_persona_interactive(
            id,
            name,
            description,
            goal,
            context,
            tech_level,
            usage_frequency,
        )?;

        UI::show_success(&result)?;
        UI::pause_for_input()?;

        Ok(())
    }

    /// List all personas
    pub fn list_personas() -> Result<()> {
        UI::show_section_header("Personas", "👥")?;

        let runner = InteractiveRunner::new();
        runner.list_personas()?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Show persona details
    pub fn show_persona() -> Result<()> {
        UI::show_section_header("Show Persona", "🔍")?;

        let id = Text::new("Persona ID:")
            .with_help_message("Enter the persona ID to view")
            .prompt()?;

        let runner = InteractiveRunner::new();
        runner.show_persona(&id)?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Delete a persona
    pub fn delete_persona() -> Result<()> {
        UI::show_section_header("Delete Persona", "🗑️")?;

        let id = Text::new("Persona ID:")
            .with_help_message("Enter the persona ID to delete")
            .prompt()?;

        // Confirm deletion
        let confirm = Select::new(
            &format!("Are you sure you want to delete persona '{}'?", id),
            vec!["No", "Yes"],
        )
        .prompt()?;

        if confirm == "Yes" {
            let runner = InteractiveRunner::new();
            let result = runner.delete_persona(&id)?;
            UI::show_success(&result)?;
        } else {
            println!("\n✓ Deletion cancelled.");
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Interactive persona management menu
    pub fn manage_personas() -> Result<()> {
        UI::clear_screen()?;
        UI::show_section_header("Persona Management", "👤")?;

        loop {
            let options = vec![
                "Create New Persona",
                "List All Personas",
                "Show Persona Details",
                "Delete Persona",
                "Back to Main Menu",
            ];

            let choice = Select::new("What would you like to do?", options).prompt()?;

            match choice {
                "Create New Persona" => Self::create_persona()?,
                "List All Personas" => Self::list_personas()?,
                "Show Persona Details" => Self::show_persona()?,
                "Delete Persona" => Self::delete_persona()?,
                "Back to Main Menu" => break,
                _ => {}
            }
        }

        Ok(())
    }
}
