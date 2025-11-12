//! # Interactive Settings Configuration
//!
//! Menu-driven settings configuration for interactive CLI mode.
//! Allows users to modify project configuration through guided prompts.

use anyhow::Result;
use inquire::{Confirm, Select, Text};

use super::ui::UI;
use crate::cli::standard::CliRunner;
use crate::config::{Config, TemplateManager};
use crate::core::LanguageRegistry;

/// Handle settings configuration
pub struct Settings;

impl Settings {
    /// Interactive settings configuration menu
    pub fn configure(_runner: &mut CliRunner) -> Result<()> {
        UI::clear_screen()?;
        UI::show_section_header("Configuration Settings", "⚙️")?;

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
                "Project Information" => Self::configure_project_info(&mut config)?,
                "Directory Settings" => Self::configure_directories(&mut config)?,
                "Generation Settings" => Self::configure_generation(&mut config)?,
                "Metadata Configuration" => Self::configure_metadata(&mut config)?,
                "View Current Config" => Self::view_config(&config)?,
                "Save & Exit" => {
                    Self::save_config(&config)?;
                    UI::pause_for_input()?;
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Configure project information
    fn configure_project_info(config: &mut Config) -> Result<()> {
        println!("\n📋 Project Information");
        println!("────────────────────────");

        config.project.name = Text::new("Project name:")
            .with_default(&config.project.name)
            .prompt()?;

        config.project.description = Text::new("Project description:")
            .with_default(&config.project.description)
            .prompt()?;

        Ok(())
    }

    /// Configure directory settings
    fn configure_directories(config: &mut Config) -> Result<()> {
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

        Ok(())
    }

    /// Configure generation settings
    fn configure_generation(config: &mut Config) -> Result<()> {
        println!("\n🔧 Generation Settings");
        println!("────────────────────────");

        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let languages = LanguageRegistry::discover_available(&templates_dir)?;
        let mut language_options = vec!["none".to_string()];
        language_options.extend(languages);

        config.generation.test_language = Select::new("Test language:", language_options)
            .with_help_message("Programming language for test generation")
            .prompt()?;

        config.generation.auto_generate_tests = Confirm::new("Auto-generate tests?")
            .with_default(config.generation.auto_generate_tests)
            .prompt()?;

        Ok(())
    }

    /// Configure metadata settings
    fn configure_metadata(config: &mut Config) -> Result<()> {
        println!("\n📊 Metadata Configuration");
        println!("────────────────────────────");

        config.metadata.created = Confirm::new("Auto-set 'created' timestamp?")
            .with_default(config.metadata.created)
            .prompt()?;

        config.metadata.last_updated = Confirm::new("Auto-update 'last_updated' timestamp?")
            .with_default(config.metadata.last_updated)
            .prompt()?;

        println!("\n💡 To configure additional fields (author, reviewer, status, priority, etc.),");
        println!("   edit [base_fields] section in .config/.mucm/mucm.toml after saving.\n");

        Ok(())
    }

    /// View current configuration
    fn view_config(config: &Config) -> Result<()> {
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
        println!("Auto-set 'created': {}", config.metadata.created);
        println!(
            "Auto-update 'last_updated': {}",
            config.metadata.last_updated
        );

        UI::pause_for_input()?;
        Ok(())
    }

    /// Save configuration
    fn save_config(config: &Config) -> Result<()> {
        config.save_in_dir(".")?;
        UI::show_success("✅ Configuration saved successfully!")?;
        Ok(())
    }
}
