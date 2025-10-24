//! Tests for persona management functionality
//! This module tests the new persona CLI commands and configuration system

use anyhow::Result;
use serial_test::serial;
use std::fs;
use std::env;
use tempfile::TempDir;
use markdown_use_case_manager::config::Config;

#[test]
#[serial]
fn test_default_persona_directory_config() -> Result<()> {
    let config = Config::default();
    assert_eq!(config.directories.persona_dir, "docs/personas");
    Ok(())
}

#[test]
#[serial]
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
#[serial]
fn test_metadata_config_timestamps() -> Result<()> {
    let config = Config::default();
    
    // Test that timestamp flags exist and are accessible
    assert!(config.metadata.created);
    assert!(config.metadata.last_updated);
    
    Ok(())
}

#[test]
#[serial]
fn test_metadata_config_serialization() -> Result<()> {
    // Test that metadata flags can be serialized and deserialized
    let mut config = Config::default();
    
    // Test setting flags
    config.metadata.created = false;
    config.metadata.last_updated = true;
    
    // Serialize and deserialize to test persistence
    let toml_content = toml::to_string(&config)?;
    let loaded_config: Config = toml::from_str(&toml_content)?;
    
    assert!(!loaded_config.metadata.created);
    assert!(loaded_config.metadata.last_updated);
    
    Ok(())
}

#[test]
#[serial]
fn test_persona_dir_in_config_toml() -> Result<()> {
    // Get original dir FIRST, before any test might have corrupted it
    let original_dir = env::current_dir().unwrap_or_else(|_| std::env::temp_dir());
    
    let temp_dir = TempDir::new()?;
    
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
    
    // Restore original directory BEFORE TempDir drops
    env::set_current_dir(&original_dir)?;
    Ok(())
}