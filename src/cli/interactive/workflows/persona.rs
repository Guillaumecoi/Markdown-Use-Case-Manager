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
    ///
    /// Creates a minimal persona (id + name) with fields determined by config.
    /// Users can fill in additional fields later by editing the TOML/SQL record.
    pub fn create_persona() -> Result<()> {
        UI::show_section_header("Create Persona", "👤")?;

        let id = Text::new("Persona ID:")
            .with_help_message("Unique identifier (e.g., 'admin', 'customer', 'developer')")
            .prompt()?;

        let name = Text::new("Persona name:")
            .with_help_message("Display name (e.g., 'System Administrator', 'End User')")
            .prompt()?;

        // Create the persona
        let mut runner = InteractiveRunner::new();
        let result = runner.create_persona_interactive(id, name)?;

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
