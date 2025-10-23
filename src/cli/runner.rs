use crate::config::Config;
use crate::core::languages::LanguageRegistry;
use crate::core::template_engine::TemplateEngine;
use crate::core::use_case_coordinator::UseCaseCoordinator;
use anyhow::Result;

/// CLI runner that can be used both programmatically and interactively
pub struct CliRunner {
    coordinator: Option<UseCaseCoordinator>,
}

impl CliRunner {
    /// Create a new CLI runner
    pub fn new() -> Self {
        Self { coordinator: None }
    }

    /// Load or initialize the coordinator
    fn ensure_coordinator(&mut self) -> Result<&mut UseCaseCoordinator> {
        if self.coordinator.is_none() {
            self.coordinator = Some(UseCaseCoordinator::load()?);
        }
        // Safe unwrap: we just ensured coordinator is Some above
        Ok(self.coordinator.as_mut().expect("coordinator was just initialized"))
    }

    /// Initialize a new use case manager project (Step 1: Create config only)
    #[allow(clippy::unused_self)]
    pub fn init_project(&mut self, language: Option<String>, methodology: Option<String>) -> Result<String> {
        // Ensure we're not already in a project
        if Config::load().is_ok() {
            anyhow::bail!("A use case manager project already exists in this directory or a parent directory");
        }

        // Resolve language aliases to primary names
        let resolved_language = if let Some(lang) = language {
            let language_registry = LanguageRegistry::new();
            if let Some(lang_def) = language_registry.get(&lang) {
                // Use the primary name (not alias)
                lang_def.name().to_string()
            } else {
                // Keep original if not found in registry (might be user-defined)
                lang
            }
        } else {
            "rust".to_string() // Default language
        };

        // Create minimal config for template processing
        let config = Config::for_template(resolved_language, methodology);
        
        // Save config file only (no templates, no directories)
        Config::save_config_only(&config)?;
        
        let recommendations = if let Some(method) = &config.templates.default_methodology {
            format!("\n\n{}", Config::methodology_recommendations(method))
        } else {
            String::new()
        };

        Ok(format!(
            "✅ Configuration file created at .config/.mucm/mucm.toml\n\n\
             📝 Please review and customize the configuration:\n\
             - Programming language: {}\n\
             - Default Methodology: {}\n\
             - TOML directory: {}\n\
             - Use case directory: {}\n\
             - Test directory: {}\n\n\
             ⚡ When ready, run: mucm init --finalize{}\n\n\
             💡 The finalize step will:\n\
             - Copy ALL methodology templates (business, developer, feature, tester)\n\
             - Copy ALL language templates\n\
             - You can use any methodology when creating use cases\n\
             - Directories will be created when you create your first use case",
            config.generation.test_language,
            config.templates.default_methodology.as_deref().unwrap_or("developer"),
            config.directories.toml_dir.as_deref().unwrap_or("docs/use-cases"),
            config.directories.use_case_dir,
            config.directories.test_dir,
            recommendations
        ))
    }

    /// Finalize initialization (Step 2: Copy templates after config review)
    #[allow(clippy::unused_self)]
    pub fn finalize_init(&mut self) -> Result<String> {
        // Load the config that should have been created in step 1
        let config = Config::load().map_err(|_| {
            anyhow::anyhow!(
                "No configuration file found. Please run 'mucm init' first to create the configuration."
            )
        })?;

        // Check if already finalized (templates directory exists)
        if Config::check_templates_exist() {
            anyhow::bail!(
                "Project already finalized. Templates directory exists.\n\
                 If you want to re-copy templates, delete .config/.mucm/handlebars/ first."
            );
        }

        // Copy templates - now copies ALL methodologies and ALL languages
        Config::copy_templates_to_config_with_language(Some(config.generation.test_language.clone()))?;

        // List available methodologies
        let available = Config::list_available_methodologies().unwrap_or_default();
        let methodologies_list = if available.is_empty() {
            "Unable to detect".to_string()
        } else {
            available.join(", ")
        };

        Ok(format!(
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
             💡 Each methodology has its own settings (test generation, metadata, etc.)\n\
             💡 Directories will be created automatically when you create your first use case.",
            config.generation.test_language,
            config.templates.default_methodology.as_deref().unwrap_or("developer"),
            methodologies_list
        ))
    }


