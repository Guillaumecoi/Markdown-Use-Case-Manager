//! # References Workflow
//!
//! Interactive management of use case references (dependencies, extensions, etc.).

use anyhow::Result;
use inquire::{Confirm, Select, Text};

use crate::cli::interactive::ui::UI;
use crate::controller::UseCaseController;

pub struct ReferencesWorkflow;

impl ReferencesWorkflow {
    /// Main use case references management entry point
    pub fn manage_references(use_case_id: &str) -> Result<()> {
        loop {
            let mut controller = UseCaseController::new()?;

            // Get and display current references
            let result = controller.list_references(use_case_id.to_string())?;

            UI::show_section_header(&format!("Manage References - {}", use_case_id), "🔗")?;
            println!("\n{}\n", result.message);

            // Menu options
            let options = vec!["Add Reference", "Remove Reference", "Back to Use Case Menu"];

            let selection = Select::new("What would you like to do?", options).prompt()?;

            match selection {
                "Add Reference" => {
                    if let Err(e) = Self::add_reference(use_case_id) {
                        UI::show_error(&format!("Failed to add reference: {}", e))?;
                    }
                }
                "Remove Reference" => {
                    if let Err(e) = Self::remove_reference(use_case_id) {
                        UI::show_error(&format!("Failed to remove reference: {}", e))?;
                    }
                }
                "Back to Use Case Menu" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Add a new reference interactively
    fn add_reference(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Add Reference", "➕")?;

        // Step 1: Select relationship type
        let relationship = Self::select_relationship_type()?;

        // Step 2: Enter target use case ID
        let target_id = Text::new("Target Use Case ID:")
            .with_help_message("e.g., UC-001")
            .prompt()?
            .trim()
            .to_string();

        if target_id.is_empty() {
            return Err(anyhow::anyhow!("Target ID cannot be empty"));
        }

        // Step 3: Optional description
        let description = Text::new("Description (optional):")
            .with_help_message("Describe the relationship")
            .prompt()?
            .trim()
            .to_string();

        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };

        // Add the reference
        let mut controller = UseCaseController::new()?;
        controller.add_reference(
            use_case_id.to_string(),
            target_id.clone(),
            relationship,
            description,
        )?;

        UI::show_success(&format!("Added reference to {}", target_id))?;
        Ok(())
    }

    /// Remove a reference interactively
    fn remove_reference(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Remove Reference", "❌")?;

        // Enter target use case ID to remove
        let target_id = Text::new("Target Use Case ID to remove:")
            .with_help_message("e.g., UC-001")
            .prompt()?
            .trim()
            .to_string();

        if target_id.is_empty() {
            return Err(anyhow::anyhow!("Target ID cannot be empty"));
        }

        // Confirm removal
        let confirm = Confirm::new(&format!("Remove reference to {}?", target_id))
            .with_default(false)
            .prompt()?;

        if confirm {
            let mut controller = UseCaseController::new()?;
            controller.remove_reference(use_case_id.to_string(), target_id.clone())?;
            UI::show_success(&format!("Removed reference to {}", target_id))?;
        } else {
            UI::show_info("Removal cancelled")?;
        }

        Ok(())
    }

    /// Select relationship type with descriptions
    fn select_relationship_type() -> Result<String> {
        let options = vec![
            (
                "depends_on",
                "Dependency - This use case depends on another",
            ),
            ("extends", "Extension - This use case extends another"),
            ("includes", "Inclusion - This use case includes another"),
            (
                "alternative_to",
                "Alternative - This use case is alternative to another",
            ),
        ];

        let display_options: Vec<String> =
            options.iter().map(|(_, desc)| desc.to_string()).collect();

        let selection = Select::new("Select relationship type:", display_options).prompt()?;

        // Find the relationship key
        let relationship = options
            .iter()
            .find(|(_, desc)| selection.starts_with(desc.split(" - ").next().unwrap()))
            .map(|(key, _)| key.to_string())
            .ok_or_else(|| anyhow::anyhow!("Invalid relationship type selected"))?;

        Ok(relationship)
    }
}
