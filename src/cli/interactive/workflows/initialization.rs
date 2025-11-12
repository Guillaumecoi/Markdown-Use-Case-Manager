//! # Interactive Project Initialization
//!
//! Guided project setup wizard for interactive CLI mode.
//! Walks users through configuring their project with language, methodologies, and templates.

use anyhow::Result;
use inquire::{Confirm, MultiSelect, Select};

use crate::cli::interactive::runner::InteractiveRunner;
use crate::cli::interactive::selectors::{
    get_available_languages, get_available_methodologies, get_methodology_descriptions,
};
use crate::cli::interactive::ui::UI;

/// Handle project initialization workflow
pub struct Initialization;

impl Initialization {
    /// Check if project is initialized, offer to initialize if not
    pub fn check_and_initialize() -> Result<()> {
        // Try to load config
        if crate::config::Config::load().is_err() {
            UI::clear_screen()?;
            UI::show_init_wizard_header()?;

            let should_init = Confirm::new("Would you like to initialize a new project?")
                .with_default(true)
                .prompt()?;

            if !should_init {
                UI::show_warning(
                    "Exiting without initializing. Run 'mucm init' to initialize later.",
                )?;
                return Err(anyhow::anyhow!("Project not initialized"));
            }

            // Run the initialization wizard
            Self::run_initialization_wizard()?;
        }

        Ok(())
    }

    /// Run the full initialization wizard
    fn run_initialization_wizard() -> Result<()> {
        let mut runner = InteractiveRunner::new();

        // Step 1: Select programming language
        UI::show_step(
            1,
            "Project Programming Language",
            "Select the primary programming language for your project.\nThis is used for test scaffolding generation.",
        )?;

        let languages = get_available_languages(&mut runner)?;
        let mut language_options = vec!["none".to_string()];
        language_options.extend(languages);

        let language_selection = Select::new("Programming language:", language_options)
            .with_help_message("Choose 'none' if you don't need test scaffolding")
            .prompt()?;

        let language = if language_selection == "none" {
            None
        } else {
            Some(language_selection)
        };

        // Step 2: Select methodologies
        UI::show_step(
            2,
            "Use Case Methodologies",
            "Select which methodologies you plan to use for documenting use cases.\n💡 You can always add or remove methodologies later!",
        )?;

        let methodology_infos = get_available_methodologies(&mut runner)?;

        if methodology_infos.is_empty() {
            UI::show_error("No methodologies available. This is unexpected.")?;
            return Err(anyhow::anyhow!("No methodologies found"));
        }

        let methodology_display = get_methodology_descriptions(&methodology_infos);

        let selected = MultiSelect::new("Select methodologies:", methodology_display.clone())
            .with_help_message(
                "Use Space to select/deselect, Enter to confirm. Select at least one.",
            )
            .prompt()?;

        if selected.is_empty() {
            UI::show_error("You must select at least one methodology.")?;
            return Err(anyhow::anyhow!("No methodology selected"));
        }

        // Extract methodology names from display strings
        let selected_methodologies: Vec<String> = selected
            .iter()
            .map(|display| {
                // Extract the methodology name (before the first space or dash)
                display.split_whitespace().next().unwrap().to_lowercase()
            })
            .collect();

        // Step 3: Select default methodology
        UI::show_step(
            3,
            "Default Methodology",
            "Choose which methodology to use by default when creating use cases.",
        )?;

        let default_methodology = if selected_methodologies.len() == 1 {
            selected_methodologies[0].clone()
        } else {
            let methodology_display = get_methodology_descriptions(&methodology_infos);

            let default_options: Vec<String> = selected_methodologies
                .iter()
                .filter_map(|m| {
                    methodology_display
                        .iter()
                        .find(|d| d.to_lowercase().starts_with(m))
                        .cloned()
                })
                .collect();

            let default_display = Select::new("Default methodology:", default_options)
                .with_help_message("This will be used when no methodology is specified")
                .prompt()?;

            default_display
                .split_whitespace()
                .next()
                .unwrap()
                .to_lowercase()
        };

        // Show summary
        show_configuration_summary(&language, &selected_methodologies, &default_methodology)?;

        // Create config
        create_config(&mut runner, language, default_methodology)?;

        // Finalize
        finalize_initialization(&mut runner)?;

        Ok(())
    }
}

/// Show configuration summary
fn show_configuration_summary(
    language: &Option<String>,
    selected_methodologies: &[String],
    default_methodology: &str,
) -> Result<()> {
    println!("\n✨ Configuration Summary:");
    println!(
        "   Language: {}",
        language.as_ref().unwrap_or(&"none".to_string())
    );
    println!("   Methodologies: {}", selected_methodologies.join(", "));
    println!("   Default: {}\n", default_methodology);
    Ok(())
}

/// Create project configuration
fn create_config(
    runner: &mut InteractiveRunner,
    language: Option<String>,
    default_methodology: String,
) -> Result<()> {
    match runner.initialize_project(language, default_methodology) {
        Ok(message) => {
            UI::show_success(&message)?;
            Ok(())
        }
        Err(e) => {
            UI::show_error(&format!("Failed to initialize project: {}", e))?;
            Err(e)
        }
    }
}

/// Finalize project initialization
fn finalize_initialization(runner: &mut InteractiveRunner) -> Result<()> {
    let auto_finalize = Confirm::new("Finalize initialization now?")
        .with_default(true)
        .with_help_message("This will copy templates. Choose 'No' to review the config file first")
        .prompt()?;

    if auto_finalize {
        match runner.finalize_initialization() {
            Ok(message) => {
                UI::show_success(&message)?;
                println!("\n💡 Note: All selected methodologies are now available!");
                println!("   You can use any of them when creating use cases.\n");
                UI::pause_for_input()?;
                Ok(())
            }
            Err(e) => {
                UI::show_error(&format!("Failed to finalize initialization: {}", e))?;
                Err(e)
            }
        }
    } else {
        UI::show_warning(
            "📝 Configuration created but not finalized.\n\
            Review .config/.mucm/mucm.toml and run 'mucm init --finalize' when ready.",
        )?;
        Err(anyhow::anyhow!("Project initialization not finalized"))
    }
}
