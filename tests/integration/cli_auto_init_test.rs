// Integration tests for auto-init and settings configuration features
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

/// Test that CLI detects uninitialized project and shows appropriate error
#[test]
#[serial]
fn test_cli_auto_init_error_detection() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Test that CLI commands fail gracefully in uninitialized directory
    let mut cmd = Command::cargo_bin("mucm")?;
    let output = cmd.arg("list").current_dir(&temp_dir).output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr)?;
    assert!(stderr.contains("No markdown use case manager project found"));
    assert!(stderr.contains("Run 'mucm init' first"));

    Ok(())
}

/// Test that CLI init works and creates proper structure
#[test]
#[serial]
fn test_cli_auto_init_creates_structure() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Test init command
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Project initialized"));

    // Verify structure was created
    let config_file = temp_dir.path().join(".config/.mucm/mucm.toml");
    assert!(config_file.exists(), "Config file should exist");

    let use_case_dir = temp_dir.path().join("docs/use-cases");
    assert!(use_case_dir.exists(), "Use case directory should exist");

    let test_dir = temp_dir.path().join("tests/use-cases");
    assert!(test_dir.exists(), "Test directory should exist");

    // Test that CLI commands now work
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("list")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("No use cases found"));

    Ok(())
}

/// Test CLI init with language parameter
#[test]
#[serial]
fn test_cli_auto_init_with_language() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Test init with Python language
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .arg("--language")
        .arg("python")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Project initialized"));

    // Verify Python template was created
    let python_template = temp_dir
        .path()
        .join(".config/.mucm/templates/lang-python/test.hbs");
    assert!(python_template.exists(), "Python template should exist");

    // Verify config has correct language
    let config_content = fs::read_to_string(temp_dir.path().join(".config/.mucm/mucm.toml"))?;
    assert!(config_content.contains("test_language = \"python\""));

    Ok(())
}

/// Test that auto-init doesn't break existing functionality
#[test]
#[serial]
fn test_cli_auto_init_preserves_functionality() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init").current_dir(&temp_dir).assert().success();

    // Test complete CLI workflow

    // 1. Create use case
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("create")
        .arg("Auto Init Test")
        .arg("--category")
        .arg("testing")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created use case: UC-TES-001"));

    // 2. Add scenario
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("add-scenario")
        .arg("UC-TES-001")
        .arg("Test Scenario")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Added scenario: UC-TES-001-S01"));

    // 3. Update status
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("update-status")
        .arg("UC-TES-001-S01")
        .arg("--status")
        .arg("implemented")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated scenario UC-TES-001-S01"));

    // 4. List use cases
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("list")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("UC-TES-001"))
        .stdout(predicate::str::contains("Auto Init Test"));

    // 5. Show status
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("status")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Total Use Cases: 1"))
        .stdout(predicate::str::contains("Total Scenarios: 1"));

    Ok(())
}

/// Test interactive mode entry points work after auto-init
#[test]
#[serial]
fn test_cli_interactive_mode_after_auto_init() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init").current_dir(&temp_dir).assert().success();

    // Test various ways to enter interactive mode with timeout
    // Since we can't easily test interactive input, we'll test that the commands
    // start properly and can be interrupted

    // Test: mucm interactive --help (should show help without entering interactive mode)
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("interactive")
        .arg("--help")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Enter interactive mode"));

    // Test: mucm -h should show the -i flag
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("--help")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("-i"))
        .stdout(predicate::str::contains("interactive"));

    Ok(())
}

