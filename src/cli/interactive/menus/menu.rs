//! # Interactive Menu System
//!
//! Menu navigation and selection logic for interactive CLI mode.
//! Provides user-friendly menu interfaces for workflow selection.

use anyhow::Result;

use crate::cli::interactive::ui::UI;
use crate::cli::interactive::workflows::initialization::Initialization;
use crate::cli::interactive::menus::settings::Settings;
use crate::cli::interactive::workflows::use_case::UseCaseWorkflow;
use crate::cli::standard::CliRunner;

use super::common::{MenuOption, display_menu};

/// Run the interactive session main loop
pub fn run_interactive_session() -> Result<()> {
    let mut runner = CliRunner::new();

    // Check if project is initialized, if not offer to initialize
    if Initialization::check_and_initialize().is_err() {
        return Ok(());
    }

    UI::show_welcome()?;

    loop {
        let options = create_main_menu_options();

        if display_menu("What would you like to do?", &options, &mut runner)? {
            return Ok(()); // Exit the session
        }

        // Pause before showing menu again
        UI::pause_for_input()?;
    }
}

/// Create the main menu options
fn create_main_menu_options() -> Vec<MenuOption<CliRunner>> {
    vec![
        MenuOption::new("📝 Create a new use case", |_| {
            if let Err(e) = UseCaseWorkflow::create_use_case() {
                UI::show_error(&format!("Error creating use case: {}", e))?;
            }
            Ok(false) // Don't exit
        }),
        MenuOption::new("⚙️  Configure settings", |_| {
            if let Err(e) = Settings::configure() {
                UI::show_error(&format!("Error configuring settings: {}", e))?;
            }
            Ok(false) // Don't exit
        }),
        MenuOption::new("📋 List all use cases", |runner: &mut CliRunner| {
            if let Err(e) = runner.list_use_cases() {
                UI::show_error(&format!("Error listing use cases: {}", e))?;
            }
            Ok(false) // Don't exit
        }),
        MenuOption::new("📊 Show project status", |runner: &mut CliRunner| {
            if let Err(e) = runner.show_status() {
                UI::show_error(&format!("Error showing status: {}", e))?;
            }
            Ok(false) // Don't exit
        }),
        MenuOption::new("🗣️  Show available languages", |_| {
            match CliRunner::show_languages() {
                Ok(languages) => println!("\n{}", languages),
                Err(e) => UI::show_error(&format!("Error showing languages: {}", e))?,
            }
            Ok(false) // Don't exit
        }),
        MenuOption::new("🚪 Exit", |_| {
            UI::show_goodbye()?;
            Ok(true) // Exit the session
        }),
    ]
}
