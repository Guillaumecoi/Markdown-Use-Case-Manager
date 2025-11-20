//! # Persona Controller
//!
//! This module provides the controller for persona management operations.
//! It handles the coordination between CLI commands and persona application
//! services, providing a clean interface for creating, updating, listing,
//! and deleting personas.
//!
//! ## Responsibilities
//!
//! - Persona creation with custom fields
//! - Persona updating (name and custom fields)
//! - Persona listing and retrieval
//! - Persona deletion
//! - Data retrieval for interactive selection prompts

use crate::config::Config;
use crate::controller::dto::DisplayResult;
use crate::core::{Persona, PersonaRepository, SqlitePersonaRepository, TomlPersonaRepository};
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Controller for persona operations and management.
///
/// Manages all persona-related operations including creation, updating,
/// listing, and deletion. Acts as the coordination layer between CLI
/// commands and persona persistence.
pub struct PersonaController {
    /// Repository for persona persistence
    repository: Box<dyn PersonaRepository>,
    /// Project configuration
    config: Config,
}

impl PersonaController {
    /// Create a new persona controller instance.
    ///
    /// Initializes the controller with the appropriate repository backend
    /// based on the project configuration (TOML or SQLite).
    ///
    /// # Returns
    /// A new PersonaController instance ready for use
    ///
    /// # Errors
    /// Returns error if the configuration cannot be loaded or repository creation fails
    pub fn new() -> Result<Self> {
        let config = Config::load()?;

        let repository: Box<dyn PersonaRepository> = match config.storage.backend {
            crate::config::StorageBackend::Sqlite => {
                use rusqlite::Connection;
                use std::sync::{Arc, Mutex};

                let db_path = format!("{}/mucm.db", config.directories.data_dir);
                let conn = Connection::open(&db_path)?;
                SqlitePersonaRepository::initialize(&conn)?;
                let repo = SqlitePersonaRepository::new(Arc::new(Mutex::new(conn)));
                Box::new(repo)
            }
            crate::config::StorageBackend::Toml => {
                let repo = TomlPersonaRepository::new(config.clone());
                Box::new(repo)
            }
        };

        Ok(Self { repository, config })
    }

    /// Create a new persona with basic information.
    ///
    /// Creates a persona with an ID and name, initializing custom fields
    /// based on the project's persona configuration.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the persona (e.g., "customer", "admin")
    /// * `name` - Display name for the persona
    ///
    /// # Returns
    /// DisplayResult with success message and persona information
    ///
    /// # Errors
    /// Returns error if persona creation fails or ID already exists
    pub fn create_persona(&self, id: String, name: String) -> Result<DisplayResult> {
        // Check if persona already exists
        if self.repository.exists(&id)? {
            return Ok(DisplayResult::error(format!(
                "Persona with ID '{}' already exists",
                id
            )));
        }

        // Create persona with config fields
        let persona = Persona::from_config_fields(id.clone(), name, &self.config.persona.fields);

        // Save the persona
        self.repository.save(&persona)?;

        Ok(DisplayResult::success(format!(
            "Created persona: {} ({})",
            persona.name, persona.id
        )))
    }

    /// Update a persona's basic information.
    ///
    /// Updates the persona's name. Custom fields are updated separately
    /// via update_persona_fields().
    ///
    /// # Arguments
    /// * `id` - The persona ID to update
    /// * `name` - Optional new name
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if persona not found or update fails
    pub fn update_persona(&self, id: String, name: Option<String>) -> Result<DisplayResult> {
        // Load existing persona
        let mut persona = self
            .repository
            .load_by_id(&id)?
            .context(format!("Persona '{}' not found", id))?;

        // Update fields
        if let Some(new_name) = name {
            persona.name = new_name;
        }

        // Save updated persona
        self.repository.save(&persona)?;

        Ok(DisplayResult::success(format!(
            "Updated persona: {}",
            persona.id
        )))
    }

