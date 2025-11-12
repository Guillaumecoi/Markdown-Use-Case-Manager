// Application service for use case operations
// This orchestrates domain services and infrastructure
use crate::config::Config;
use crate::core::application::generators::{MarkdownGenerator, OverviewGenerator, TestGenerator};
use crate::core::utils::suggest_alternatives;
use crate::core::{
    file_operations::FileOperations, TemplateEngine, TomlUseCaseRepository, UseCase,
    UseCaseRepository, UseCaseService,
};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Application service that coordinates use case operations
/// This replaces the old UseCaseCoordinator with clean architecture
pub struct UseCaseApplicationService {
    config: Config,
    use_case_service: UseCaseService,
    repository: Box<dyn UseCaseRepository>,
    file_operations: FileOperations,
    template_engine: TemplateEngine,
    use_cases: Vec<UseCase>,
    markdown_generator: MarkdownGenerator,
    test_generator: TestGenerator,
    overview_generator: OverviewGenerator,
}

impl UseCaseApplicationService {
    pub fn new(config: Config) -> Result<Self> {
        let use_case_service = UseCaseService::new();
        let repository: Box<dyn UseCaseRepository> =
            Box::new(TomlUseCaseRepository::new(config.clone()));
        let file_operations = FileOperations::new(config.clone());
        let template_engine = TemplateEngine::with_config(Some(&config));
        
        // Create generators
        let markdown_generator = MarkdownGenerator::new(config.clone());
        let test_generator = TestGenerator::new(config.clone());
        let overview_generator = OverviewGenerator::new(config.clone());

        let use_cases = repository.load_all()?;

        Ok(Self {
            config,
            use_case_service,
            repository,
            file_operations,
            template_engine,
            use_cases,
            markdown_generator,
            test_generator,
            overview_generator,
        })
    }

    pub fn load() -> Result<Self> {
        let config = Config::load()?;
        Self::new(config)
    }

    // ========== Public API Methods ==========

    /// Get all use cases (for display)
    pub fn get_all_use_cases(&self) -> &[UseCase] {
        &self.use_cases
    }

    /// Get all use case IDs from repository
    pub fn get_all_use_case_ids(&self) -> Result<Vec<String>> {
        Ok(self.use_cases.iter().map(|uc| uc.id.clone()).collect())
    }

    /// Get all categories in use
    pub fn get_all_categories(&self) -> Result<Vec<String>> {
        let mut categories: Vec<String> = self
            .use_cases
            .iter()
            .map(|uc| uc.category.clone())
            .collect();

        categories.sort();
        categories.dedup();
        Ok(categories)
    }

    // ========== Methodology Management ==========

