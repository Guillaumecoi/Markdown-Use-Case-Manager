//! CLI commands for managing personas

use crate::cli::args::PersonaCommands;
use crate::config::Config;
use crate::core::{Persona, PersonaRepository, RepositoryFactory};
use anyhow::{Context, Result};

/// Handle persona commands
pub fn handle_persona_command(command: PersonaCommands, config: &Config) -> Result<()> {
    match command {
        PersonaCommands::Create {
            id,
            name,
            description,
            goal,
            context,
            tech_level,
            usage_frequency,
        } => create_persona(
            id,
            name,
            description,
            goal,
            context,
            tech_level,
            usage_frequency,
            config,
        ),
        PersonaCommands::List => list_personas(config),
        PersonaCommands::Show { id } => show_persona(&id, config),
        PersonaCommands::UseCases { id } => list_use_cases_for_persona(&id),
        PersonaCommands::Delete { id } => delete_persona(&id, config),
    }
}

/// Create a new persona
fn create_persona(
    id: String,
    name: String,
    description: String,
    goal: String,
    context: Option<String>,
    tech_level: Option<u8>,
    usage_frequency: Option<String>,
    config: &Config,
) -> Result<()> {
    // Validate tech_level if provided
    if let Some(level) = tech_level {
        if level < 1 || level > 5 {
            anyhow::bail!("Tech level must be between 1 and 5");
        }
    }

    // Create persona
    let mut persona = Persona::new(id.clone(), name, description, goal);

    if let Some(ctx) = context {
        persona = persona.with_context(ctx);
    }
    if let Some(level) = tech_level {
        persona = persona.with_tech_level(level);
    }
    if let Some(freq) = usage_frequency {
        persona = persona.with_usage_frequency(freq);
    }

    // Save to repository
    let repo = RepositoryFactory::create_persona_repository(config)
        .context("Failed to create persona repository")?;

    // Check if persona already exists
    if repo.exists(&id)? {
        anyhow::bail!("Persona with ID '{}' already exists", id);
    }

    repo.save(&persona).context("Failed to save persona")?;

    println!("✓ Created persona: {} ({})", persona.name, persona.id);
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
        println!("     Goal: {}", persona.goal);
        if let Some(tech_level) = persona.tech_level {
            println!("     Tech Level: {}/5", tech_level);
        }
        if let Some(freq) = &persona.usage_frequency {
            println!("     Usage: {}", freq);
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
    println!("\nDescription:");
    println!("  {}", persona.description);
    println!("\nGoal:");
    println!("  {}", persona.goal);

    if let Some(ctx) = &persona.context {
        println!("\nContext:");
        println!("  {}", ctx);
    }

    if let Some(level) = persona.tech_level {
        println!("\nTechnical Proficiency: {}/5", level);
    }

    if let Some(freq) = &persona.usage_frequency {
        println!("Usage Frequency: {}", freq);
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
        config.directories.toml_dir = Some(format!("{}/.mucm", temp_dir.path().display()));
        config
    }

    #[test]
    #[serial]
    fn test_create_persona_toml() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            None,
            Some(3),
            Some("daily".to_string()),
            &config,
        )?;

        // Verify persona exists
        let repo = RepositoryFactory::create_persona_repository(&config)?;
        assert!(repo.exists("test-persona")?);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_persona_sqlite() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Sqlite);

        create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            Some("Remote worker".to_string()),
            Some(4),
            Some("weekly".to_string()),
            &config,
        )?;

        // Verify persona exists
        let repo = RepositoryFactory::create_persona_repository(&config)?;
        let persona = repo.load_by_id("test-persona")?.unwrap();
        assert_eq!(persona.name, "Test User");
        assert_eq!(persona.tech_level, Some(4));
        assert_eq!(persona.context, Some("Remote worker".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_persona_duplicate() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        // Create first persona
        create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            None,
            None,
            None,
            &config,
        )?;

        // Try to create duplicate
        let result = create_persona(
            "test-persona".to_string(),
            "Another User".to_string(),
            "Another description".to_string(),
            "Another goal".to_string(),
            None,
            None,
            None,
            &config,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_persona_invalid_tech_level() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        // Try tech level 0
        let result = create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            None,
            Some(0),
            None,
            &config,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("between 1 and 5"));

        // Try tech level 6
        let result = create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            None,
            Some(6),
            None,
            &config,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("between 1 and 5"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_list_personas_empty() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        // Should not error with empty repository
        list_personas(&config)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_list_personas() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        // Create multiple personas
        create_persona(
            "persona-1".to_string(),
            "User One".to_string(),
            "First user".to_string(),
            "Goal one".to_string(),
            None,
            Some(3),
            None,
            &config,
        )?;

        create_persona(
            "persona-2".to_string(),
            "User Two".to_string(),
            "Second user".to_string(),
            "Goal two".to_string(),
            None,
            None,
            Some("daily".to_string()),
            &config,
        )?;

        // List should succeed
        list_personas(&config)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_show_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            Some("Works from home".to_string()),
            Some(4),
            Some("weekly".to_string()),
            &config,
        )?;

        show_persona("test-persona", &config)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_show_nonexistent_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        let result = show_persona("nonexistent", &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_delete_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test user".to_string(),
            "Test the system".to_string(),
            None,
            None,
            None,
            &config,
        )?;

        delete_persona("test-persona", &config)?;

        // Verify persona is deleted
        let repo = RepositoryFactory::create_persona_repository(&config)?;
        assert!(!repo.exists("test-persona")?);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_delete_nonexistent_persona() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        let result = delete_persona("nonexistent", &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_persona_with_all_optional_fields() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona(
            "full-persona".to_string(),
            "Full User".to_string(),
            "A complete persona".to_string(),
            "Use all features".to_string(),
            Some("Remote developer in Europe".to_string()),
            Some(5),
            Some("daily".to_string()),
            &config,
        )?;

        let repo = RepositoryFactory::create_persona_repository(&config)?;
        let persona = repo.load_by_id("full-persona")?.unwrap();

        assert_eq!(
            persona.context,
            Some("Remote developer in Europe".to_string())
        );
        assert_eq!(persona.tech_level, Some(5));
        assert_eq!(persona.usage_frequency, Some("daily".to_string()));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_persona_without_optional_fields() -> Result<()> {
        let temp_dir = TempDir::new()?;
        env::set_current_dir(&temp_dir)?;
        let config = create_test_config(&temp_dir, StorageBackend::Toml);

        create_persona(
            "minimal-persona".to_string(),
            "Minimal User".to_string(),
            "A minimal persona".to_string(),
            "Simple goal".to_string(),
            None,
            None,
            None,
            &config,
        )?;

        let repo = RepositoryFactory::create_persona_repository(&config)?;
        let persona = repo.load_by_id("minimal-persona")?.unwrap();

        assert_eq!(persona.context, None);
        assert_eq!(persona.tech_level, None);
        assert_eq!(persona.usage_frequency, None);

        Ok(())
    }
}