    /// Update persona custom fields.
    ///
    /// Updates or adds custom fields to a persona. Fields are merged with
    /// existing values - only specified fields are updated.
    ///
    /// # Arguments
    /// * `id` - The persona ID to update
    /// * `fields` - Map of field names to new values (as strings)
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if persona not found or update fails
    pub fn update_persona_fields(
        &self,
        id: String,
        fields: HashMap<String, String>,
    ) -> Result<DisplayResult> {
        // Load existing persona
        let mut persona = self
            .repository
            .load_by_id(&id)?
            .context(format!("Persona '{}' not found", id))?;

        // Update custom fields
        for (field_name, field_value) in fields {
            // Parse field value as JSON to support different types
            let json_value: serde_json::Value = if field_value.is_empty() {
                serde_json::Value::String(String::new())
            } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(&field_value) {
                val  // Successfully parsed as JSON (arrays, objects, etc.)
            } else if let Ok(num) = field_value.parse::<f64>() {
                // Number
                serde_json::json!(num)
            } else if field_value == "true" || field_value == "false" {
                // Boolean
                serde_json::json!(field_value.parse::<bool>().unwrap())
            } else {
                // String
                serde_json::json!(field_value)
            };

            persona.extra.insert(field_name, json_value);
        }

        // Save updated persona
        self.repository.save(&persona)?;

        Ok(DisplayResult::success(format!(
            "Updated custom fields for persona: {}",
            persona.id
        )))
    }

    /// Delete a persona.
    ///
    /// Removes a persona from the project, including its data file and
    /// any generated markdown documentation.
    ///
    /// # Arguments
    /// * `id` - The persona ID to delete
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if persona not found or deletion fails
    pub fn delete_persona(&self, id: String) -> Result<DisplayResult> {
        // Check if persona exists
        if !self.repository.exists(&id)? {
            return Ok(DisplayResult::error(format!("Persona '{}' not found", id)));
        }

        // Delete the persona
        self.repository.delete(&id)?;

        Ok(DisplayResult::success(format!("Deleted persona: {}", id)))
    }

    /// Get a single persona by ID.
    ///
    /// Retrieves a persona for viewing or editing.
    ///
    /// # Arguments
    /// * `id` - The persona ID to retrieve
    ///
    /// # Returns
    /// Reference to the persona
    ///
    /// # Errors
    /// Returns error if persona not found
    pub fn get_persona(&self, id: &str) -> Result<Persona> {
        self.repository
            .load_by_id(id)?
            .context(format!("Persona '{}' not found", id))
    }

    /// List all personas in the project.
    ///
    /// Retrieves all personas for display or selection.
    ///
    /// # Returns
    /// Vector of all personas
    ///
    /// # Errors
    /// Returns error if persona retrieval fails
    pub fn list_personas(&self) -> Result<Vec<Persona>> {
        self.repository.load_all()
    }

    /// Get list of persona IDs for selection prompts.
    ///
    /// Returns a simple list of IDs suitable for dropdown menus
    /// and selection prompts.
    ///
    /// # Returns
    /// Vector of persona IDs
    ///
    /// # Errors
    /// Returns error if persona retrieval fails
    pub fn get_persona_ids(&self) -> Result<Vec<String>> {
        let personas = self.repository.load_all()?;
        Ok(personas.into_iter().map(|p| p.id).collect())
    }

    /// Get persona custom field definitions from config.
    ///
    /// Returns the custom field definitions configured for personas
    /// in the project, useful for dynamic form generation.
    ///
    /// # Returns
    /// Map of field name to field configuration
    pub fn get_persona_field_config(&self) -> HashMap<String, crate::core::CustomFieldConfig> {
        self.config.persona.fields.clone()
    }

