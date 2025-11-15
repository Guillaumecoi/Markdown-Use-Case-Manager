//! # Project Controller
//!
//! This module provides the controller for project-level operations including
//! initialization, configuration management, and project status checking.
//! It coordinates between the CLI layer and the configuration management system.
//!
//! ## Responsibilities
//!
//! - Project initialization (two-step process: config creation, then template copying)
//! - Configuration validation and status checking
//! - Methodology and language information retrieval
//! - Project setup coordination and user guidance
//!
//! ## Initialization Process
//!
//! Project initialization follows a two-step process:
//! 1. **Configuration Creation**: Creates `.config/.mucm/mucm.toml` with user preferences
//! 2. **Template Finalization**: Copies methodology and language templates to project
//!
//! This separation allows users to review and customize configuration before
//! committing to template copying.

use anyhow::Result;

use super::dto::{DisplayResult, MethodologyInfo, SelectionOptions};
use crate::config::Config;
use crate::core::{LanguageRegistry, Methodology, MethodologyRegistry};

/// Controller for project initialization and management operations.
///
/// Handles all project-level operations including initialization, configuration
/// management, and providing information about available methodologies and languages.
/// Acts as the coordination layer between CLI commands and the configuration system.
pub struct ProjectController;

impl ProjectController {
    /// Check if a project is already initialized.
    ///
    /// Determines whether a use case manager project has been set up in the
    /// current directory or any parent directory by checking for a valid
    /// configuration file.
    ///
    /// # Returns
    /// True if a project is initialized, false otherwise
    pub fn is_initialized() -> bool {
        Config::load().is_ok()
    }

    /// Get available languages.
    ///
    /// Retrieves all supported programming languages that can be used for
    /// test generation and template selection.
    ///
    /// # Returns
    /// SelectionOptions containing available language names
    ///
    /// TODO: Use this in interactive init workflow for language selection
    pub fn get_available_languages() -> Result<SelectionOptions> {
        use crate::config::Config;

        // Always load language metadata (info.toml) from source templates
        let templates_dir = Config::get_metadata_load_dir()?;
        let languages = LanguageRegistry::discover_available(&templates_dir)?;
        Ok(SelectionOptions::new(languages))
    }

    /// Get available methodologies with descriptions.
    ///
    /// Retrieves all available methodologies with their display names and
    /// descriptions for user selection and information display.
    ///
    /// # Returns
    /// Vector of MethodologyInfo containing name, display name, and description
    pub fn get_available_methodologies() -> Result<Vec<MethodologyInfo>> {
        use crate::config::Config;

        // Always load methodology metadata (info.toml) from source templates
        let templates_dir = Config::get_metadata_load_dir()?;
        let registry = MethodologyRegistry::new_dynamic(&templates_dir)?;

        let methodology_infos: Vec<MethodologyInfo> = registry
            .available_methodologies()
            .into_iter()
            .map(|name| {
                let methodology_def = registry.get(&name).unwrap(); // Should always exist since we got it from available_methodologies

                let display_name = name
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

                MethodologyInfo {
                    name: name.clone(),
                    display_name,
                    description: methodology_def.description().to_string(),
                }
            })
            .collect();

        Ok(methodology_infos)
    }