/// Test that config file is properly formatted after auto-init
#[test]
#[serial]
fn test_cli_auto_init_config_format() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .arg("--language")
        .arg("rust")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Read and verify config file format
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let config_content = fs::read_to_string(&config_path)?;

    // Verify TOML structure
    assert!(config_content.contains("[project]"));
    assert!(config_content.contains("[directories]"));
    assert!(config_content.contains("[templates]"));
    assert!(config_content.contains("[generation]"));
    assert!(config_content.contains("[metadata]"));

    // Verify default values
    assert!(config_content.contains("name = \"My Project\""));
    assert!(config_content.contains("use_case_dir = \"docs/use-cases\""));
    assert!(config_content.contains("test_dir = \"tests/use-cases\""));
    assert!(config_content.contains("test_language = \"rust\""));
    assert!(config_content.contains("enabled = true"));

    // Verify custom fields array
    assert!(config_content.contains("custom_fields = ["));
    assert!(config_content.contains("\"author\""));
    assert!(config_content.contains("\"prerequisites\""));

    Ok(())
}

/// Test error handling for invalid initialization scenarios
#[test]
#[serial]
fn test_cli_auto_init_error_handling() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Test init with invalid language
    let mut cmd = Command::cargo_bin("mucm")?;
    let output = cmd
        .arg("init")
        .arg("--language")
        .arg("invalidlang")
        .current_dir(&temp_dir)
        .output()?;

    // Should either succeed (creating placeholder) or fail gracefully
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        assert!(stderr.contains("language") || stderr.contains("Unsupported"));
    }

    // Test double initialization
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init").current_dir(&temp_dir).assert().success();

    // Second init should handle existing project gracefully
    let mut cmd = Command::cargo_bin("mucm")?;
    let output = cmd.arg("init").current_dir(&temp_dir).output()?;

    // Check that it handles existing project (either succeeds or warns)
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    // Should not crash - either success or appropriate message
    assert!(
        output.status.success()
            || stderr.contains("already")
            || stderr.contains("exist")
            || stdout.contains("Project initialized")
    );

    Ok(())
}

/// Test that templates are properly copied during auto-init
#[test]
#[serial]
fn test_cli_auto_init_template_creation() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize with Rust
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init")
        .arg("--language")
        .arg("rust")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created template:"));

    let templates_dir = temp_dir.path().join(".config/.mucm/templates");

    // Verify core templates
    assert!(templates_dir.join("use_case_simple.hbs").exists());
    assert!(templates_dir.join("use_case_detailed.hbs").exists());
    assert!(templates_dir.join("overview.hbs").exists());

    // Verify language-specific template
    assert!(templates_dir.join("lang-rust/test.hbs").exists());

    // Verify template content is not empty
    let simple_template = fs::read_to_string(templates_dir.join("use_case_simple.hbs"))?;
    assert!(!simple_template.is_empty());
    assert!(simple_template.contains("{{")); // Should contain handlebars syntax

    let rust_template = fs::read_to_string(templates_dir.join("lang-rust/test.hbs"))?;
    assert!(!rust_template.is_empty());

    Ok(())
}

/// Test CLI workflow with custom configuration directories
#[test]
#[serial]
fn test_cli_with_custom_config_directories() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("init").current_dir(&temp_dir).assert().success();

    // Manually modify config to use custom directories
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path)?;

    // Update directories in config
    config_content = config_content.replace(
        "use_case_dir = \"docs/use-cases\"",
        "use_case_dir = \"custom/use-cases\"",
    );
    config_content = config_content.replace(
        "test_dir = \"tests/use-cases\"",
        "test_dir = \"custom/tests\"",
    );

    fs::write(&config_path, config_content)?;

    // Create the custom directories
    fs::create_dir_all(temp_dir.path().join("custom/use-cases"))?;
    fs::create_dir_all(temp_dir.path().join("custom/tests"))?;

    // Test that CLI respects custom directories
    let mut cmd = Command::cargo_bin("mucm")?;
    cmd.arg("create")
        .arg("Custom Dir Test")
        .arg("--category")
        .arg("custom")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created use case: UC-CUS-001"));

    // Verify file was created in custom directory
    let custom_use_case_file = temp_dir
        .path()
        .join("custom/use-cases/custom/UC-CUS-001.md");
    assert!(
        custom_use_case_file.exists(),
        "Use case should be in custom directory"
    );

    Ok(())
}
