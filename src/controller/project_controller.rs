use anyhow::Result;

use super::dto::{DisplayResult, MethodologyInfo, SelectionOptions};
use crate::config::Config;
use crate::core::LanguageRegistry;

/// Controller for project initialization and management
pub struct ProjectController;

impl ProjectController {
    /// Check if a project is already initialized
    pub fn is_initialized() -> bool {
        Config::load().is_ok()
    }

    /// Get available programming languages for selection prompts
    /// TODO: Use this in interactive init workflow for language selection
    #[allow(dead_code)]
    pub fn get_available_languages() -> Result<SelectionOptions> {
        let languages = Config::get_available_languages()?;
        Ok(SelectionOptions::new(languages))
    }

    /// Get available methodologies with descriptions
    pub fn get_available_methodologies() -> Result<Vec<MethodologyInfo>> {
        let methodologies = Config::list_available_methodologies()?;

        let methodology_infos: Vec<MethodologyInfo> = methodologies
            .into_iter()
            .map(|name| {
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

                let description = match name.as_str() {
                    "business" => "Business-focused use cases with actors and goals",
                    "developer" => "Technical use cases for development teams",
                    "feature" => "Feature-oriented use case documentation",
                    "tester" => "QA and testing-focused use cases",
                    _ => "Custom methodology",
                };

                MethodologyInfo {
                    name,
                    display_name,
                    description: description.to_string(),
                }
            })
            .collect();

        Ok(methodology_infos)
    }

    /// Initialize a new project (Step 1: Create config)
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
            let language_registry = LanguageRegistry::new();
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

        let recommendations =
            Config::methodology_recommendations(&config.templates.default_methodology);

        let message = format!(
            "✅ Configuration file created at .config/.mucm/mucm.toml\n\n\
             📝 Please review and customize the configuration:\n\
             - Programming language: {}\n\
             - Default Methodology: {}\n\
             - TOML directory: {}\n\
             - Use case directory: {}\n\
             - Test directory: {}\n\n\
             ⚡ When ready, run: mucm init --finalize\n\n\
             {}\n\n\
             💡 The finalize step will:\n\
             - Copy the used methodology templates\n\
             - Copy the used language templates\n\
             - You can use any methodology when creating use cases\n\
             - Directories will be created when you create your first use case",
            config.templates.test_language,
            &config.templates.default_methodology,
            config
                .directories
                .toml_dir
                .as_deref()
                .unwrap_or("docs/use-cases"),
            config.directories.use_case_dir,
            config.directories.test_dir,
            recommendations
        );

        Ok(DisplayResult::success(message))
    }

    /// Finalize project initialization (Step 2: Copy templates)
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
            config.templates.test_language.clone(),
        ))?;

        // Get available methodologies
        let available = Config::list_available_methodologies().unwrap_or_default();
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
            config.templates.test_language,
            &config.templates.default_methodology,
            methodologies_list
        );

        Ok(DisplayResult::success(message))
    }

    /// Get the default methodology from config
    /// TODO: Use this in interactive mode to pre-select default methodology
    #[allow(dead_code)]
    pub fn get_default_methodology() -> Result<String> {
        let config = Config::load()?;
        Ok(config.templates.default_methodology.clone())
    }

    /// Get available languages as a formatted string (for display)
    pub fn show_languages() -> Result<String> {
        let mut output = String::from("Available programming languages:\n");

        match Config::get_available_languages() {
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
                let language_registry = LanguageRegistry::new();
                let builtin_languages = language_registry.available_languages();
                output.push_str(&format!(
                    "Built-in languages: {}\n",
                    builtin_languages.join(", ")
                ));
            }
        }

        Ok(output)
    }
}
