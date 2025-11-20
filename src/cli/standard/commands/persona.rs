//! CLI commands for managing personas

use crate::cli::args::PersonaCommands;
use crate::config::Config;
use crate::core::{Persona, RepositoryFactory};
use anyhow::{Context, Result};

/// Handle persona commands
pub fn handle_persona_command(command: PersonaCommands, config: &Config) -> Result<()> {
    match command {
        PersonaCommands::Create { id, name } => create_persona(id, name, config),
        PersonaCommands::List => list_personas(config),
        PersonaCommands::Show { id } => show_persona(&id, config),
        PersonaCommands::UseCases { id } => list_use_cases_for_persona(&id),
        PersonaCommands::Delete { id } => delete_persona(&id, config),
    }
}

/// Create a new persona with fields from config
///
/// Creates a minimal persona (id + name) and initializes additional fields
/// based on the persona configuration. The user can fill in the values later
/// by editing the generated TOML file or SQL record directly.
fn create_persona(id: String, name: String, config: &Config) -> Result<()> {
    // Create persona with config-driven fields
    let persona = if config.persona.fields.is_empty() {
        // No custom fields defined, just create minimal persona
        Persona::new(id.clone(), name)
    } else {
        // Initialize persona with empty/default values for all config fields
        Persona::from_config_fields(id.clone(), name, &config.persona.fields)
    };

    // Save to repository
    let repo = RepositoryFactory::create_persona_repository(config)
        .context("Failed to create persona repository")?;

    // Check if persona already exists
    if repo.exists(&id)? {
        anyhow::bail!("Persona with ID '{}' already exists", id);
    }

    repo.save(&persona).context("Failed to save persona")?;

    if !config.persona.fields.is_empty() {
        println!("✓ Created persona: {} ({})", persona.name, persona.id);
        println!("  Edit the generated file to fill in these fields:");
        for field_name in config.persona.field_names() {
            println!("    - {}", field_name);
        }
    } else {
        println!("✓ Created persona: {} ({})", persona.name, persona.id);
        println!("  Tip: Add custom fields in .config/.mucm/mucm.toml under [persona.fields]");
    }

    Ok(())
}

/// List all personas
fn list_personas(config: &Config) -> Result<()> {
    let repo = RepositoryFactory::create_persona_repository(config)
        .context("Failed to create persona repository")?;

    let personas = repo.load_all().context("Failed to load personas")?;

    if personas.is_empty() {
        println!("No personas found.");
        return Ok(());
    }

    println!("Personas ({}):\n", personas.len());
    for persona in personas {
        println!("  {} {} - {}", persona.emoji(), persona.name, persona.id);

        // Show a few key extra fields if they exist
        if let Some(role) = persona.extra.get("role") {
            if let Some(role_str) = role.as_str() {
                println!("     Role: {}", role_str);
            }
        }
        if let Some(dept) = persona.extra.get("department") {
            if let Some(dept_str) = dept.as_str() {
                println!("     Department: {}", dept_str);
            }
        }
        println!();
    }

    Ok(())
}

/// Show persona details
fn show_persona(id: &str, config: &Config) -> Result<()> {
    let repo = RepositoryFactory::create_persona_repository(config)
        .context("Failed to create persona repository")?;

    let persona = repo
        .load_by_id(id)
        .context("Failed to load persona")?
        .ok_or_else(|| anyhow::anyhow!("Persona '{}' not found", id))?;

    println!("{} {}", persona.emoji(), persona.name);
    println!("ID: {}", persona.id);
    println!("Created: {}", persona.metadata.created_at);
    println!("Last Updated: {}", persona.metadata.updated_at);

    if !persona.extra.is_empty() {
        println!("\nFields:");
        for (key, value) in &persona.extra {
            match value {
                serde_json::Value::String(s) if !s.is_empty() => {
                    println!("  {}: {}", key, s);
                }
                serde_json::Value::Array(arr) if !arr.is_empty() => {
                    println!("  {}:", key);
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            println!("    - {}", s);
                        }
                    }
                }
                serde_json::Value::Number(n) => {
                    println!("  {}: {}", key, n);
                }
                serde_json::Value::Bool(b) => {
                    println!("  {}: {}", key, b);
                }
                _ => {} // Skip empty or null values
            }
        }
    } else {
        println!("\n(No additional fields defined)");
    }

    Ok(())
}

/// Delete a persona
fn delete_persona(id: &str, config: &Config) -> Result<()> {
    let repo = RepositoryFactory::create_persona_repository(config)
        .context("Failed to create persona repository")?;

    // Check if persona exists
    if !repo.exists(id)? {
        anyhow::bail!("Persona '{}' not found", id);
    }

    repo.delete(id).context("Failed to delete persona")?;

    println!("✓ Deleted persona: {}", id);
    Ok(())
}

/// List use cases that use a specific persona
fn list_use_cases_for_persona(persona_id: &str) -> Result<()> {
    use crate::cli::standard::CliRunner;
    use crate::presentation::DisplayResultFormatter;

    let mut runner = CliRunner::new();

    let result = runner.list_use_cases_for_persona(persona_id.to_string())?;
    DisplayResultFormatter::display(&result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StorageBackend;
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    fn create_test_config(temp_dir: &TempDir, backend: StorageBackend) -> Config {
        let mut config = Config::default();
        config.storage.backend = backend;
        config.directories.use_case_dir = format!("{}/docs/use-cases", temp_dir.path().display());
        config.directories.data_dir = format!("{}/.mucm", temp_dir.path().display());
        config
    }

    #[test]
    #[serial]
    fn test_create_persona_basic() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona("test".to_string(), "Test User".to_string(), &config)?;

        let repo = RepositoryFactory::create_persona_repository(&config)?;
        assert!(repo.exists("test")?);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_persona_duplicate() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona("test".to_string(), "Test User".to_string(), &config)?;
        let result = create_persona("test".to_string(), "Another User".to_string(), &config);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_delete_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona("test".to_string(), "Test User".to_string(), &config)?;
        delete_persona("test", &config)?;

        let repo = RepositoryFactory::create_persona_repository(&config)?;
        assert!(!repo.exists("test")?);

        Ok(())
    }
}
