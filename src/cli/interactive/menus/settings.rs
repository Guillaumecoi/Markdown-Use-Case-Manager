//! # Interactive Settings Configuration
//!
//! Menu-driven settings configuration for interactive CLI mode.
//! Coordinates between specialized workflow modules for different configuration domains.

use anyhow::Result;

use crate::cli::interactive::ui::UI;
use crate::config::Config;

use crate::cli::interactive::workflows::config::ConfigWorkflow;
use crate::cli::interactive::workflows::methodology::MethodologyWorkflow;
use crate::cli::interactive::workflows::use_case::UseCaseWorkflow;

use super::common::{display_menu, MenuOption};

/// Handle settings configuration
pub struct Settings;

impl Settings {
    /// Interactive settings configuration menu
    pub fn configure() -> Result<()> {
        UI::clear_screen()?;
        UI::show_section_header("Configuration Settings", "⚙️")?;

        // Load current config
        let mut config = Config::load()?;

        loop {
            let options = Self::create_settings_options();

            if display_menu("What would you like to configure?", &options, &mut config)? {
                break; // Exit the settings menu
            }
        }

        Ok(())
    }

    /// Create the settings menu options
    fn create_settings_options() -> Vec<MenuOption<Config>> {
        vec![
            MenuOption::new("Project Information", |config| {
                ConfigWorkflow::configure_project_info(config)?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Directory Settings", |config| {
                ConfigWorkflow::configure_directories(config)?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Generation Settings", |config| {
                ConfigWorkflow::configure_generation(config)?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Metadata Configuration", |config| {
                ConfigWorkflow::configure_metadata(config)?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Change Programming Language", |_| {
                ConfigWorkflow::change_programming_language()?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Add Methodologies", |_| {
                MethodologyWorkflow::add_methodologies()?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Remove Methodologies", |_| {
                MethodologyWorkflow::remove_methodologies()?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Use Case Management", |_| {
                UseCaseWorkflow::manage_use_cases()?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("View Current Config", |config| {
                ConfigWorkflow::view_config(config)?;
                Ok(false) // Continue menu
            }),
            MenuOption::new("Save & Exit", |config| {
                ConfigWorkflow::save_config(config)?;
                UI::pause_for_input()?;
                Ok(true) // Exit the settings menu
            }),
        ]
    }
}