    /// Get current custom field values for a persona.
    ///
    /// Retrieves the current values of custom fields for editing.
    ///
    /// # Arguments
    /// * `id` - The persona ID
    ///
    /// # Returns
    /// Map of field names to JSON values
    ///
    /// # Errors
    /// Returns error if persona not found
    pub fn get_persona_field_values(&self, id: &str) -> Result<HashMap<String, serde_json::Value>> {
        let persona = self.get_persona(id)?;
        Ok(persona.extra.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config(temp_dir: &TempDir) -> Result<()> {
        let config_content = r#"
[project]
name = "Test Project"
description = "Test project"

[directories]
use_case_dir = "docs/use-cases"
test_dir = "tests"
persona_dir = "docs/personas"
data_dir = "data"

[templates]
methodologies = ["feature"]
default_methodology = "feature"

[generation]
test_language = "rust"
auto_generate_tests = false
overwrite_test_documentation = false

[storage]
backend = "toml"

[metadata]
created = true
last_updated = true

[persona.fields]
department = { type = "string", required = false }
experience_level = { type = "string", required = false }
"#;

        // Create .config/.mucm directory structure (where config is expected)
        fs::create_dir_all(temp_dir.path().join(".config/.mucm"))?;
        fs::create_dir_all(temp_dir.path().join("data"))?;
        fs::write(
            temp_dir.path().join(".config/.mucm/mucm.toml"),
            config_content,
        )?;
        std::env::set_current_dir(temp_dir.path())?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(&temp_dir)?;

        let controller = PersonaController::new()?;
        let result = controller.create_persona("test_user".to_string(), "Test User".to_string())?;

        assert!(result.success);
        assert!(result.message.contains("Created persona"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_duplicate_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(&temp_dir)?;

        let controller = PersonaController::new()?;
        controller.create_persona("test_user".to_string(), "Test User".to_string())?;

        let result =
            controller.create_persona("test_user".to_string(), "Another User".to_string())?;

        assert!(!result.success);
        assert!(result.message.contains("already exists"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_update_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(&temp_dir)?;

        let controller = PersonaController::new()?;
        controller.create_persona("test_user".to_string(), "Test User".to_string())?;

        let result =
            controller.update_persona("test_user".to_string(), Some("Updated Name".to_string()))?;

        assert!(result.success);

        let persona = controller.get_persona("test_user")?;
        assert_eq!(persona.name, "Updated Name");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_update_persona_fields_single_item_array() -> Result<()> {
        let temp_dir = TempDir::new()?;
        
        // Create config with an array field
        let config_content = r#"
[project]
name = "Test Project"
description = "Test project"

[directories]
use_case_dir = "docs/use-cases"
test_dir = "tests"
persona_dir = "docs/personas"
data_dir = "data"

[templates]
methodologies = ["feature"]
default_methodology = "feature"

[generation]
test_language = "rust"
auto_generate_tests = false
overwrite_test_documentation = false

[storage]
backend = "toml"

[metadata]
created = true
last_updated = true

[persona.fields]
pain_points = { type = "array", required = false }
"#;

        fs::create_dir_all(temp_dir.path().join(".config/.mucm"))?;
        fs::create_dir_all(temp_dir.path().join("data"))?;
        fs::write(
            temp_dir.path().join(".config/.mucm/mucm.toml"),
            config_content,
        )?;
        std::env::set_current_dir(temp_dir.path())?;

        let controller = PersonaController::new()?;
        controller.create_persona("test_user".to_string(), "Test User".to_string())?;

        // Update with single-item array (the bug case) - now sent as JSON
        let mut fields = HashMap::new();
        fields.insert("pain_points".to_string(), r#"["Single pain point"]"#.to_string());

        let result = controller.update_persona_fields("test_user".to_string(), fields)?;
        assert!(result.success);

        // Verify it's stored as an array, not a string
        let persona = controller.get_persona("test_user")?;
        let pain_points = persona.extra.get("pain_points").unwrap();
        
        assert!(pain_points.is_array(), "Single item should be stored as array");
        let arr = pain_points.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0].as_str().unwrap(), "Single pain point");

        Ok(())
    }
}
