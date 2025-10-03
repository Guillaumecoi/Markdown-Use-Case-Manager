use anyhow::Result;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

use crate::cli::interactive::menu::{
    guided_add_scenario, guided_create_use_case, guided_update_scenario_status, show_main_menu,
    MainMenuOption,
};
use crate::cli::runner::CliRunner;

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
        if self.check_initialization().is_err() {
            return Ok(());
        }

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
                MainMenuOption::ConfigureSettings => {
                    if let Err(e) = self.configure_settings() {
                        self.show_error(&format!("Error configuring settings: {}", e))?;
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
                MainMenuOption::ShowLanguages => match CliRunner::show_languages() {
                    Ok(languages) => println!("\n{}", languages),
                    Err(e) => self.show_error(&format!("Error showing languages: {}", e))?,
                },
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

    /// Check if project is initialized, offer to initialize if not
    fn check_initialization(&mut self) -> Result<()> {
        use crate::config::Config;
        use inquire::Confirm;

        // Try to load config
        if Config::load().is_err() {
            self.clear_screen()?;

            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                Print("🔧 No use case manager project found in this directory.\n"),
                ResetColor,
                Print("Would you like to initialize a new project here?\n")
            )?;

            let should_init = Confirm::new("Initialize project?")
                .with_default(true)
                .prompt()?;

            if should_init {
                // Ask for language
                let languages = Config::get_available_languages()?;
                let mut language_options = vec!["none".to_string()];
                language_options.extend(languages);

                let language = inquire::Select::new("Choose test language:", language_options)
                    .with_help_message("Select a programming language for test generation, or 'none' to skip test generation")
                    .prompt()?;

                let language = if language == "none" {
                    None
                } else {
                    Some(language)
                };

                match self.runner.init_project(language) {
                    Ok(message) => {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Green),
                            Print(&format!("\n✅ {}\n", message)),
                            ResetColor
                        )?;
                        self.pause_for_input()?;
                    }
                    Err(e) => {
                        self.show_error(&format!("Failed to initialize project: {}", e))?;
                        return Err(e);
                    }
                }
            } else {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::Yellow),
                    Print("\nExiting without initializing. Run 'mucm init' to initialize later.\n"),
                    ResetColor
                )?;
                return Err(anyhow::anyhow!("Project not initialized"));
            }
        }

        Ok(())
    }

    /// Interactive settings configuration
    fn configure_settings(&mut self) -> Result<()> {
        use crate::config::Config;
        use inquire::{Confirm, Select, Text};

        self.clear_screen()?;

        execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print("⚙️  Configuration Settings\n"),
            Print("═══════════════════════\n\n"),
            ResetColor
        )?;

        // Load current config
        let mut config = Config::load()?;

        loop {
            let options = vec![
                "Project Information",
                "Directory Settings",
                "Generation Settings",
                "Metadata Configuration",
                "View Current Config",
                "Save & Exit",
            ];

            let choice = Select::new("What would you like to configure?", options).prompt()?;

            match choice {
                "Project Information" => {
                    println!("\n📋 Project Information");
                    println!("────────────────────────");

                    config.project.name = Text::new("Project name:")
                        .with_default(&config.project.name)
                        .prompt()?;

                    config.project.description = Text::new("Project description:")
                        .with_default(&config.project.description)
                        .prompt()?;
                }
                "Directory Settings" => {
                    println!("\n📁 Directory Settings");
                    println!("───────────────────────");

                    config.directories.use_case_dir = Text::new("Use case directory:")
                        .with_default(&config.directories.use_case_dir)
                        .with_help_message("Where to store use case markdown files")
                        .prompt()?;

                    config.directories.test_dir = Text::new("Test directory:")
                        .with_default(&config.directories.test_dir)
                        .with_help_message("Where to generate test scaffolding")
                        .prompt()?;
                }
                "Generation Settings" => {
                    println!("\n🔧 Generation Settings");
                    println!("────────────────────────");

                    let languages = Config::get_available_languages()?;
                    let mut language_options = vec!["none".to_string()];
                    language_options.extend(languages);

                    config.generation.test_language =
                        Select::new("Test language:", language_options)
                            .with_help_message("Programming language for test generation")
                            .prompt()?;

                    config.generation.auto_generate_tests = Confirm::new("Auto-generate tests?")
                        .with_default(config.generation.auto_generate_tests)
                        .prompt()?;

                    if config.templates.use_case_style.is_some() {
                        let style_options = vec!["simple", "detailed"];
                        let selected_style = Select::new("Use case template style:", style_options)
                            .with_help_message(
                                "Choose between simple or detailed use case templates",
                            )
                            .prompt()?;
                        config.templates.use_case_style = Some(selected_style.to_string());
                    }
                }
                "Metadata Configuration" => {
                    println!("\n📊 Metadata Configuration");
                    println!("────────────────────────────");

                    config.metadata.enabled = Confirm::new("Enable metadata generation?")
                        .with_default(config.metadata.enabled)
                        .prompt()?;

                    if config.metadata.enabled {
                        println!("\nWhich auto-generated fields to include:");

                        config.metadata.include_id = Confirm::new("Include ID?")
                            .with_default(config.metadata.include_id)
                            .prompt()?;

                        config.metadata.include_status = Confirm::new("Include status?")
                            .with_default(config.metadata.include_status)
                            .prompt()?;

                        config.metadata.include_priority = Confirm::new("Include priority?")
                            .with_default(config.metadata.include_priority)
                            .prompt()?;

                        config.metadata.include_created = Confirm::new("Include creation date?")
                            .with_default(config.metadata.include_created)
                            .prompt()?;
                    }
                }
                "View Current Config" => {
                    println!("\n📄 Current Configuration");
                    println!("═══════════════════════════");
                    println!(
                        "Project: {} - {}",
                        config.project.name, config.project.description
                    );
                    println!("Use Case Dir: {}", config.directories.use_case_dir);
                    println!("Test Dir: {}", config.directories.test_dir);
                    println!("Test Language: {}", config.generation.test_language);
                    println!(
                        "Auto Generate Tests: {}",
                        config.generation.auto_generate_tests
                    );
                    println!("Metadata Enabled: {}", config.metadata.enabled);

                    self.pause_for_input()?;
                }
                "Save & Exit" => {
                    config.save_in_dir(".")?;

                    execute!(
                        stdout(),
                        SetForegroundColor(Color::Green),
                        Print("\n✅ Configuration saved successfully!\n"),
                        ResetColor
                    )?;

                    self.pause_for_input()?;
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
