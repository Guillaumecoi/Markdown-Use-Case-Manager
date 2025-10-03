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
        Ok(self.coordinator.as_mut().unwrap())
    }

    /// Initialize a new use case manager project
    pub fn init_project(&mut self, language: Option<String>) -> Result<String> {
        let config = Config::init_project_with_language(language)?;
        Ok(format!(
            "Project initialized! Configuration saved to .config/.mucm/mucm.toml\n\
             Feel free to edit the configuration file to customize your setup.\n\
             Unless changed, use cases will be stored in: {}",
            config.directories.use_case_dir
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
}