    /// Initialize a new project (Step 1: Create config).
    ///
    /// Creates the initial project configuration file with user-specified
    /// language and methodology preferences. This is the first step in
    /// project initialization.
    ///
    /// # Arguments
    /// * `language` - Optional programming language for test generation
    /// * `default_methodology` - Default methodology for use case creation
    ///
    /// # Returns
    /// DisplayResult with success message and next steps guidance
    ///
    /// # Errors
    /// Returns error if project is already initialized or configuration creation fails
    pub fn init_project(
        language: Option<String>,
        default_methodology: String,
    ) -> Result<DisplayResult> {
        // Check if already initialized
        if Self::is_initialized() {
            return Ok(DisplayResult::error(
                "A use case manager project already exists in this directory or a parent directory"
                    .to_string(),
            ));
        }

        // Resolve language aliases to primary names
        let resolved_language = if let Some(lang) = language {
            use crate::config::Config;

            // Always load language metadata (info.toml) from source templates
            let templates_dir = Config::get_metadata_load_dir()?;
            let language_registry = LanguageRegistry::new_dynamic(&templates_dir)?;
            if let Some(lang_def) = language_registry.get(&lang) {
                lang_def.name().to_string()
            } else {
                lang
            }
        } else {
            "rust".to_string()
        };

        // Create minimal config
        let config =
            Config::for_template(Some(resolved_language), Some(default_methodology.clone()));

        // Save config file
        Config::save_config_only(&config)?;

        let message = format!(
            "✅ Configuration file created at .config/.mucm/mucm.toml\n\n\
             📝 Please review and customize the configuration:\n\
             - Programming language: {}\n\
             - Default Methodology: {}\n\
             - TOML directory: {}\n\
             - Use case directory: {}\n\
             - Test directory: {}\n\n\
             ⚡ When ready, run: mucm init --finalize\n\n\
             \n\
             💡 The finalize step will:\n\
             - Copy the used methodology templates\n\
             - Copy the used language templates\n\
             - You can use any methodology when creating use cases\n\
             - Directories will be created when you create your first use case",
            config.generation.test_language,
            &config.templates.default_methodology,
            config
                .directories
                .toml_dir
                .as_deref()
                .unwrap_or("docs/use-cases"),
            config.directories.use_case_dir,
            config.directories.test_dir,
        );

        Ok(DisplayResult::success(message))
    }

    /// Initialize a new project with multiple methodologies (Step 1: Create config).
    ///
    /// Creates the initial project configuration file with user-specified
    /// language, methodologies, storage backend, and default methodology preferences. This is the first step in
    /// project initialization.
    ///
    /// # Arguments
    /// * `language` - Optional programming language for test generation
    /// * `methodologies` - List of methodologies to enable
    /// * `storage` - Storage backend to use (toml or sqlite)
    /// * `default_methodology` - Default methodology for use case creation
    ///
    /// # Returns
    /// DisplayResult with success message and next steps guidance
    ///
    /// # Errors
    /// Returns error if project is already initialized or configuration creation fails
    pub fn init_project_with_methodologies(
        language: Option<String>,
        methodologies: Vec<String>,
        storage: String,
        default_methodology: String,
    ) -> Result<DisplayResult> {
        // Check if already initialized
        if Self::is_initialized() {
            return Ok(DisplayResult::error(
                "A use case manager project already exists in this directory or a parent directory"
                    .to_string(),
            ));
        }

        // Resolve language aliases to primary names
        let resolved_language = if let Some(lang) = language {
            use crate::config::Config;

            // Always load language metadata (info.toml) from source templates
            let templates_dir = Config::get_metadata_load_dir()?;
            let language_registry = LanguageRegistry::new_dynamic(&templates_dir)?;
            if let Some(lang_def) = language_registry.get(&lang) {
                lang_def.name().to_string()
            } else {
                lang
            }
        } else {
            "rust".to_string()
        };

        // Create minimal config with specified methodologies and storage
        let config = Config::for_template_with_methodologies_and_storage(
            Some(resolved_language),
            methodologies,
            Some(default_methodology.clone()),
            storage,
        );

        // Save config file
        Config::save_config_only(&config)?;

        let message = format!(
            "✅ Configuration file created at .config/.mucm/mucm.toml\n\n\
             📝 Please review and customize the configuration:\n\
             - Programming language: {}\n\
             - Default Methodology: {}\n\
             - Enabled Methodologies: {}\n\
             - Storage Backend: {}\n\
             - TOML directory: {}\n\
             - Use case directory: {}\n\
             - Test directory: {}\n\n\
             ⚡ When ready, run: mucm init --finalize\n\n\
             \n\
             💡 The finalize step will:\n\
             - Copy the selected methodology templates\n\
             - Copy the selected language templates\n\
             - You can use any enabled methodology when creating use cases\n\
             - Directories will be created when you create your first use case",
            config.generation.test_language,
            &config.templates.default_methodology,
            config.templates.methodologies.join(", "),
            config.storage.backend,
            config
                .directories
                .toml_dir
                .as_deref()
                .unwrap_or("docs/use-cases"),
            config.directories.use_case_dir,
            config.directories.test_dir,
        );

        Ok(DisplayResult::success(message))
    }

