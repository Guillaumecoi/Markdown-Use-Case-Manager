use anyhow::Result;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

/// UI utility functions for the interactive mode
pub struct UI;

impl UI {
    /// Show welcome message
    pub fn show_welcome() -> Result<()> {
        Self::clear_screen()?;

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
    pub fn show_goodbye() -> Result<()> {
        println!("👋 Thanks for using MD Use Case Manager!");
        Ok(())
    }

    /// Show error message
    pub fn show_error(message: &str) -> Result<()> {
        execute!(
            stdout(),
            Print(&format!("\n❌ {message}\n")),
            Print("Press Enter to continue..."),
        )?;
        let mut _input = String::new();
        std::io::stdin().read_line(&mut _input)?;
        Ok(())
    }

    /// Clear the screen
    pub fn clear_screen() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    /// Pause for user input before continuing
    pub fn pause_for_input() -> Result<()> {
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

    /// Show a section header
    pub fn show_section_header(title: &str, icon: &str) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print(&format!("{} {}\n", icon, title)),
            Print("═".repeat(title.len() + icon.len() + 1)),
            Print("\n\n"),
            ResetColor
        )?;
        Ok(())
    }

    /// Show a step in a wizard
    pub fn show_step(step_number: usize, title: &str, description: &str) -> Result<()> {
        println!("\n📚 Step {}: {}", step_number, title);
        println!("────────────────────────────────────────");
        println!("{}\n", description);
        Ok(())
    }

    /// Show the initialization wizard header
    pub fn show_init_wizard_header() -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print("╔══════════════════════════════════════════════════════════════╗\n"),
            Print("║                                                              ║\n"),
            Print("║              🚀 Project Initialization Wizard                ║\n"),
            Print("║                                                              ║\n"),
            Print("╚══════════════════════════════════════════════════════════════╝\n"),
            ResetColor,
            Print("\n"),
            SetForegroundColor(Color::Yellow),
            Print("No use case manager project found in this directory.\n"),
            ResetColor,
            Print("\nLet's set up a new project to manage your use cases!\n\n")
        )?;
        Ok(())
    }

    /// Show a success message
    pub fn show_success(message: &str) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(&format!("\n{}\n", message)),
            ResetColor
        )?;
        Ok(())
    }

    /// Show a warning message
    pub fn show_warning(message: &str) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Yellow),
            Print(&format!("\n{}\n", message)),
            ResetColor
        )?;
        Ok(())
    }
}
