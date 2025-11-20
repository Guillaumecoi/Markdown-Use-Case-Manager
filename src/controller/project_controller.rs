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
use crate::core::{DocumentationLevel, LanguageRegistry, Methodology, MethodologyRegistry};

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
    /// Get all available methodologies from source templates.
    /// This is used during initialization to show what can be installed.
    pub fn get_available_methodologies() -> Result<Vec<MethodologyInfo>> {
        use crate::config::Config;

        // Load methodology metadata (info.toml) from source templates
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

    /// Get installed/configured methodologies in the current project.
    /// This is used when creating use cases to show only what's configured.
    pub fn get_installed_methodologies() -> Result<Vec<MethodologyInfo>> {
        use crate::config::Config;
        use std::fs;

        // Check what's actually installed in project templates directory
        let project_templates_dir = Config::get_project_templates_dir()?;
        let methodologies_dir = project_templates_dir.join("methodologies");

        if !methodologies_dir.exists() {
            anyhow::bail!(
                "Project methodologies directory not found. Run 'mucm init --finalize' first."
            );
        }

        // Read directory to find installed methodologies
        let installed: Vec<String> = fs::read_dir(&methodologies_dir)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_dir() {
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect();

        if installed.is_empty() {
            anyhow::bail!(
                "No methodologies installed. Run 'mucm init --finalize' to copy methodology templates."
            );
        }

        // Load methodology metadata (info.toml) from project templates
        let registry = MethodologyRegistry::new_dynamic(&project_templates_dir)?;

        // Build info for installed methodologies
        let methodology_infos: Vec<MethodologyInfo> = installed
            .iter()
            .filter_map(|name| {
                registry.get(name).map(|methodology_def| {
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
            })
            .collect();

        Ok(methodology_infos)
    }

    /// Get available levels for a specific methodology
    pub fn get_methodology_levels(methodology_name: &str) -> Result<Vec<DocumentationLevel>> {
        use crate::config::Config;

        // Load methodology metadata from project-installed templates
        // This allows users to customize levels and templates per project
        let templates_dir = Config::get_project_templates_dir()?;
        let registry = MethodologyRegistry::new_dynamic(&templates_dir)?;

        let methodology_def = registry
            .get(methodology_name)
            .ok_or_else(|| anyhow::anyhow!("Methodology '{}' not found", methodology_name))?;

        Ok(methodology_def.levels().to_vec())
    }

    /// Initialize a new project (config + templates + directories).
    ///
    /// Creates the project configuration, copies templates, and creates directories in one step.
    /// Uses defaults for any missing parameters (like the normal init process).
    ///
    /// # Arguments
    /// * `language` - Optional programming language for test generation (default: "none")
    /// * `methodologies` - Optional list of methodologies to enable (default: all available)
    /// * `storage` - Optional storage backend (default: "toml")
    /// * `default_methodology` - Optional default methodology (default: first methodology or "feature")
    /// * `use_case_dir` - Optional directory for use case files (default: "docs/use-cases")
    /// * `test_dir` - Optional directory for test files (default: "tests/use-cases")
    /// * `persona_dir` - Optional directory for persona files (default: "docs/personas")
    /// * `data_dir` - Optional directory for data files (default: "use-cases-data")
    ///
    /// # Returns
    /// DisplayResult with completion message and usage guidance
    ///
    /// # Errors
    /// Returns error if project is already initialized or initialization fails
    pub fn init_project(
        language: Option<String>,
        methodologies: Option<Vec<String>>,
        storage: Option<String>,
        default_methodology: Option<String>,
        use_case_dir: Option<String>,
        test_dir: Option<String>,
        persona_dir: Option<String>,
        data_dir: Option<String>,
    ) -> Result<DisplayResult> {
        // Check if already initialized
        if Self::is_initialized() {
            return Ok(DisplayResult::error(
                "A use case manager project already exists in this directory or a parent directory"
                    .to_string(),
            ));
        }

        // Resolve language aliases to primary names (default: "none")
        let resolved_language = if let Some(lang) = language {
            use crate::config::Config;

            // Handle special case for "none"
            if lang == "none" {
                "none".to_string()
            } else {
                // Always load language metadata (info.toml) from source templates
                let templates_dir = Config::get_metadata_load_dir()?;
                let language_registry = LanguageRegistry::new_dynamic(&templates_dir)?;
                if let Some(lang_def) = language_registry.get(&lang) {
                    lang_def.name().to_string()
                } else {
                    lang
                }
            }
        } else {
            "none".to_string()
        };

        // Get available methodologies if none specified
        let resolved_methodologies = if let Some(meths) = methodologies {
            meths
        } else {
            // Load all available methodologies as default
            use crate::config::Config;
            let templates_dir = Config::get_metadata_load_dir()?;
            MethodologyRegistry::discover_available(&templates_dir).unwrap_or_default()
        };

        // Default to "toml" if storage not specified
        let resolved_storage = storage.unwrap_or_else(|| "toml".to_string());

        // Default methodology: use provided, or first methodology, or "feature"
        let resolved_default_methodology = if let Some(default) = default_methodology {
            default
        } else if !resolved_methodologies.is_empty() {
            resolved_methodologies[0].clone()
        } else {
            "feature".to_string()
        };

        // Use defaults for directories
        let resolved_use_case_dir = use_case_dir.unwrap_or_else(|| "docs/use-cases".to_string());
        let resolved_test_dir = test_dir.unwrap_or_else(|| "tests/use-cases".to_string());
        let resolved_persona_dir = persona_dir.unwrap_or_else(|| "docs/personas".to_string());
        let resolved_data_dir = data_dir.unwrap_or_else(|| "use-cases-data".to_string());

        // Create config with resolved parameters
        let config = Config::for_template_with_methodologies_storage_and_directories(
            Some(resolved_language.clone()),
            resolved_methodologies.clone(),
            Some(resolved_default_methodology.clone()),
            resolved_storage.clone(),
            resolved_use_case_dir.clone(),
            resolved_test_dir.clone(),
            resolved_persona_dir.clone(),
            resolved_data_dir.clone(),
        );

        // Save config file
        Config::save_config_only(&config)?;

        // Immediately finalize (copy templates) with force=true
        Self::finalize_init_internal(true)?;

        // Create all project directories
        Config::create_project_directories()?;

        // Get available methodologies for display
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
             � Storage Backend: {}\n\
             �📚 Default Methodology: {}\n\
             📋 Available Methodologies: {}\n\n\
             🚀 You're ready to create use cases!\n\
             - Run: mucm create --category <category> \"<title>\" --methodology <name>\n\
             - Run: mucm list to see all use cases\n\
             - Run: mucm methodologies to see all available methodologies\n\
             - Run: mucm --help for all available commands\n\n\
             💡 Each methodology has its own settings (test generation, metadata, etc.)\n\n\
             📁 Project directories created:\n\
             - {} (use cases)\n\
             - {} (tests)\n\
             - {} (personas)\n\
             - {} (data)",
            resolved_language,
            resolved_storage,
            resolved_default_methodology,
            methodologies_list,
            resolved_use_case_dir,
            resolved_test_dir,
            resolved_persona_dir,
            resolved_data_dir,
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
        Self::finalize_init_internal(false)
    }

    /// Internal finalize with force option (public for use by interactive mode)
    pub fn finalize_init_internal(force: bool) -> Result<DisplayResult> {
        use std::fs;

        // Check if config exists
        let config = Config::load().map_err(|_| {
            anyhow::anyhow!("No configuration file found. Please run 'mucm init' first to create the configuration.")
        })?;

        // Check if already finalized (unless forced)
        if !force && Config::check_templates_exist() {
            return Ok(DisplayResult::error(
                "Project already finalized. Templates directory exists.\n\
                 If you want to re-copy templates, delete .config/.mucm/handlebars/ first."
                    .to_string(),
            ));
        }

        // Delete existing templates if forcing recopy
        if force && Config::check_templates_exist() {
            let templates_path = std::path::Path::new(".config/.mucm/template-assets");
            if templates_path.exists() {
                fs::remove_dir_all(templates_path)?;
            }
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
