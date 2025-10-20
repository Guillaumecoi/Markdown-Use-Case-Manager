//! Integration tests for persona CLI commands
//! This module tests the complete persona management workflow

use anyhow::Result;
use assert_cmd::Command;
use predicates;
use std::fs;
use tempfile::TempDir;
use serial_test::serial;

/// Test persona creation via CLI
#[test]
#[serial]
fn test_persona_create_command() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project first
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Create a persona
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("create")
        .arg("developer")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Created persona 'developer'"));

    // Verify the persona file was created
    let persona_path = temp_dir.path().join("docs/personas/developer.md");
    assert!(persona_path.exists());

    let content = fs::read_to_string(&persona_path)?;
    assert!(content.contains("# Persona: developer"));
    assert!(content.contains("## Characteristics"));
    assert!(content.contains("## Goals"));
    assert!(content.contains("## Pain Points"));

    Ok(())
}

/// Test persona listing via CLI
#[test]
#[serial]
fn test_persona_list_command() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Create multiple personas
    for persona in &["user", "admin", "manager"] {
        let mut cmd = Command::cargo_bin("mucm")?;
        cmd.arg("persona")
            .arg("create")
            .arg(persona)
            .current_dir(&temp_dir)
            .assert()
            .success();
    }

    // List personas
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("list")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("👥 Available Personas"))
        .stdout(predicates::str::contains("user"))
        .stdout(predicates::str::contains("admin"))
        .stdout(predicates::str::contains("manager"));

    Ok(())
}

/// Test persona deletion via CLI
#[test]
#[serial]
fn test_persona_delete_command() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project and create persona
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("create")
        .arg("test-user")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Verify persona exists
    let persona_path = temp_dir.path().join("docs/personas/test-user.md");
    assert!(persona_path.exists());

    // Delete persona
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("delete")
        .arg("test-user")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Deleted persona 'test-user'"));

    // Verify persona is gone
    assert!(!persona_path.exists());

    Ok(())
}

/// Test persona creation with custom directory
#[test]
#[serial]
fn test_persona_custom_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Modify config to use custom persona directory
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path)?;
    config_content = config_content.replace("docs/personas", "custom/stakeholders");
    fs::write(&config_path, config_content)?;

    // Create persona
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("create")
        .arg("stakeholder")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Verify persona was created in custom directory
    let persona_path = temp_dir.path().join("custom/stakeholders/stakeholder.md");
    assert!(persona_path.exists());

    // Verify default directory is not used
    let default_path = temp_dir.path().join("docs/personas/stakeholder.md");
    assert!(!default_path.exists());

    Ok(())
}

/// Test persona list with empty directory
#[test]
#[serial]
fn test_persona_list_empty() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // List personas in empty directory
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("list")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("No personas found"));

    Ok(())
}

/// Test persona delete non-existent
#[test]
#[serial]
fn test_persona_delete_nonexistent() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Try to delete non-existent persona
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("delete")
        .arg("nonexistent")
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicates::str::contains("Persona 'nonexistent' not found"));

    Ok(())
}

/// Test that persona commands work without project initialization should fail
#[test]
#[serial]
fn test_persona_commands_require_project() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Try persona command without init
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("persona")
        .arg("list")
        .current_dir(&temp_dir)
        .assert()
        .failure()
        .stderr(predicates::str::contains("No markdown use case manager project found"));

    Ok(())
}