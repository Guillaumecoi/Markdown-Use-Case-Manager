// Application service for use case operations
// This orchestrates domain services and infrastructure
use crate::config::Config;
use crate::core::{
    file_operations::FileOperations, to_snake_case, TemplateEngine, TomlUseCaseRepository,
    UseCase, UseCaseRepository, UseCaseService,
};
use crate::core::utils::suggest_alternatives;
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Application service that coordinates use case operations
/// This replaces the old UseCaseCoordinator with clean architecture
pub struct UseCaseApplicationService {
    config: Config,
    use_case_service: UseCaseService,
    repository: Box<dyn UseCaseRepository>,
    file_operations: FileOperations,
    template_engine: TemplateEngine,
    use_cases: Vec<UseCase>,
}

impl UseCaseApplicationService {
    pub fn new(config: Config) -> Result<Self> {
        let use_case_service = UseCaseService::new();
        let repository: Box<dyn UseCaseRepository> =
            Box::new(TomlUseCaseRepository::new(config.clone()));
        let file_operations = FileOperations::new(config.clone());
        let template_engine = TemplateEngine::with_config(Some(&config));

        let use_cases = repository.load_all()?;

        Ok(Self {
            config,
            use_case_service,
            repository,
            file_operations,
            template_engine,
            use_cases,
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

        let use_case = self.create_use_case_internal(title, category, description)?;
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

        let use_case = self.use_case_service.create_use_case(
            use_case_id.clone(),
            title,
            category,
            description,
        ).map_err(|e| anyhow::anyhow!(e))?;

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
        // Use the default methodology from config
        let default_methodology = &self.config.templates.default_methodology;
        self.generate_use_case_markdown_with_methodology(use_case, default_methodology)
    }

    /// Helper to generate use case markdown with specific methodology
    /// Converts UseCase to JSON and passes it directly to the template engine
    /// This allows templates to access ANY field from the TOML file without hardcoding
    fn generate_use_case_markdown_with_methodology(
        &self,
        use_case: &UseCase,
        methodology: &str,
    ) -> Result<String> {
        // Convert UseCase directly to JSON - templates can access any field from TOML
        let use_case_json = serde_json::to_value(use_case)?;

        // Convert to HashMap for template engine compatibility
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json)?;

        // Merge extra fields into top-level HashMap so templates can access them directly
        if let Some(Value::Object(extra_map)) = data.remove("extra") {
            for (key, value) in extra_map {
                data.insert(key, value);
            }
        }

        self.template_engine
            .render_use_case_with_methodology(&data, methodology)
    }

    /// Generate test file for a use case
    fn generate_test_file(&self, use_case: &UseCase) -> Result<()> {
        // Check if test file already exists and overwrite is disabled
        let file_extension = self.get_test_file_extension();
        if self
            .file_operations
            .test_file_exists(use_case, &file_extension)
            && !self.config.generation.overwrite_test_documentation
        {
            // Use the formatter to display the skipped message
            use crate::presentation::UseCaseFormatter;
            UseCaseFormatter::display_test_skipped();
            return Ok(());
        }

        // Generate test content using template
        let test_content = self.generate_test_content(use_case)?;

        // Save the test file
        self.file_operations
            .save_test_file(use_case, &test_content, &file_extension)?;

        // Get the test file path for display
        let test_file_path = self.get_test_file_path(use_case)?;

        // Use the formatter to display the generated message
        use crate::presentation::UseCaseFormatter;
        UseCaseFormatter::display_test_generated(
            &use_case.id,
            &test_file_path.display().to_string(),
        );

        Ok(())
    }

    /// Get the test file path for a use case
    fn get_test_file_path(&self, use_case: &UseCase) -> Result<std::path::PathBuf> {
        let test_dir = std::path::Path::new(&self.config.directories.test_dir);
        let category_dir = test_dir.join(to_snake_case(&use_case.category));
        let file_extension = self.get_test_file_extension();
        let file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        Ok(category_dir.join(file_name))
    }

    /// Generate test content for a use case
    fn generate_test_content(&self, use_case: &UseCase) -> Result<String> {
        // Convert UseCase to JSON for template engine
        let use_case_json = serde_json::to_value(use_case)?;
        let mut data: HashMap<String, Value> = serde_json::from_value(use_case_json)?;

        // Merge extra fields into top-level HashMap
        if let Some(Value::Object(extra_map)) = data.remove("extra") {
            for (key, value) in extra_map {
                data.insert(key, value);
            }
        }

        // Add generated timestamp
        data.insert(
            "generated_at".to_string(),
            json!(chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string()),
        );

        // Add snake_case version of title for class names
        if let Some(Value::String(title)) = data.get("title") {
            data.insert("title_snake_case".to_string(), json!(to_snake_case(title)));
        }

        // Render using test template for the configured language
        self.template_engine
            .render_test(&self.config.generation.test_language, &data)
    }

    /// Get the file extension for test files based on test language
    fn get_test_file_extension(&self) -> String {
        match self.config.generation.test_language.as_str() {
            "python" => "py".to_string(),
            "javascript" => "js".to_string(),
            "rust" => "rs".to_string(),
            _ => "txt".to_string(), // fallback
        }
    }

    /// Generate overview file
    fn generate_overview(&self) -> Result<()> {
        let mut data = HashMap::new();

        // Basic counts
        data.insert("total_use_cases".to_string(), json!(self.use_cases.len()));

        // Project name and generated date
        data.insert("project_name".to_string(), json!(self.config.project.name));
        data.insert(
            "generated_date".to_string(),
            json!(chrono::Utc::now().format("%Y-%m-%d").to_string()),
        );

        // Group use cases by category
        let mut categories_map: HashMap<String, Vec<serde_json::Map<String, Value>>> =
            HashMap::new();
        for uc in &self.use_cases {
            categories_map
                .entry(uc.category.clone())
                .or_default()
                .push({
                    let mut uc_data = serde_json::Map::new();
                    uc_data.insert("id".to_string(), json!(uc.id));
                    uc_data.insert("title".to_string(), json!(uc.title));
                    uc_data.insert(
                        "aggregated_status".to_string(),
                        json!(uc.status().display_name()),
                    );
                    uc_data.insert("priority".to_string(), json!(uc.priority.to_string()));
                    uc_data
                });
        }

        // Convert to array format expected by template
        let categories: Vec<serde_json::Map<String, Value>> = categories_map
            .into_iter()
            .map(|(category_name, use_cases)| {
                let mut cat = serde_json::Map::new();
                cat.insert("category_name".to_string(), json!(category_name));
                cat.insert("use_cases".to_string(), json!(use_cases));
                cat
            })
            .collect();

        data.insert("categories".to_string(), json!(categories));

        let overview_content = self.template_engine.render_overview(&data)?;
        self.file_operations.save_overview(&overview_content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    /// Helper to initialize a project for tests
    fn init_test_project(language: Option<String>) -> Result<Config> {
        use crate::core::LanguageRegistry;

        let config_dir = Path::new(".config/.mucm");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let mut config = Config::default();

        if let Some(ref lang) = language {
            let language_registry = LanguageRegistry::new();
            if let Some(lang_def) = language_registry.get(lang) {
                let primary_name = lang_def.name().to_string();
                config.generation.test_language = primary_name.clone();
                config.templates.test_language = primary_name.clone();
            } else {
                config.generation.test_language = lang.clone();
                config.templates.test_language = lang.clone();
            }
        }

        config.save_in_dir(".")?;
        Config::copy_templates_to_config_with_language(language)?;

        Ok(config)
    }

    #[test]
    #[serial]
    fn test_interactive_workflow_simulation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;

        init_test_project(None)?;

        let mut coordinator = UseCaseApplicationService::load()?;

        let use_case_id = coordinator.create_use_case_with_methodology(
            "Interactive Test".to_string(),
            "testing".to_string(),
            Some("Created via interactive mode".to_string()),
            "feature",
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

        let categories = coordinator.get_all_categories()?;
        assert!(categories.is_empty());

        coordinator.create_use_case_with_methodology(
            "Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            "feature",
        )?;

        coordinator.create_use_case_with_methodology(
            "API Use Case".to_string(),
            "api".to_string(),
            None,
            "feature",
        )?;

        coordinator.create_use_case_with_methodology(
            "Another Auth Use Case".to_string(),
            "authentication".to_string(),
            None,
            "feature",
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

        let _uc1 = coordinator.create_use_case_with_methodology(
            "User Authentication".to_string(),
            "auth".to_string(),
            Some("Handle user login and logout".to_string()),
            "feature",
        )?;

        let _uc2 = coordinator.create_use_case_with_methodology(
            "Data Export".to_string(),
            "api".to_string(),
            Some("Export data in various formats".to_string()),
            "feature",
        )?;

        let all_use_cases = coordinator.get_all_use_case_ids()?;
        assert_eq!(all_use_cases.len(), 2);

        let categories = coordinator.get_all_categories()?;
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"api".to_string()));
        assert!(categories.contains(&"auth".to_string()));

        Ok(())
    }
}
