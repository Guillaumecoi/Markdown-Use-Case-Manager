//! # Conditions Workflow
//!
//! Interactive management of use case preconditions and postconditions.
//! Provides guided workflows for adding, editing, removing, and reordering conditions.

use anyhow::Result;
use inquire::{Confirm, Select, Text};

use crate::cli::interactive::ui::UI;
use crate::controller::UseCaseController;

/// Workflow handler for use case preconditions and postconditions
pub struct ConditionsWorkflow;

impl ConditionsWorkflow {
    /// Main preconditions management entry point
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to manage preconditions for
    pub fn manage_preconditions(use_case_id: &str) -> Result<()> {
        loop {
            let mut controller = UseCaseController::new()?;

            // Get current preconditions
            let result = controller.list_preconditions(use_case_id.to_string())?;

            // Show section header
            UI::show_section_header(&format!("Preconditions - {}", use_case_id), "✅")?;

            // Parse and display preconditions
            let lines: Vec<&str> = result.message.lines().collect();
            let preconditions_text = lines.iter().skip(1).collect::<Vec<_>>();

            if preconditions_text.is_empty()
                || preconditions_text
                    .first()
                    .map(|s| s.contains("No preconditions"))
                    .unwrap_or(false)
            {
                println!("\n  No preconditions yet.\n");
            } else {
                for line in preconditions_text {
                    println!("  {}", line);
                }
                println!();
            }

            // Show menu options
            let options = vec![
                "Add Precondition",
                "Edit Precondition",
                "Remove Precondition",
                "Reorder Preconditions",
                "Clear All Preconditions",
                "Back to Use Case Menu",
            ];

            let choice = Select::new("What would you like to do?", options).prompt()?;

            match choice {
                "Add Precondition" => {
                    Self::add_precondition(use_case_id)?;
                }
                "Edit Precondition" => {
                    Self::edit_precondition(use_case_id)?;
                }
                "Remove Precondition" => {
                    Self::remove_precondition(use_case_id)?;
                }
                "Reorder Preconditions" => {
                    Self::reorder_preconditions(use_case_id)?;
                }
                "Clear All Preconditions" => {
                    Self::clear_preconditions(use_case_id)?;
                }
                "Back to Use Case Menu" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Main postconditions management entry point
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to manage postconditions for
    pub fn manage_postconditions(use_case_id: &str) -> Result<()> {
        loop {
            let mut controller = UseCaseController::new()?;

            // Get current postconditions
            let result = controller.list_postconditions(use_case_id.to_string())?;

            // Show section header
            UI::show_section_header(&format!("Postconditions - {}", use_case_id), "✔")?;

            // Parse and display postconditions
            let lines: Vec<&str> = result.message.lines().collect();
            let postconditions_text = lines.iter().skip(1).collect::<Vec<_>>();

            if postconditions_text.is_empty()
                || postconditions_text
                    .first()
                    .map(|s| s.contains("No postconditions"))
                    .unwrap_or(false)
            {
                println!("\n  No postconditions yet.\n");
            } else {
                for line in postconditions_text {
                    println!("  {}", line);
                }
                println!();
            }

            // Show menu options
            let options = vec![
                "Add Postcondition",
                "Edit Postcondition",
                "Remove Postcondition",
                "Reorder Postconditions",
                "Clear All Postconditions",
                "Back to Use Case Menu",
            ];

            let choice = Select::new("What would you like to do?", options).prompt()?;

            match choice {
                "Add Postcondition" => {
                    Self::add_postcondition(use_case_id)?;
                }
                "Edit Postcondition" => {
                    Self::edit_postcondition(use_case_id)?;
                }
                "Remove Postcondition" => {
                    Self::remove_postcondition(use_case_id)?;
                }
                "Reorder Postconditions" => {
                    Self::reorder_postconditions(use_case_id)?;
                }
                "Clear All Postconditions" => {
                    Self::clear_postconditions(use_case_id)?;
                }
                "Back to Use Case Menu" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Add precondition interactively
    fn add_precondition(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Add Precondition", "➕")?;

        let precondition = Text::new("Enter precondition:")
            .with_help_message(
                "Describe a condition that must be true before this use case executes",
            )
            .prompt()?;

        if precondition.trim().is_empty() {
            UI::show_error("Precondition cannot be empty")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        let mut controller = UseCaseController::new()?;
        let result = controller.add_precondition(use_case_id.to_string(), precondition)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Edit precondition interactively
    fn edit_precondition(use_case_id: &str) -> Result<()> {
        let mut controller = UseCaseController::new()?;

        // Get current preconditions
        let result = controller.list_preconditions(use_case_id.to_string())?;
        let lines: Vec<&str> = result.message.lines().collect();
        let preconditions: Vec<String> = lines
            .iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.contains("No preconditions"))
            .map(|s| s.to_string())
            .collect();

        if preconditions.is_empty() {
            UI::show_error("No preconditions to edit")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        UI::show_section_header("Edit Precondition", "✏️")?;

        // Select precondition to edit
        let selection =
            Select::new("Select precondition to edit:", preconditions.clone()).prompt()?;

        // Find index (extract number from "1. text")
        let index = preconditions
            .iter()
            .position(|p| p == &selection)
            .map(|i| i + 1)
            .ok_or_else(|| anyhow::anyhow!("Could not find selected precondition"))?;

        // Extract current text (remove number prefix)
        let current_text = selection
            .split_once(". ")
            .map(|(_, text)| text.trim())
            .unwrap_or(&selection);

        // Prompt for new text with current value
        let new_text = Text::new("Edit precondition:")
            .with_initial_value(current_text)
            .prompt()?;

        if new_text.trim().is_empty() {
            UI::show_error("Precondition cannot be empty")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Update precondition
        let result = controller.edit_precondition(use_case_id.to_string(), index, new_text)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Remove precondition interactively
    fn remove_precondition(use_case_id: &str) -> Result<()> {
        let mut controller = UseCaseController::new()?;

        // Get current preconditions
        let result = controller.list_preconditions(use_case_id.to_string())?;
        let lines: Vec<&str> = result.message.lines().collect();
        let preconditions: Vec<String> = lines
            .iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.contains("No preconditions"))
            .map(|s| s.to_string())
            .collect();

        if preconditions.is_empty() {
            UI::show_error("No preconditions to remove")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        UI::show_section_header("Remove Precondition", "🗑️")?;

        // Select precondition to remove
        let selection =
            Select::new("Select precondition to remove:", preconditions.clone()).prompt()?;

        // Find index
        let index = preconditions
            .iter()
            .position(|p| p == &selection)
            .map(|i| i + 1)
            .ok_or_else(|| anyhow::anyhow!("Could not find selected precondition"))?;

        // Confirm removal
        let confirm = Confirm::new(&format!(
            "Are you sure you want to remove this precondition? ({})",
            index
        ))
        .with_default(false)
        .prompt()?;

        if !confirm {
            UI::show_info("Removal cancelled")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Remove precondition
        let result = controller.remove_precondition(use_case_id.to_string(), index)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Reorder preconditions interactively
    fn reorder_preconditions(use_case_id: &str) -> Result<()> {
        let mut controller = UseCaseController::new()?;

        // Get current preconditions
        let result = controller.list_preconditions(use_case_id.to_string())?;
        let lines: Vec<&str> = result.message.lines().collect();
        let preconditions: Vec<String> = lines
            .iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.contains("No preconditions"))
            .map(|s| s.to_string())
            .collect();

        if preconditions.len() < 2 {
            UI::show_error("Need at least 2 preconditions to reorder")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        UI::show_section_header("Reorder Preconditions", "🔀")?;

        // Select precondition to move
        let from_selection =
            Select::new("Select precondition to move:", preconditions.clone()).prompt()?;

        let from_index = preconditions
            .iter()
            .position(|p| p == &from_selection)
            .map(|i| i + 1)
            .ok_or_else(|| anyhow::anyhow!("Could not find selected precondition"))?;

        // Create position options (1 to N)
        let position_options: Vec<String> = (1..=preconditions.len())
            .map(|i| format!("Position {}", i))
            .collect();

        let to_selection = Select::new("Move to position:", position_options).prompt()?;

        let to_index = to_selection
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or_else(|| anyhow::anyhow!("Invalid position"))?;

        if from_index == to_index {
            UI::show_info("No change in position")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Reorder precondition
        let result =
            controller.reorder_preconditions(use_case_id.to_string(), from_index, to_index)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Clear all preconditions
    fn clear_preconditions(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Clear All Preconditions", "⚠️")?;

        let confirm = Confirm::new(
            "Are you sure you want to clear ALL preconditions? This cannot be undone.",
        )
        .with_default(false)
        .prompt()?;

        if !confirm {
            UI::show_info("Operation cancelled")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        let mut controller = UseCaseController::new()?;
        let result = controller.clear_preconditions(use_case_id.to_string())?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Add postcondition interactively
    fn add_postcondition(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Add Postcondition", "➕")?;

        let postcondition = Text::new("Enter postcondition:")
            .with_help_message(
                "Describe a condition that must be true after this use case executes",
            )
            .prompt()?;

        if postcondition.trim().is_empty() {
            UI::show_error("Postcondition cannot be empty")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        let mut controller = UseCaseController::new()?;
        let result = controller.add_postcondition(use_case_id.to_string(), postcondition)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Edit postcondition interactively
    fn edit_postcondition(use_case_id: &str) -> Result<()> {
        let mut controller = UseCaseController::new()?;

        // Get current postconditions
        let result = controller.list_postconditions(use_case_id.to_string())?;
        let lines: Vec<&str> = result.message.lines().collect();
        let postconditions: Vec<String> = lines
            .iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.contains("No postconditions"))
            .map(|s| s.to_string())
            .collect();

        if postconditions.is_empty() {
            UI::show_error("No postconditions to edit")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        UI::show_section_header("Edit Postcondition", "✏️")?;

        // Select postcondition to edit
        let selection =
            Select::new("Select postcondition to edit:", postconditions.clone()).prompt()?;

        // Find index
        let index = postconditions
            .iter()
            .position(|p| p == &selection)
            .map(|i| i + 1)
            .ok_or_else(|| anyhow::anyhow!("Could not find selected postcondition"))?;

        // Extract current text
        let current_text = selection
            .split_once(". ")
            .map(|(_, text)| text.trim())
            .unwrap_or(&selection);

        // Prompt for new text
        let new_text = Text::new("Edit postcondition:")
            .with_initial_value(current_text)
            .prompt()?;

        if new_text.trim().is_empty() {
            UI::show_error("Postcondition cannot be empty")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Update postcondition
        let result = controller.edit_postcondition(use_case_id.to_string(), index, new_text)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Remove postcondition interactively
    fn remove_postcondition(use_case_id: &str) -> Result<()> {
        let mut controller = UseCaseController::new()?;

        // Get current postconditions
        let result = controller.list_postconditions(use_case_id.to_string())?;
        let lines: Vec<&str> = result.message.lines().collect();
        let postconditions: Vec<String> = lines
            .iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.contains("No postconditions"))
            .map(|s| s.to_string())
            .collect();

        if postconditions.is_empty() {
            UI::show_error("No postconditions to remove")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        UI::show_section_header("Remove Postcondition", "🗑️")?;

        // Select postcondition to remove
        let selection =
            Select::new("Select postcondition to remove:", postconditions.clone()).prompt()?;

        // Find index
        let index = postconditions
            .iter()
            .position(|p| p == &selection)
            .map(|i| i + 1)
            .ok_or_else(|| anyhow::anyhow!("Could not find selected postcondition"))?;

        // Confirm removal
        let confirm = Confirm::new(&format!(
            "Are you sure you want to remove this postcondition? ({})",
            index
        ))
        .with_default(false)
        .prompt()?;

        if !confirm {
            UI::show_info("Removal cancelled")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Remove postcondition
        let result = controller.remove_postcondition(use_case_id.to_string(), index)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Reorder postconditions interactively
    fn reorder_postconditions(use_case_id: &str) -> Result<()> {
        let mut controller = UseCaseController::new()?;

        // Get current postconditions
        let result = controller.list_postconditions(use_case_id.to_string())?;
        let lines: Vec<&str> = result.message.lines().collect();
        let postconditions: Vec<String> = lines
            .iter()
            .skip(1)
            .filter(|line| !line.trim().is_empty() && !line.contains("No postconditions"))
            .map(|s| s.to_string())
            .collect();

        if postconditions.len() < 2 {
            UI::show_error("Need at least 2 postconditions to reorder")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        UI::show_section_header("Reorder Postconditions", "🔀")?;

        // Select postcondition to move
        let from_selection =
            Select::new("Select postcondition to move:", postconditions.clone()).prompt()?;

        let from_index = postconditions
            .iter()
            .position(|p| p == &from_selection)
            .map(|i| i + 1)
            .ok_or_else(|| anyhow::anyhow!("Could not find selected postcondition"))?;

        // Create position options
        let position_options: Vec<String> = (1..=postconditions.len())
            .map(|i| format!("Position {}", i))
            .collect();

        let to_selection = Select::new("Move to position:", position_options).prompt()?;

        let to_index = to_selection
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or_else(|| anyhow::anyhow!("Invalid position"))?;

        if from_index == to_index {
            UI::show_info("No change in position")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Reorder postcondition
        let result =
            controller.reorder_postconditions(use_case_id.to_string(), from_index, to_index)?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Clear all postconditions
    fn clear_postconditions(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Clear All Postconditions", "⚠️")?;

        let confirm = Confirm::new(
            "Are you sure you want to clear ALL postconditions? This cannot be undone.",
        )
        .with_default(false)
        .prompt()?;

        if !confirm {
            UI::show_info("Operation cancelled")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        let mut controller = UseCaseController::new()?;
        let result = controller.clear_postconditions(use_case_id.to_string())?;

        if result.success {
            UI::show_success(&result.message)?;
        } else {
            UI::show_error(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }
}
