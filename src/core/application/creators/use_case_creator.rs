use crate::config::Config;
use crate::core::{
    Methodology, MethodologyDefinition, MethodologyRegistry, UseCase, UseCaseRepository,
    UseCaseService,
};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

/// Handles use case creation with methodology support
pub struct UseCaseCreator {
    config: Config,
    use_case_service: UseCaseService,
}

impl UseCaseCreator {
    pub fn new(config: Config) -> Self {
        Self {
            use_case_service: UseCaseService::new(),
            config,
        }
    }

    /// Create a basic use case without methodology-specific fields
    pub fn create_use_case(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        existing_use_cases: &[UseCase],
        repository: &dyn UseCaseRepository,
    ) -> Result<UseCase> {
        let use_case_id = self.use_case_service.generate_unique_use_case_id(
            &category,
            existing_use_cases,
            &self.config.directories.use_case_dir,
        );
        let description = description.unwrap_or_default();

        let use_case = self
            .use_case_service
            .create_use_case(use_case_id.clone(), title, category, description)
            .map_err(|e| anyhow::anyhow!(e))?;

        // Step 1: Save TOML first (source of truth)
        repository.save(&use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        Ok(use_case_from_toml)
    }

    /// Create a use case with methodology-specific custom fields
    pub fn create_use_case_with_methodology(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: &str,
        existing_use_cases: &[UseCase],
        repository: &dyn UseCaseRepository,
    ) -> Result<UseCase> {
        let use_case_id = self.use_case_service.generate_unique_use_case_id(
            &category,
            existing_use_cases,
            &self.config.directories.use_case_dir,
        );
        let description = description.unwrap_or_default();

        // Load methodology definition to get custom fields
        let extra_fields = self.load_methodology_fields(methodology)?;

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
        repository.save(&use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        Ok(use_case_from_toml)
    }

    /// Load custom fields for a methodology with default values
    fn load_methodology_fields(&self, methodology: &str) -> Result<HashMap<String, Value>> {
        let templates_dir = self
            .config
            .directories
            .template_dir
            .clone()
            .unwrap_or_else(|| format!(".config/.mucm/{}", crate::config::Config::TEMPLATES_DIR));

        // MethodologyRegistry expects the parent directory containing "methodologies/"
        // But during template copying, methodologies are placed directly in template-assets/
        // Check if methodology directory exists directly in templates_dir
        let methodology_dir = Path::new(&templates_dir).join(methodology);
        let extra_fields =
            if methodology_dir.exists() && methodology_dir.join("config.toml").exists() {
                // Load methodology definition directly
                match MethodologyDefinition::from_toml(&methodology_dir) {
                    Ok(methodology_def) => self.extract_fields_from_definition(&methodology_def),
                    Err(_) => HashMap::new(),
                }
            } else {
                // Try standard structure: template-assets/methodologies/feature/
                let methodology_registry = MethodologyRegistry::new_dynamic(&templates_dir)?;
                if let Some(methodology_def) = methodology_registry.get(methodology) {
                    self.extract_fields_from_definition(methodology_def)
                } else {
                    HashMap::new()
                }
            };

        Ok(extra_fields)
    }

    /// Extract fields from a methodology definition with appropriate defaults
    fn extract_fields_from_definition(
        &self,
        methodology_def: &MethodologyDefinition,
    ) -> HashMap<String, Value> {
        let mut fields = HashMap::new();
        for (field_name, field_config) in methodology_def.custom_fields() {
            // Use default value if provided, otherwise use empty string for required fields
            let value = if let Some(default) = &field_config.default {
                Value::String(default.clone())
            } else if field_config.required {
                // For required fields without defaults, use empty string
                Value::String(String::new())
            } else {
                // For optional fields without defaults, use null
                Value::Null
            };
            fields.insert(field_name.clone(), value);
        }
        fields
    }
}
