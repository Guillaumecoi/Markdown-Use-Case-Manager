// Integration tests to verify that all commands properly enforce initialization
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use tempfile::TempDir;

/// Test that create command requires initialization
#[test]
#[serial]
fn test_create_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "test", "Test Use Case"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that add-scenario command requires initialization
#[test]
#[serial]
fn test_add_scenario_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["add-scenario", "UC-001", "Test Scenario"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that update-status command requires initialization
#[test]
#[serial]
fn test_update_status_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["update-status", "UC-001-S01", "--status", "implemented"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that list command requires initialization
#[test]
#[serial]
fn test_list_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that status command requires initialization
#[test]
#[serial]
fn test_status_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("status");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that regenerate command requires initialization
#[test]
#[serial]
fn test_regenerate_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("regenerate");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that persona create command requires initialization
#[test]
#[serial]
fn test_persona_create_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["persona", "create", "Test User"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that persona list command requires initialization
#[test]
#[serial]
fn test_persona_list_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["persona", "list"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that persona delete command requires initialization
#[test]
#[serial]
fn test_persona_delete_requires_init() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["persona", "delete", "Test User"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No markdown use case manager project found",
        ))
        .stderr(predicate::str::contains("Run 'mucm init' first"));
}

/// Test that commands that DON'T require init work without initialization
#[test]
#[serial]
fn test_commands_that_dont_require_init() {
    let temp_dir = TempDir::new().unwrap();

    // languages command should work without init
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("languages");
    cmd.assert().success();

    // methodologies command should work without init
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("methodologies");
    cmd.assert().success();

    // methodology-info command should work without init
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["methodology-info", "simple"]);
    cmd.assert().success();

    // help should work without init
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--help");
    cmd.assert().success();

    // version should work without init
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("--version");
    cmd.assert().success();
}

/// Test that all commands work AFTER initialization
#[test]
#[serial]
fn test_commands_work_after_init() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // create should work
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "test", "Test Use Case"]);
    cmd.assert().success();

    // list should work
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert().success();

    // status should work
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("status");
    cmd.assert().success();

    // add-scenario should work
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["add-scenario", "UC-TES-001", "Test Scenario"]);
    cmd.assert().success();

    // regenerate should work
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("regenerate");
    cmd.assert().success();

    // persona commands should work
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["persona", "create", "Test User", "--description", "A test user"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["persona", "list"]);
    cmd.assert().success();
}
