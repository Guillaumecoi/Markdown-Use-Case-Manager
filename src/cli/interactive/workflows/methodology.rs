//! # Methodology Workflow
//!
//! Interactive methodology management for project configuration.
//! Handles adding, removing, and managing methodologies used in the project.

use anyhow::Result;
use inquire::{Confirm, MultiSelect};

use crate::cli::interactive::ui::UI;
use crate::config::Config;

/// Methodology workflow handler
pub struct MethodologyWorkflow;

impl MethodologyWorkflow {
    /// Add methodologies to project
    pub fn add_methodologies() -> Result<()> {
        UI::show_section_header("Add Methodologies", "➕")?;

        let available = crate::controller::ProjectController::get_available_methodologies()?;

        if available.is_empty() {
            UI::show_warning("No additional methodologies available.")?;
            return Ok(());
        }

        let config = Config::load()?;
        let current_methodologies = config.templates.methodologies.clone();

        // Filter out already configured methodologies
        let new_methodologies: Vec<String> = available
            .into_iter()
            .filter(|m| !current_methodologies.contains(&m.name))
            .map(|m| m.name)
            .collect();

        if new_methodologies.is_empty() {
            UI::show_warning("All available methodologies are already configured.")?;
            return Ok(());
        }

        let selected = MultiSelect::new("Select methodologies to add:", new_methodologies)
            .with_help_message("Use Space to select/deselect, Enter to confirm")
            .prompt()?;

        if selected.is_empty() {
            UI::show_warning("No methodologies selected.")?;
            return Ok(());
        }

        // Update config
        let mut updated_config = config;
        for methodology in selected {
            if !updated_config.templates.methodologies.contains(&methodology) {
                updated_config.templates.methodologies.push(methodology);
            }
        }

        updated_config.save_in_dir(".")?;
        UI::show_success("✅ Methodologies added successfully!")?;

        Ok(())
    }

    /// Remove methodologies from project
    pub fn remove_methodologies() -> Result<()> {
        UI::show_section_header("Remove Methodologies", "➖")?;

        let config = Config::load()?;
        let methodologies = config.templates.methodologies.clone();

        if methodologies.is_empty() {
            UI::show_warning("No methodologies configured to remove.")?;
            return Ok(());
        }

        let selected = MultiSelect::new("Select methodologies to remove:", methodologies)
            .with_help_message("Use Space to select/deselect, Enter to confirm")
            .prompt()?;

        if selected.is_empty() {
            UI::show_warning("No methodologies selected for removal.")?;
            return Ok(());
        }

        // Confirm removal
        let confirm = Confirm::new(&format!(
            "Remove {} methodologies? This action cannot be undone.",
            selected.len()
        ))
        .with_default(false)
        .prompt()?;

        if !confirm {
            UI::show_warning("Removal cancelled.")?;
            return Ok(());
        }

        // Update config
        let mut updated_config = config;
        updated_config.templates.methodologies.retain(|m| !selected.contains(m));

        updated_config.save_in_dir(".")?;
        UI::show_success("✅ Methodologies removed successfully!")?;

        Ok(())
    }
}