    /// Create a new use case
    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        let use_case_id = coordinator.create_use_case(title, category, description)?;
        Ok(format!(
            "Created use case: {}\n\n💡 Tip: Use this exact ID ('{}') when adding scenarios or updating status.",
            use_case_id, use_case_id
        ))
    }

    /// Create a new use case with extended metadata
    pub fn create_use_case_with_metadata(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        _extended_metadata: crate::cli::interactive::menu::ExtendedMetadata,
    ) -> Result<String> {
        // Note: Extended metadata functionality removed as it's now handled by TOML files
        let coordinator = self.ensure_coordinator()?;
        let use_case_id = coordinator.create_use_case(
            title, 
            category, 
            description,
        )?;
        Ok(format!("Created use case: {}", use_case_id))
    }

    /// Add a scenario to a use case
    pub fn add_scenario(
        &mut self,
        use_case_id: String,
        title: String,
        description: Option<String>,
    ) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        let scenario_id = coordinator.add_scenario_to_use_case(use_case_id, title, description)?;
        Ok(format!("Added scenario: {}", scenario_id))
    }

    /// Update scenario status
    pub fn update_scenario_status(
        &mut self,
        scenario_id: String,
        status: String,
    ) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.update_scenario_status(scenario_id.clone(), status.clone())?;
        Ok(format!(
            "Updated scenario {} status to {}",
            scenario_id, status
        ))
    }

    /// List all use cases
    pub fn list_use_cases(&mut self) -> Result<()> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.list_use_cases()
    }

    /// Show available languages
    #[allow(clippy::unnecessary_wraps)]
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

    /// Show project status
    pub fn show_status(&mut self) -> Result<()> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.show_status()
    }

    /// Get all use case IDs for selection
    pub fn get_use_case_ids(&mut self) -> Result<Vec<String>> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.get_all_use_case_ids()
    }

    /// Get all scenario IDs for a specific use case
    pub fn get_scenario_ids(&mut self, use_case_id: &str) -> Result<Vec<String>> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.get_scenario_ids_for_use_case(use_case_id)
    }

    /// Get all categories in use
    pub fn get_categories(&mut self) -> Result<Vec<String>> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.get_all_categories()
    }
    /// Create a new use case with specific methodology
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: String,
    ) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        let use_case_id = coordinator.create_use_case_with_methodology(title, category, description, &methodology)?;
        Ok(format!("Created use case: {} with {} methodology", use_case_id, methodology))
    }

    /// List available methodologies
    #[allow(clippy::unused_self)]
    pub fn list_methodologies(&mut self) -> Result<String> {
        // Create a temporary template engine just to list methodologies
        // This doesn't require a full project initialization
        let template_engine = TemplateEngine::new()
            .map_err(|e| anyhow::anyhow!("Failed to create template engine: {}", e))?;
        let methodologies = template_engine.available_methodologies();
        
        if methodologies.is_empty() {
            return Ok("No methodologies available.".to_string());
        }
        
        let mut result = String::from("Available methodologies:\n");
        for methodology in methodologies {
            result.push_str(&format!("  - {}\n", methodology));
        }
        
        Ok(result)
    }

    /// Get information about a specific methodology
    #[allow(clippy::unused_self)]
    pub fn get_methodology_info(&mut self, methodology: String) -> Result<String> {
        // Create a temporary template engine just to get methodology info
        // This doesn't require a full project initialization
        let template_engine = TemplateEngine::new()
            .map_err(|e| anyhow::anyhow!("Failed to create template engine: {}", e))?;
        
        match template_engine.get_methodology_info(&methodology) {
            Some((name, description)) => {
                Ok(format!("Methodology: {}\nDescription: {}", name, description))
            }
            None => {
                Ok(format!("Methodology '{}' not found.", methodology))
            }
        }
    }

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.regenerate_use_case_with_methodology(&use_case_id, &methodology)?;
        Ok(format!("Regenerated use case {} with {} methodology", use_case_id, methodology))
    }

    /// Regenerate markdown for a single use case from its TOML file
    pub fn regenerate_use_case(&mut self, use_case_id: &str) -> Result<()> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.regenerate_markdown(use_case_id)
    }

    /// Regenerate markdown for all use cases from their TOML files
    pub fn regenerate_all_use_cases(&mut self) -> Result<()> {
        let coordinator = self.ensure_coordinator()?;
        coordinator.regenerate_all_markdown()
    }
}
