// CLI integration tests - comprehensive testing of command line interface
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

/// Test CLI help command shows usage information
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Use Case Manager"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"));
}

/// Test CLI version command shows version information
#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ucm"))
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test CLI init command in temporary directory
#[test] 
fn test_cli_init_creates_project_structure() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initializing use case manager project"))
        .stdout(predicate::str::contains("Project initialized"))
        .stdout(predicate::str::contains("Configuration saved to .config/ucm.toml"));
    
    // Verify project structure was created
    assert!(temp_dir.path().join(".config").exists());
    assert!(temp_dir.path().join(".config/ucm.toml").exists());
    assert!(temp_dir.path().join("docs/use-cases").exists());
    assert!(temp_dir.path().join("tests/use-cases").exists());
    
    // Verify config file content
    let config_content = fs::read_to_string(temp_dir.path().join(".config/ucm.toml")).unwrap();
    assert!(config_content.contains("[project]"));
    assert!(config_content.contains("[directories]"));
    assert!(config_content.contains("[metadata]"));
}

/// Test CLI create command creates use case
#[test]
fn test_cli_create_use_case() {
    let temp_dir = TempDir::new().unwrap();
    
    // First initialize project
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    // Then create a use case
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "auth", "User Login"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created use case: UC-AUT-001"));
    
    // Verify use case file was created
    let use_case_file = temp_dir.path().join("docs/use-cases/auth/UC-AUT-001.md");
    assert!(use_case_file.exists());
    
    // Verify file content
    let content = fs::read_to_string(&use_case_file).unwrap();
    assert!(content.contains("User Login"));
    assert!(content.contains("# User Login"));
    assert!(content.contains("## Description"));
    assert!(content.contains("## Scenarios"));
}

/// Test CLI create command with description
#[test]
fn test_cli_create_use_case_with_description() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize project
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    // Create use case with description
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&[
        "create", 
        "--category", "security", 
        "--description", "Handle user authentication and session management",
        "User Authentication"
    ]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created use case: UC-SEC-001"));
    
    // Verify content includes description
    let use_case_file = temp_dir.path().join("docs/use-cases/security/UC-SEC-001.md");
    let content = fs::read_to_string(&use_case_file).unwrap();
    assert!(content.contains("Handle user authentication and session management"));
}

/// Test CLI list command shows use cases
#[test]
fn test_cli_list_use_cases() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize and create some use cases
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    // Create first use case
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "auth", "User Login"]);
    cmd.assert().success();
    
    // Create second use case
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "profile", "Update Profile"]);
    cmd.assert().success();
    
    // List use cases
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Use Cases"))
        .stdout(predicate::str::contains("UC-AUT-001"))
        .stdout(predicate::str::contains("UC-PRO-001"))
        .stdout(predicate::str::contains("User Login"))
        .stdout(predicate::str::contains("Update Profile"));
}

/// Test CLI status command shows project statistics
#[test]
fn test_cli_status_shows_statistics() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize and create use cases
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    // Create a use case
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "test", "Test Case"]);
    cmd.assert().success();
    
    // Check status
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("status");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project Status"))
        .stdout(predicate::str::contains("Total Use Cases: 1"))
        .stdout(predicate::str::contains("Total Scenarios: 0"));
}

/// Test CLI add-scenario command
#[test]
fn test_cli_add_scenario() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize and create use case
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "test", "Test Case"]);
    cmd.assert().success();
    
    // Add scenario
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&[
        "add-scenario", 
        "UC-TES-001", 
        "Happy Path",
        "--description",
        "User successfully completes the task"
    ]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Added scenario: UC-TES-001-S01"));
    
    // Verify scenario was added to use case file
    let use_case_file = temp_dir.path().join("docs/use-cases/test/UC-TES-001.md");
    let content = fs::read_to_string(&use_case_file).unwrap();
    assert!(content.contains("Happy Path"));
    assert!(content.contains("UC-TES-001-S01"));
}

/// Test CLI update-status command
#[test]
fn test_cli_update_scenario_status() {
    let temp_dir = TempDir::new().unwrap();
    
    // Setup: init, create use case, add scenario
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "test", "Test Case"]);
    cmd.assert().success();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["add-scenario", "UC-TES-001", "Test Scenario"]);
    cmd.assert().success();
    
    // Update scenario status
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["update-status", "UC-TES-001-S01", "--status", "in_progress"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Updated scenario UC-TES-001-S01 status"));
}

/// Test CLI overview command generates overview
#[test]
fn test_cli_overview_generation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Setup project with use cases
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "auth", "Login"]);
    cmd.assert().success();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "profile", "Update Profile"]);
    cmd.assert().success();
    
    // Generate overview
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("overview");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated overview"));
    
    // Verify overview file was created
    let overview_file = temp_dir.path().join("docs/use-cases/README.md");
    assert!(overview_file.exists());
    
    let content = fs::read_to_string(&overview_file).unwrap();
    assert!(content.contains("Use Cases Overview") || content.contains("Overview"));
    
    // More flexible assertions for overview content
    assert!(content.contains("UC-AUT-001") || content.contains("Login"));
    assert!(content.contains("UC-PRO-001") || content.contains("Update Profile"));
}

/// Test CLI error handling for missing project
#[test]
fn test_cli_error_no_project() {
    let temp_dir = TempDir::new().unwrap();
    
    // Try to create use case without initializing
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "test", "Test"]);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No use case manager project found"))
        .stderr(predicate::str::contains("Run 'ucm init' first"));
}

/// Test CLI error handling for invalid commands
#[test]
fn test_cli_invalid_command() {
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.arg("invalid-command");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

/// Test CLI error handling for missing arguments
#[test]
fn test_cli_missing_arguments() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize project
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    // Try create without title
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "test"]);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

/// Test CLI workflow: complete use case lifecycle
#[test]
fn test_cli_complete_workflow() {
    let temp_dir = TempDir::new().unwrap();
    
    // 1. Initialize project
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();
    
    // 2. Create use case
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["create", "--category", "workflow", "Complete Workflow"]);
    cmd.assert().success();
    
    // 3. Add multiple scenarios
    for i in 1..=3 {
        let mut cmd = Command::cargo_bin("ucm").unwrap();
        cmd.current_dir(&temp_dir);
        cmd.args(&[
            "add-scenario", 
            "UC-WOR-001", 
            &format!("Scenario {}", i),
            "--description",
            &format!("Description for scenario {}", i)
        ]);
        cmd.assert().success();
    }
    
    // 4. Update scenario statuses
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["update-status", "UC-WOR-001-S01", "--status", "implemented"]);
    cmd.assert().success();
    
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(&["update-status", "UC-WOR-001-S02", "--status", "tested"]);
    cmd.assert().success();
    
    // 5. Check final status
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Total Use Cases: 1"))
        .stdout(predicate::str::contains("Total Scenarios: 3"));
    
    // 6. Generate overview
    let mut cmd = Command::cargo_bin("ucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("overview");
    cmd.assert().success();
    
    // Verify all files exist and have correct content
    assert!(temp_dir.path().join("docs/use-cases/workflow/UC-WOR-001.md").exists());
    assert!(temp_dir.path().join("docs/use-cases/README.md").exists());
    
    let use_case_content = fs::read_to_string(
        temp_dir.path().join("docs/use-cases/workflow/UC-WOR-001.md")
    ).unwrap();
    assert!(use_case_content.contains("Scenario 1"));
    assert!(use_case_content.contains("Scenario 2")); 
    assert!(use_case_content.contains("Scenario 3"));
}