    /// Create a use case with specific methodology
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
    ) -> Result<String> {
        // Validate methodology exists
        let available_methodologies = self.template_engine.available_methodologies();
        if !available_methodologies.contains(&methodology.to_string()) {
            return Err(anyhow::anyhow!(
                "Unknown methodology '{}'. Available: {:?}",
                methodology,
                available_methodologies
            ));
        }

        let use_case =
            self.create_use_case_with_methodology_internal(title, category, description, methodology)?;
        let use_case_id = use_case.id.clone();

        // Save the use case with methodology-specific rendering
        self.save_use_case_with_methodology(&use_case, methodology)?;
        self.use_cases.push(use_case);
        self.generate_overview()?;

        Ok(use_case_id)
    }

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: &str,
        methodology: &str,
    ) -> Result<()> {
        // Find the use case
        let use_case = match self.use_cases.iter().find(|uc| uc.id == use_case_id) {
            Some(uc) => uc.clone(),
            None => {
                // Get available use case IDs for suggestions
                let available_ids: Vec<String> =
                    self.use_cases.iter().map(|uc| uc.id.clone()).collect();
                let error_msg = suggest_alternatives(use_case_id, &available_ids, "Use case");
                return Err(anyhow::anyhow!("{}", error_msg));
            }
        };

        // Validate methodology exists
        let available_methodologies = self.template_engine.available_methodologies();
        if !available_methodologies.contains(&methodology.to_string()) {
            return Err(anyhow::anyhow!(
                "Unknown methodology '{}'. Available: {:?}",
                methodology,
                available_methodologies
            ));
        }

        // Regenerate with new methodology
        self.save_use_case_with_methodology(&use_case, methodology)?;

        Ok(())
    }

    /// Regenerate markdown for a single use case
    pub fn regenerate_markdown(&self, use_case_id: &str) -> Result<()> {
        // Load use case from TOML (source of truth)
        let use_case = match self.repository.load_by_id(use_case_id)? {
            Some(uc) => uc,
            None => {
                // Get available use case IDs for suggestions
                let available_ids: Vec<String> =
                    self.use_cases.iter().map(|uc| uc.id.clone()).collect();
                let error_msg = suggest_alternatives(use_case_id, &available_ids, "Use case");
                return Err(anyhow::anyhow!("{}", error_msg));
            }
        };

        // Generate markdown from TOML data
        let markdown_content = self.generate_use_case_markdown(&use_case)?;
        self.repository
            .save_markdown_only(use_case_id, &markdown_content)?;

        Ok(())
    }

    /// Regenerate markdown for all use cases
    pub fn regenerate_all_markdown(&self) -> Result<()> {
        // Load all use cases from TOML (source of truth)
        let use_cases = self.repository.load_all()?;

        for use_case in &use_cases {
            // Generate markdown from TOML data
            let markdown_content = self.generate_use_case_markdown(use_case)?;
            self.repository
                .save_markdown_only(&use_case.id, &markdown_content)?;
        }

        self.generate_overview()?;
        Ok(())
    }

    // ========== Private Helper Methods ==========

    /// Internal helper to create use cases
    fn create_use_case_internal(
        &self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<UseCase> {
        let use_case_id = self.use_case_service.generate_unique_use_case_id(
            &category,
            &self.use_cases,
            &self.config.directories.use_case_dir,
        );
        let description = description.unwrap_or_default();

        let use_case = self
            .use_case_service
            .create_use_case(use_case_id.clone(), title, category, description)
            .map_err(|e| anyhow::anyhow!(e))?;

        // Step 1: Save TOML first (source of truth)
        self.repository.save_toml_only(&use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = self
            .repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        // Step 3: Generate markdown from TOML data
        let markdown_content = self.generate_use_case_markdown(&use_case_from_toml)?;
        self.repository
            .save_markdown_only(&use_case.id, &markdown_content)?;

        Ok(use_case)
    }

    /// Internal helper to create use cases with methodology custom fields
    fn create_use_case_with_methodology_internal(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
    ) -> Result<UseCase> {
        let use_case_id = self.use_case_service.generate_unique_use_case_id(
            &category,
            &self.use_cases,
            &self.config.directories.use_case_dir,
        );
        let description = description.unwrap_or_default();

        // Load methodology definition to get custom fields
        use crate::core::{Methodology, MethodologyRegistry};
        let templates_dir = self
            .config
            .directories
            .template_dir
            .clone()
            .unwrap_or_else(|| {
                format!(
                    ".config/.mucm/{}",
                    crate::config::Config::TEMPLATES_DIR
                )
            });
        
        // MethodologyRegistry expects the parent directory containing "methodologies/"
        // But during template copying, methodologies are placed directly in template-assets/
        // Check if methodology directory exists directly in templates_dir
        let methodology_dir = Path::new(&templates_dir).join(methodology);
        let extra_fields = if methodology_dir.exists() && methodology_dir.join("config.toml").exists() {
            // Load methodology definition directly
            use crate::core::MethodologyDefinition;
            match MethodologyDefinition::from_toml(&methodology_dir) {
                Ok(methodology_def) => {
                    let mut fields = HashMap::new();
                    for (field_name, field_config) in methodology_def.custom_fields() {
                        // Use default value if provided, otherwise use empty string for required fields
                        let value = if let Some(default) = &field_config.default {
                            serde_json::Value::String(default.clone())
                        } else if field_config.required {
                            // For required fields without defaults, use empty string
                            serde_json::Value::String(String::new())
                        } else {
                            // For optional fields without defaults, use null
                            serde_json::Value::Null
                        };
                        fields.insert(field_name.clone(), value);
                    }
                    fields
                }
                Err(_) => HashMap::new(),
            }
        } else {
            // Try standard structure: template-assets/methodologies/feature/
            let methodology_registry = MethodologyRegistry::new_dynamic(&templates_dir)?;
            if let Some(methodology_def) = methodology_registry.get(methodology) {
                let mut fields = HashMap::new();
                for (field_name, field_config) in methodology_def.custom_fields() {
                    // Use default value if provided, otherwise use empty string for required fields
                    let value = if let Some(default) = &field_config.default {
                        serde_json::Value::String(default.clone())
                    } else if field_config.required {
                        // For required fields without defaults, use empty string
                        serde_json::Value::String(String::new())
                    } else {
                        // For optional fields without defaults, use null
                        serde_json::Value::Null
                    };
                    fields.insert(field_name.clone(), value);
                }
                fields
            } else {
                HashMap::new()
            }
        };

        let use_case = self
            .use_case_service
            .create_use_case_with_extra(
                use_case_id.clone(),
                title,
                category,
                description,
                extra_fields,
            )
            .map_err(|e| anyhow::anyhow!(e))?;

        // Step 1: Save TOML first (source of truth) - this will include custom fields
        self.repository.save_toml_only(&use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = self
            .repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        // Step 3: Generate markdown from TOML data
        let markdown_content = self.generate_use_case_markdown(&use_case_from_toml)?;
        self.repository
            .save_markdown_only(&use_case.id, &markdown_content)?;

        Ok(use_case)
    }

    /// Save use case with specific methodology rendering
    fn save_use_case_with_methodology(&self, use_case: &UseCase, methodology: &str) -> Result<()> {
        // Step 1: Save TOML first (source of truth)
        self.repository.save_toml_only(use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = self
            .repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load use case from TOML"))?;

        // Step 3: Generate methodology-specific markdown from TOML data
        let markdown_content =
            self.generate_use_case_markdown_with_methodology(&use_case_from_toml, methodology)?;
        self.repository
            .save_markdown_only(&use_case.id, &markdown_content)?;

        // Generate test file if enabled
        if self.config.generation.auto_generate_tests {
            self.generate_test_file(&use_case_from_toml)?;
        }

        Ok(())
    }

    /// Helper to generate use case markdown
    fn generate_use_case_markdown(&self, use_case: &UseCase) -> Result<String> {
        self.markdown_generator.generate(use_case)
    }

    /// Helper to generate use case markdown with specific methodology
    fn generate_use_case_markdown_with_methodology(
        &self,
        use_case: &UseCase,
        methodology: &str,
    ) -> Result<String> {
        self.markdown_generator.generate_with_methodology(use_case, methodology)
    }

    /// Generate test file for a use case
    fn generate_test_file(&self, use_case: &UseCase) -> Result<()> {
        self.test_generator.generate(use_case)
    }

    /// Generate overview file
    fn generate_overview(&self) -> Result<()> {
        self.overview_generator.generate(&self.use_cases)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::application::testing::test_helpers::init_test_project;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_interactive_workflow_simulation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(None)?;

        let mut coordinator = UseCaseApplicationService::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let use_case_id = coordinator.create_use_case_with_methodology(
            "Interactive Test".to_string(),
            "testing".to_string(),
            Some("Created via interactive mode".to_string()),
            &default_methodology,
        )?;
        assert_eq!(use_case_id, "UC-TES-001");

        let use_case_ids = coordinator.get_all_use_case_ids()?;
        assert_eq!(use_case_ids.len(), 1);
        assert!(use_case_ids.contains(&"UC-TES-001".to_string()));

        let final_use_case_ids = coordinator.get_all_use_case_ids()?;
        assert_eq!(final_use_case_ids.len(), 1);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_interactive_category_suggestions() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(None)?;
        let mut coordinator = UseCaseApplicationService::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let categories = coordinator.get_all_categories()?;
        assert!(categories.is_empty());

        coordinator.create_use_case_with_methodology(
            "Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            &default_methodology,
        )?;

        coordinator.create_use_case_with_methodology(
            "API Use Case".to_string(),
            "api".to_string(),
            None,
            &default_methodology,
        )?;

        coordinator.create_use_case_with_methodology(
            "Another Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            &default_methodology,
        )?;

        let categories = coordinator.get_all_categories()?;
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0], "api");
        assert_eq!(categories[1], "authentication");

        let use_case_ids = coordinator.get_all_use_case_ids()?;
        assert_eq!(use_case_ids.len(), 3);
        assert!(use_case_ids.contains(&"UC-AUT-001".to_string()));
        assert!(use_case_ids.contains(&"UC-API-001".to_string()));
        assert!(use_case_ids.contains(&"UC-AUT-002".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_complete_interactive_workflow_simulation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(Some("rust".to_string()))?;

        let mut coordinator = UseCaseApplicationService::load()?;
        let default_methodology = coordinator.config.templates.default_methodology.clone();

        let _uc1 = coordinator.create_use_case_with_methodology(
            "User Authentication".to_string(),
            "auth".to_string(),
            Some("Handle user login and logout".to_string()),
            &default_methodology,
        )?;

        let _uc2 = coordinator.create_use_case_with_methodology(
            "Data Export".to_string(),
            "api".to_string(),
            Some("Export data in various formats".to_string()),
            &default_methodology,
        )?;

        let all_use_cases = coordinator.get_all_use_case_ids()?;
        assert_eq!(all_use_cases.len(), 2);

        let categories = coordinator.get_all_categories()?;
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"api".to_string()));
        assert!(categories.contains(&"auth".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_custom_fields_end_to_end_flow() -> Result<()> {
        // Skip test if source templates can't be found
        // This can happen when running all tests together
        if crate::config::TemplateManager::find_source_templates_dir().is_err() {
            eprintln!("SKIPPING test_custom_fields_end_to_end_flow: source templates not available");
            return Ok(());
        }

        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf();
        env::set_current_dir(&temp_path)?;

        // Initialize project with templates (includes feature methodology with custom fields)
        init_test_project(None)?;

        // Verify templates were copied - if not, fail with clear message
        let templates_dir = Path::new(".config/.mucm/template-assets");
        if !templates_dir.exists() {
            anyhow::bail!("Templates were not copied. Template dir {:?} doesn't exist", templates_dir);
        }
        
        let feature_dir = templates_dir.join("feature");
        if !feature_dir.exists() {
            anyhow::bail!("Feature methodology not found at {:?}", feature_dir);
        }

        // Use the existing "feature" methodology which already has custom fields defined
        // (user_story, acceptance_criteria, epic_link, sprint, story_points, etc.)
        let mut coordinator = UseCaseApplicationService::load()?;

        let use_case_id = coordinator.create_use_case_with_methodology(
            "Test Custom Fields".to_string(),
            "testing".to_string(),
            Some("Testing custom fields integration".to_string()),
            "feature",
        )?;

        // Verify the use case was created
        assert_eq!(use_case_id, "UC-TES-001");

        // Load the use case from TOML to verify custom fields were saved
        let loaded_use_case = coordinator
            .repository
            .load_by_id(&use_case_id)?
            .expect("Use case should exist");

        // Verify custom fields from feature methodology are present
        // Check for required fields (from source-templates/methodologies/feature/config.toml)
        assert!(
            loaded_use_case.extra.contains_key("user_story"),
            "user_story should be present (required field). Found keys: {:?}",
            loaded_use_case.extra.keys().collect::<Vec<_>>()
        );
        assert!(
            loaded_use_case.extra.contains_key("acceptance_criteria"),
            "acceptance_criteria should be present (required field)"
        );

        // Note: Optional fields with null/empty values are not saved to TOML
        // This is intentional - TOML doesn't support null values like JSON does
        // Optional fields will only appear in the loaded use case if they have actual values

        // Verify TOML file exists and contains custom fields
        let toml_dir = Path::new(coordinator.config.directories.get_toml_dir()).join("testing");
        let toml_path = toml_dir.join("UC-TES-001.toml");
        assert!(
            toml_path.exists(),
            "TOML file should exist at {:?}",
            toml_path
        );

        let toml_content = fs::read_to_string(&toml_path)?;
        assert!(
            toml_content.contains("user_story"),
            "TOML should contain user_story field"
        );
        assert!(
            toml_content.contains("acceptance_criteria"),
            "TOML should contain acceptance_criteria field"
        );

        // Verify markdown was generated
        let md_path = Path::new(&coordinator.config.directories.use_case_dir)
            .join("testing")
            .join("UC-TES-001.md");
        assert!(md_path.exists(), "Markdown file should exist at {:?}", md_path);

        let md_content = fs::read_to_string(&md_path)?;
        assert!(
            md_content.contains("Test Custom Fields"),
            "Markdown should contain title"
        );

        Ok(())
    }
}
