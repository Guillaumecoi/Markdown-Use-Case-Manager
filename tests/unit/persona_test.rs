//! Tests for persona management functionality
//! This module tests the new persona CLI commands and configuration system

use anyhow::Result;
use std::fs;
use std::env;
use tempfile::TempDir;
use markdown_use_case_manager::config::Config;

#[test]
fn test_default_persona_directory_config() -> Result<()> {
    let config = Config::default();
    assert_eq!(config.directories.persona_dir, "docs/personas");
    Ok(())
}

#[test]
fn test_custom_persona_directory_config() -> Result<()> {
    // Test that Config can serialize and include custom persona directory
    let mut config = Config::default();
    config.directories.persona_dir = "custom/personas".to_string();
    
    // Serialize to TOML string and verify content
    let toml_content = toml::to_string(&config)?;
    assert!(toml_content.contains("custom/personas"));
    
    // Test deserialization
    let loaded_config: Config = toml::from_str(&toml_content)?;
    assert_eq!(loaded_config.directories.persona_dir, "custom/personas");
    
    Ok(())
}

#[test]
fn test_metadata_config_boolean_flags() -> Result<()> {
    let config = Config::default();
    
    // Test that all boolean flags exist and are accessible
    assert!(config.metadata.include_prerequisites);
    assert!(config.metadata.include_personas);
    assert!(config.metadata.include_author);
    assert!(config.metadata.include_reviewer);
    assert!(config.metadata.include_business_value);
    assert!(config.metadata.include_complexity);
    assert!(config.metadata.include_epic);
    assert!(config.metadata.include_acceptance_criteria);
    assert!(config.metadata.include_assumptions);
    assert!(config.metadata.include_constraints);
    
    Ok(())
}

#[test]
fn test_metadata_config_individual_flags() -> Result<()> {
    // Test that individual metadata flags can be serialized and deserialized
    let mut config = Config::default();
    
    // Test setting individual flags
    config.metadata.include_prerequisites = false;
    config.metadata.include_personas = true;
    config.metadata.include_author = false;
    config.metadata.include_reviewer = true;
    
    // Serialize and deserialize to test persistence
    let toml_content = toml::to_string(&config)?;
    let loaded_config: Config = toml::from_str(&toml_content)?;
    
    assert!(!loaded_config.metadata.include_prerequisites);
    assert!(loaded_config.metadata.include_personas);
    assert!(!loaded_config.metadata.include_author);
    assert!(loaded_config.metadata.include_reviewer);
    
    Ok(())
}

#[test]
fn test_config_no_custom_fields_array() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    
    // Change to temp directory for the test
    env::set_current_dir(&temp_dir)?;
    
    // Create the config directory structure
    std::fs::create_dir_all(".config/.mucm")?;
    
    let config = Config::default();
    config.save_in_dir(".")?;
    
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let config_content = fs::read_to_string(&config_path)?;
    
    // Ensure the old custom_fields array is not present
    assert!(!config_content.contains("custom_fields"));
    
    // Ensure the new boolean flags are present
    assert!(config_content.contains("include_prerequisites"));
    assert!(config_content.contains("include_personas"));
    assert!(config_content.contains("include_author"));
    assert!(config_content.contains("include_reviewer"));
    
    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_extended_metadata_serialization() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    
    // Change to temp directory for the test
    env::set_current_dir(&temp_dir)?;
    
    // Create the config directory structure
    std::fs::create_dir_all(".config/.mucm")?;
    
    let mut config = Config::default();
    
    // Set specific extended metadata flags
    config.metadata.include_business_value = false;
    config.metadata.include_complexity = true;
    config.metadata.include_epic = false;
    config.metadata.include_acceptance_criteria = true;
    config.metadata.include_assumptions = false;
    config.metadata.include_constraints = true;
    
    // Save and reload
    config.save_in_dir(".")?;
    let loaded_config = Config::load()?;
    
    // Verify extended metadata flags are preserved
    assert!(!loaded_config.metadata.include_business_value);
    assert!(loaded_config.metadata.include_complexity);
    assert!(!loaded_config.metadata.include_epic);
    assert!(loaded_config.metadata.include_acceptance_criteria);
    assert!(!loaded_config.metadata.include_assumptions);
    assert!(loaded_config.metadata.include_constraints);
    
    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_persona_dir_in_config_toml() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    
    // Change to temp directory for the test
    env::set_current_dir(&temp_dir)?;
    
    // Create the config directory structure
    std::fs::create_dir_all(".config/.mucm")?;
    
    let mut config = Config::default();
    config.directories.persona_dir = "docs/stakeholders".to_string();
    config.save_in_dir(".")?;
    
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let config_content = fs::read_to_string(&config_path)?;
    
    // Ensure persona_dir is saved in the TOML file
    assert!(config_content.contains("persona_dir = \"docs/stakeholders\""));
    
    // Restore original directory
    env::set_current_dir(original_dir)?;
    Ok(())
}