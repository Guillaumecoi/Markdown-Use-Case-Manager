use crate::config::Config;
use crate::core::languages::LanguageRegistry;
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

    /// Initialize a new use case manager project
    #[allow(clippy::unused_self)]
    pub fn init_project(&mut self, language: Option<String>, methodology: Option<String>) -> Result<String> {
        let config = if let Some(method) = methodology {
            // Create config with methodology-specific recommendations
            let mut config = Config::new_with_methodology(&method);
            if let Some(lang) = language {
                config.generation.test_language = lang;
            }
            Config::init_project_with_config(config)?
        } else {
            Config::init_project_with_language(language)?
        };
        
        let recommendations = if let Some(method) = &config.templates.methodology {
            format!("\n\n{}", Config::methodology_recommendations(method))
        } else {
            String::new()
        };

        Ok(format!(
            "Project initialized! Configuration saved to .config/.mucm/mucm.toml\n\
             Feel free to edit the configuration file to customize your setup.\n\
             Unless changed, use cases will be stored in: {}{}",
            config.directories.use_case_dir,
            recommendations
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
                output.push_str("To add a new language manually, create a directory: .config/.mucm/templates/lang-<language>/\n");
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
    pub fn list_methodologies(&mut self) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        let methodologies = coordinator.list_available_methodologies();
        
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
    pub fn get_methodology_info(&mut self, methodology: String) -> Result<String> {
        let coordinator = self.ensure_coordinator()?;
        
        match coordinator.get_methodology_info(&methodology) {
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
