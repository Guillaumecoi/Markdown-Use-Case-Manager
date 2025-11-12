use anyhow::Result;

use crate::cli::interactive::initialization::Initialization;
use crate::cli::interactive::menu::{show_main_menu, MainMenuOption};
use crate::cli::interactive::settings::Settings;
use crate::cli::interactive::ui::UI;
use crate::cli::interactive::workflows::guided_create_use_case;
use crate::cli::standard::CliRunner;

/// Interactive session manager
pub struct InteractiveSession {
    runner: CliRunner,
}

impl InteractiveSession {
    /// Create a new interactive session
    pub fn new() -> Self {
        Self {
            runner: CliRunner::new(),
        }
    }

    /// Run the interactive session
    pub fn run(&mut self) -> Result<()> {
        // Check if project is initialized, if not offer to initialize
        if Initialization::check_and_initialize().is_err() {
            return Ok(());
        }

        UI::show_welcome()?;

        loop {
            match show_main_menu()? {
                MainMenuOption::CreateUseCase => {
                    if let Err(e) = guided_create_use_case(&mut self.runner) {
                        UI::show_error(&format!("Error creating use case: {}", e))?;
                    }
                }
                MainMenuOption::ConfigureSettings => {
                    if let Err(e) = Settings::configure(&mut self.runner) {
                        UI::show_error(&format!("Error configuring settings: {}", e))?;
                    }
                }
                MainMenuOption::ListUseCases => {
                    if let Err(e) = self.runner.list_use_cases() {
                        UI::show_error(&format!("Error listing use cases: {}", e))?;
                    }
                }
                MainMenuOption::ShowStatus => {
                    if let Err(e) = self.runner.show_status() {
                        UI::show_error(&format!("Error showing status: {}", e))?;
                    }
                }
                MainMenuOption::ShowLanguages => match CliRunner::show_languages() {
                    Ok(languages) => println!("\n{}", languages),
                    Err(e) => UI::show_error(&format!("Error showing languages: {}", e))?,
                },
                MainMenuOption::Exit => {
                    UI::show_goodbye()?;
                    break;
                }
            }

            // Pause before showing menu again
            UI::pause_for_input()?;
        }

        Ok(())
    }
}
