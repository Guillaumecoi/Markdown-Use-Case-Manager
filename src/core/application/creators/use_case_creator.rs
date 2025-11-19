use crate::config::Config;
use crate::core::application::MethodologyFieldCollector;
use crate::core::domain::UseCaseService;
use crate::core::{
    Methodology, MethodologyDefinition, MethodologyRegistry, MethodologyView, UseCase,
    UseCaseRepository,
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

    /// Create a use case with methodology-specific custom fields
    pub fn create_use_case_with_methodology(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        priority: String,
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
                priority,
                extra_fields,
            )
            .map_err(|e: String| anyhow::anyhow!(e))?;

        // Step 1: Save TOML first (source of truth) - this will include custom fields
        repository.save(&use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        Ok(use_case_from_toml)
    }

    pub fn create_use_case_with_custom_fields(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        priority: String,
        methodology: &str,
        user_fields: std::collections::HashMap<String, String>,
        existing_use_cases: &[UseCase],
        repository: &dyn UseCaseRepository,
    ) -> Result<UseCase> {
        let use_case_id = self.use_case_service.generate_unique_use_case_id(
            &category,
            existing_use_cases,
            &self.config.directories.use_case_dir,
        );
        let description = description.unwrap_or_default();

        // Load methodology default fields
        let mut extra_fields = self.load_methodology_fields(methodology)?;

        // Override with user-provided fields
        for (key, value) in user_fields {
            extra_fields.insert(key, serde_json::Value::String(value));
        }

        let use_case = self
            .use_case_service
            .create_use_case_with_extra(
                use_case_id.clone(),
                title,
                category,
                description,
                priority,
                extra_fields,
            )
            .map_err(|e: String| anyhow::anyhow!(e))?;

        // Step 1: Save TOML first (source of truth) - this will include custom fields
        repository.save(&use_case)?;

        // Step 2: Load from TOML to ensure we're working with persisted data
        let use_case_from_toml = repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        Ok(use_case_from_toml)
    }

    /// Create a use case with multiple methodology views and collected fields
    ///
    /// This method properly uses MethodologyFieldCollector to gather fields from all views,
    /// stores them in methodology_fields structure, and handles user value overrides.
    pub fn create_use_case_with_views(
        &self,
        title: String,
        category: String,
        description: Option<String>,
        priority: String,
        views: Vec<MethodologyView>,
        user_fields: HashMap<String, String>,
        existing_use_cases: &[UseCase],
        repository: &dyn UseCaseRepository,
    ) -> Result<UseCase> {
        let use_case_id = self.use_case_service.generate_unique_use_case_id(
            &category,
            existing_use_cases,
            &self.config.directories.use_case_dir,
        );
        let description = description.unwrap_or_default();

        // Collect fields from all methodology views using the collector
        // If collector fails (e.g., in test environment without methodologies), use empty fields
        let collector = MethodologyFieldCollector::new()?;
        let view_pairs: Vec<(String, String)> = views
            .iter()
            .map(|v| (v.methodology.clone(), v.level.clone()))
            .collect();

        let field_collection = match collector.collect_fields_for_views(&view_pairs) {
            Ok(collection) => collection,
            Err(e) => {
                eprintln!(
                    "Warning: Could not collect methodology fields: {}. Using empty fields.",
                    e
                );
                Default::default()
            }
        };

        // Display any warnings (e.g., standard field conflicts)
        for warning in &field_collection.warnings {
            eprintln!("{}", warning);
        }

        // Apply user-provided values to the collected fields
        let methodology_field_values = collector.apply_user_values(&field_collection, user_fields);

        // Group fields by methodology for storage in methodology_fields
        let mut methodology_fields: HashMap<String, HashMap<String, Value>> = HashMap::new();

        // Initialize empty HashMap for each methodology in views
        for view in &views {
            methodology_fields
                .entry(view.methodology.clone())
                .or_insert_with(HashMap::new);
        }

        // Populate with actual field values
        for (field_name, field_value) in methodology_field_values {
            // Find which methodology this field belongs to
            if let Some(collected_field) = field_collection.fields.get(&field_name) {
                for methodology in &collected_field.methodologies {
                    methodology_fields
                        .entry(methodology.clone())
                        .or_insert_with(HashMap::new)
                        .insert(field_name.clone(), field_value.clone());
                }
            }
        }

        // Create the use case with empty extra fields (methodology fields go in methodology_fields)
        let mut use_case =
            UseCase::new(use_case_id.clone(), title, category, description, priority)
                .map_err(|e| anyhow::anyhow!(e))?;

        // Set methodology fields
        use_case.methodology_fields = methodology_fields;

        // Add all views
        for view in views {
            use_case.add_view(view);
        }

        // Save and reload from TOML
        repository.save(&use_case)?;
        let use_case_from_toml = repository
            .load_by_id(&use_case.id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to load newly created use case from TOML"))?;

        Ok(use_case_from_toml)
    }

    /// Load custom fields for a methodology with default values
    fn load_methodology_fields(&self, methodology: &str) -> Result<HashMap<String, Value>> {
        let templates_dir = format!(".config/.mucm/{}", crate::config::Config::TEMPLATES_DIR);

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
