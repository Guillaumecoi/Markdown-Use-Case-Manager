// Tests for the new modular CLI architecture
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use tempfile::TempDir;

/// Test that CLI without args enters interactive mode (just check it starts)
#[test]
#[serial]
fn test_cli_no_args_starts_interactive_mode() {
    let temp_dir = TempDir::new().unwrap();

    // First initialize a project
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Test that running without args shows interactive welcome
    // We'll timeout quickly since we can't easily automate the interactive input
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.timeout(std::time::Duration::from_secs(2));
    
    // The command will timeout but we can check that it started interactive mode
    let result = cmd.assert().failure(); // Will fail due to timeout
    
    // Check that the stdout contains the interactive mode welcome
    result.stdout(predicate::str::contains("Interactive Mode"));
}

/// Test interactive flag starts interactive mode
#[test]
#[serial]
fn test_cli_interactive_flag_starts_interactive_mode() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Test -i flag
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("-i");
    cmd.timeout(std::time::Duration::from_secs(2));
    
    let result = cmd.assert().failure(); // Will fail due to timeout
    result.stdout(predicate::str::contains("Interactive Mode"));
}

/// Test interactive command starts interactive mode
#[test]
#[serial]
fn test_cli_interactive_command_starts_interactive_mode() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Test interactive command
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("interactive");
    cmd.timeout(std::time::Duration::from_secs(2));
    
    let result = cmd.assert().failure(); // Will fail due to timeout
    result.stdout(predicate::str::contains("Interactive Mode"));
}

/// Test that all original commands still work (backward compatibility)
#[test]
#[serial]
fn test_cli_backward_compatibility() {
    let temp_dir = TempDir::new().unwrap();

    // Test init
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project initialized"));

    // Test create
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "test", "Test Use Case"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created use case: UC-TES-001"));

    // Test list
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UC-TES-001"))
        .stdout(predicate::str::contains("Test Use Case"));

    // Test add-scenario
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["add-scenario", "UC-TES-001", "Test Scenario"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Added scenario: UC-TES-001-S01"));

    // Test update-status
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["update-status", "UC-TES-001-S01", "--status", "implemented"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Updated scenario UC-TES-001-S01"));

    // Test status
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project Status"))
        .stdout(predicate::str::contains("Total Use Cases: 1"));

    // Test languages
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("languages");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available programming languages"));
}

/// Test error handling when trying to run commands without initialization
#[test]
#[serial]
fn test_cli_error_handling_no_project() {
    let temp_dir = TempDir::new().unwrap();

    // Test create without initialization
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "test", "Test Use Case"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No markdown use case manager project found"));

    // Test list without initialization
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No markdown use case manager project found"));
}

/// Test that help command includes new options
#[test]
fn test_cli_help_includes_interactive_options() {
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Markdown Use Case Manager"))
        .stdout(predicate::str::contains("-i, --interactive"))
        .stdout(predicate::str::contains("interactive    Enter interactive mode"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"));
}

/// Test mixed mode usage (script then verify with script commands)
#[test]
#[serial]
fn test_mixed_mode_usage_script_only() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize with script mode
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Create use case with script mode
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "mixed", "Mixed Mode Test"]);
    cmd.assert().success();

    // Verify with script mode list command
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UC-MIX-001"))
        .stdout(predicate::str::contains("Mixed Mode Test"));

    // Add scenario with script mode
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["add-scenario", "UC-MIX-001", "Script Scenario"]);
    cmd.assert().success();

    // Verify scenario was added
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UC-MIX-001-S01"))
        .stdout(predicate::str::contains("Script Scenario"));
}