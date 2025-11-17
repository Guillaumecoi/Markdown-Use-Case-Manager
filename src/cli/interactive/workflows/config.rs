//! # Configuration Workflow
//!
//! Interactive configuration management for project settings.
//! Handles general project configuration excluding methodologies and use cases.

use anyhow::Result;
use inquire::{Confirm, Select, Text};

use crate::cli::interactive::ui::UI;
use crate::config::{Config, TemplateManager};
use crate::core::LanguageRegistry;

/// Configuration workflow handler
pub struct ConfigWorkflow;

impl ConfigWorkflow {
    /// Configure project information
    pub fn configure_project_info(config: &mut Config) -> Result<()> {
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
    pub fn configure_directories(config: &mut Config) -> Result<()> {
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
    pub fn configure_generation(config: &mut Config) -> Result<()> {
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
    pub fn configure_metadata(config: &mut Config) -> Result<()> {
        println!("\n📊 Metadata Configuration");
        println!("────────────────────────────");

        config.metadata.created = Confirm::new("Auto-set 'created' timestamp?")
            .with_default(config.metadata.created)
            .prompt()?;

        config.metadata.last_updated = Confirm::new("Auto-update 'last_updated' timestamp?")
            .with_default(config.metadata.last_updated)
            .prompt()?;

        println!("\n💡 To configure additional fields (author, reviewer, status, priority, etc.),");
        println!("   edit [extra_fields] section in .config/.mucm/mucm.toml after saving.\n");

        Ok(())
    }

    /// Change programming language setting
    pub fn change_programming_language() -> Result<()> {
        UI::show_section_header("Change Programming Language", "🔧")?;

        let languages = crate::controller::ProjectController::get_available_languages()?;
        let mut language_options = vec!["none".to_string()];
        language_options.extend(languages.items);

        let language = Select::new("Select new programming language:", language_options)
            .with_help_message("Choose 'none' to disable test scaffolding")
            .prompt()?;

        // Update config
        let mut config = Config::load()?;
        config.generation.test_language = language;

        config.save_in_dir(".")?;
        UI::show_success("✅ Programming language updated successfully!")?;

        Ok(())
    }

    /// View current configuration
    pub fn view_config(config: &Config) -> Result<()> {
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
    pub fn save_config(config: &Config) -> Result<()> {
        config.save_in_dir(".")?;
        UI::show_success("✅ Configuration saved successfully!")?;
        Ok(())
    }
}
