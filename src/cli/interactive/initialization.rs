use anyhow::Result;
use inquire::{Confirm, MultiSelect};

use super::ui::UI;
use crate::cli::runner::CliRunner;
use crate::config::{Config, TemplateManager};
use crate::core::{LanguageRegistry, MethodologyRegistry, Methodology};

/// Handle project initialization workflow
pub struct Initialization;

impl Initialization {
    /// Check if project is initialized, offer to initialize if not
    pub fn check_and_initialize(runner: &mut CliRunner) -> Result<()> {
        // Try to load config
        if Config::load().is_err() {
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
            Self::run_initialization_wizard(runner)?;
        }

        Ok(())
    }

    /// Run the full initialization wizard
    fn run_initialization_wizard(runner: &mut CliRunner) -> Result<()> {
        // Step 1: Select programming language
        let language = Self::select_programming_language()?;

        // Step 2: Select methodologies
        let selected_methodologies = Self::select_methodologies()?;

        // Step 3: Select default methodology
        let default_methodology = Self::select_default_methodology(&selected_methodologies)?;

        // Show summary
        Self::show_configuration_summary(&language, &selected_methodologies, &default_methodology)?;

        // Create config
        Self::create_config(runner, language, default_methodology)?;

        // Finalize
        Self::finalize_initialization(runner)?;

        Ok(())
    }

    /// Step 1: Select programming language
    fn select_programming_language() -> Result<Option<String>> {
        UI::show_step(
            1,
            "Project Programming Language",
            "Select the primary programming language for your project.\nThis is used for test scaffolding generation.",
        )?;

        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let languages = LanguageRegistry::discover_available(&templates_dir)?;
        let mut language_options = vec!["none".to_string()];
        language_options.extend(languages);

        let language = inquire::Select::new("Programming language:", language_options)
            .with_help_message("Choose 'none' if you don't need test scaffolding")
            .prompt()?;

        Ok(if language == "none" {
            None
        } else {
            Some(language)
        })
    }

    /// Step 2: Select methodologies
    fn select_methodologies() -> Result<Vec<String>> {
        UI::show_step(
            2,
            "Use Case Methodologies",
            "Select which methodologies you plan to use for documenting use cases.\n💡 You can always add or remove methodologies later!",
        )?;

        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let methodologies = MethodologyRegistry::discover_available(&templates_dir)?;

        if methodologies.is_empty() {
            UI::show_error("No methodologies available. This is unexpected.")?;
            return Err(anyhow::anyhow!("No methodologies found"));
        }

        // Get methodology descriptions for better selection
        let methodology_display = Self::get_methodology_descriptions(&methodologies);

        let selected =
            MultiSelect::new("Select methodologies to use:", methodology_display.clone())
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

        Ok(selected_methodologies)
    }

    /// Step 3: Select default methodology
    fn select_default_methodology(selected_methodologies: &[String]) -> Result<String> {
        UI::show_step(
            3,
            "Default Methodology",
            "Choose which methodology to use by default when creating use cases.",
        )?;

        if selected_methodologies.len() == 1 {
            return Ok(selected_methodologies[0].clone());
        }

        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let all_methodologies = MethodologyRegistry::discover_available(&templates_dir)?;
        let methodology_display = Self::get_methodology_descriptions(&all_methodologies);

        let default_options: Vec<String> = selected_methodologies
            .iter()
            .filter_map(|m| {
                methodology_display
                    .iter()
                    .find(|d| d.to_lowercase().starts_with(m))
                    .cloned()
            })
            .collect();

        let default_display = inquire::Select::new("Default methodology:", default_options)
            .with_help_message("This will be used when no methodology is specified")
            .prompt()?;

        Ok(default_display
            .split_whitespace()
            .next()
            .unwrap()
            .to_lowercase())
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

    /// Create the configuration file
    fn create_config(
        runner: &mut CliRunner,
        language: Option<String>,
        default_methodology: String,
    ) -> Result<()> {
        match runner.init_project(language, Some(default_methodology)) {
            Ok(result) => {
                UI::show_success(&result.message)?;
                Ok(())
            }
            Err(e) => {
                UI::show_error(&format!("Failed to initialize project: {}", e))?;
                Err(e)
            }
        }
    }

    /// Finalize initialization (copy templates)
    fn finalize_initialization(runner: &mut CliRunner) -> Result<()> {
        let auto_finalize = Confirm::new("Finalize initialization now?")
            .with_default(true)
            .with_help_message(
                "This will copy templates. Choose 'No' to review the config file first",
            )
            .prompt()?;

        if auto_finalize {
            match runner.finalize_init() {
                Ok(result) => {
                    UI::show_success(&result.message)?;
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

    /// Get methodology descriptions for display
    fn get_methodology_descriptions(methodologies: &[String]) -> Vec<String> {
        use crate::config::TemplateManager;
        use crate::core::MethodologyRegistry;

        let templates_dir = match TemplateManager::find_source_templates_dir() {
            Ok(dir) => dir,
            Err(_) => return methodologies.iter().map(|m| m.clone()).collect(), // Fallback to just names
        };

        let registry = match MethodologyRegistry::new_dynamic(&templates_dir) {
            Ok(reg) => reg,
            Err(_) => return methodologies.iter().map(|m| m.clone()).collect(), // Fallback to just names
        };

        methodologies
            .iter()
            .map(|m| {
                // Capitalize first letter and format
                let formatted = m
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().next().unwrap()
                        } else {
                            c
                        }
                    })
                    .collect::<String>();

                // Get description from methodology definition
                if let Some(methodology_def) = registry.get(m) {
                    format!("{} - {}", formatted, methodology_def.description())
                } else {
                    formatted // Fallback if methodology not found
                }
            })
            .collect()
    }
}
