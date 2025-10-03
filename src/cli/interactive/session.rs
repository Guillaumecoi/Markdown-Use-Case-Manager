use anyhow::Result;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::stdout;

use crate::cli::runner::CliRunner;
use crate::cli::interactive::menu::{show_main_menu, MainMenuOption, guided_create_use_case, guided_add_scenario, guided_update_scenario_status};

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
        self.show_welcome()?;

        loop {
            match show_main_menu()? {
                MainMenuOption::CreateUseCase => {
                    if let Err(e) = guided_create_use_case(&mut self.runner) {
                        self.show_error(&format!("Error creating use case: {}", e))?;
                    }
                }
                MainMenuOption::AddScenario => {
                    if let Err(e) = guided_add_scenario(&mut self.runner) {
                        self.show_error(&format!("Error adding scenario: {}", e))?;
                    }
                }
                MainMenuOption::UpdateScenarioStatus => {
                    if let Err(e) = guided_update_scenario_status(&mut self.runner) {
                        self.show_error(&format!("Error updating scenario status: {}", e))?;
                    }
                }
                MainMenuOption::ListUseCases => {
                    if let Err(e) = self.runner.list_use_cases() {
                        self.show_error(&format!("Error listing use cases: {}", e))?;
                    }
                }
                MainMenuOption::ShowStatus => {
                    if let Err(e) = self.runner.show_status() {
                        self.show_error(&format!("Error showing status: {}", e))?;
                    }
                }
                MainMenuOption::ShowLanguages => {
                    match CliRunner::show_languages() {
                        Ok(languages) => println!("\n{}", languages),
                        Err(e) => self.show_error(&format!("Error showing languages: {}", e))?,
                    }
                }
                MainMenuOption::Exit => {
                    self.show_goodbye()?;
                    break;
                }
            }

            // Pause before showing menu again
            self.pause_for_input()?;
        }

        Ok(())
    }

    /// Show welcome message
    fn show_welcome(&self) -> Result<()> {
        self.clear_screen()?;
        
        execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print("╔══════════════════════════════════════════════════════════════╗\n"),
            Print("║                                                              ║\n"),
            Print("║        📝 Markdown Use Case Manager - Interactive Mode       ║\n"),
            Print("║                                                              ║\n"),
            Print("║          Manage your use cases and scenarios with ease       ║\n"),
            Print("║                                                              ║\n"),
            Print("╚══════════════════════════════════════════════════════════════╝\n"),
            ResetColor,
            Print("\n")
        )?;

        Ok(())
    }

    /// Show goodbye message
    fn show_goodbye(&self) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("\n🎉 Thank you for using Markdown Use Case Manager!\n"),
            Print("📚 Happy documenting! 📚\n\n"),
            ResetColor
        )?;

        Ok(())
    }

    /// Show error message
    fn show_error(&self, message: &str) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(&format!("\n❌ {}\n", message)),
            ResetColor
        )?;

        Ok(())
    }

    /// Clear the screen
    fn clear_screen(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    /// Pause for user input before continuing
    fn pause_for_input(&self) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("\nPress Enter to continue..."),
            ResetColor
        )?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        Ok(())
    }
}