    /// Finalize project initialization (Step 2: Copy templates).
    ///
    /// Completes project setup by copying methodology and language templates
    /// to the project configuration directory. This is the second and final
    /// step in project initialization.
    ///
    /// # Returns
    /// DisplayResult with completion message and usage guidance
    ///
    /// # Errors
    /// Returns error if configuration doesn't exist or template copying fails
    pub fn finalize_init() -> Result<DisplayResult> {
        // Check if config exists
        let config = Config::load().map_err(|_| {
            anyhow::anyhow!("No configuration file found. Please run 'mucm init' first to create the configuration.")
        })?;

        // Check if already finalized
        if Config::check_templates_exist() {
            return Ok(DisplayResult::error(
                "Project already finalized. Templates directory exists.\n\
                 If you want to re-copy templates, delete .config/.mucm/handlebars/ first."
                    .to_string(),
            ));
        }

        // Copy templates
        Config::copy_templates_to_config_with_language(Some(
            config.generation.test_language.clone(),
        ))?;

        // Get available methodologies
        use crate::config::TemplateManager;
        let templates_dir = TemplateManager::find_source_templates_dir()?;
        let available = MethodologyRegistry::discover_available(&templates_dir).unwrap_or_default();
        let methodologies_list = if available.is_empty() {
            "Unable to detect".to_string()
        } else {
            available.join(", ")
        };

        let message = format!(
            "✅ Project setup complete!\n\n\
             📁 Templates copied to: .config/.mucm/handlebars/\n\
             🔧 Language: {}\n\
             📚 Default Methodology: {}\n\
             📋 Available Methodologies: {}\n\n\
             🚀 You're ready to create use cases!\n\
             - Run: mucm create --category <category> \"<title>\" --methodology <name>\n\
             - Run: mucm list to see all use cases\n\
             - Run: mucm methodologies to see all available methodologies\n\
             - Run: mucm --help for all available commands\n\n\
             💡 Each methodology has its own settings (test generation, metadata, etc.)",
            config.generation.test_language,
            &config.templates.default_methodology,
            methodologies_list
        );

        Ok(DisplayResult::success(message))
    }

    /// Get the default methodology from configuration.
    ///
    /// Retrieves the default methodology setting from the current project
    /// configuration for use in interactive workflows.
    ///
    /// # Returns
    /// The default methodology name as a string
    ///
    /// TODO: Use this in interactive mode to pre-select default methodology
    pub fn get_default_methodology() -> Result<String> {
        let config = Config::load()?;
        Ok(config.templates.default_methodology.clone())
    }

    /// Get available languages as a formatted string for display.
    ///
    /// Provides a user-friendly formatted list of available programming
    /// languages with usage instructions and fallback information.
    ///
    /// # Returns
    /// Formatted string containing language list and usage instructions
    pub fn show_languages() -> Result<String> {
        let mut output = String::from("Available programming languages:\n");

        use crate::config::Config;

        // Always load language metadata (info.toml) from source templates
        let templates_dir = match Config::get_metadata_load_dir() {
            Ok(dir) => dir,
            Err(e) => {
                return Ok(format!(
                    "Error: Could not find templates directory: {}\n",
                    e
                ));
            }
        };

        match LanguageRegistry::discover_available(&templates_dir) {
            Ok(languages) => {
                for lang in languages {
                    output.push_str(&format!("  - {}\n", lang));
                }
                output.push_str(
                    "\nTo initialize with a specific language: mucm init -l <language>\n",
                );
                output.push_str("To add a new language manually, create a directory: .config/.mucm/handlebars/lang-<language>/\n");
            }
            Err(e) => {
                output.push_str(&format!("Error getting available languages: {}\n", e));
                // Fallback to show built-in languages if discovery fails
                if let Ok(language_registry) = LanguageRegistry::new_dynamic(&templates_dir) {
                    let builtin_languages = language_registry.available_languages();
                    output.push_str(&format!(
                        "Built-in languages: {}\n",
                        builtin_languages.join(", ")
                    ));
                }
            }
        }

        Ok(output)
    }
}
