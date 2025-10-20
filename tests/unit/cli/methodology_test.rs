// tests/unit/cli_methodology_test.rs
use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::path::PathBuf;
use tempfile::TempDir;

fn with_temp_dir<F, R>(test_fn: F) -> R
where
    F: FnOnce(&PathBuf) -> R,
{
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let original_dir = env::current_dir().expect("Failed to get current directory");
    
    // Change to temp directory
    env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");
    
    // Create necessary config directory structure
    std::fs::create_dir_all(".config/.mucm").expect("Failed to create config directory");
    
    // Run the test and capture result
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        test_fn(&temp_dir.path().to_path_buf())
    }));
    
    // Always try to restore original directory, but don't panic if it fails
    let _ = env::set_current_dir(original_dir);
    
    // Re-throw any panic that occurred in the test
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

#[test]
fn test_cli_methodologies_command_basic() {
    with_temp_dir(|temp_dir| {
        // Initialize project first
        let mut cmd = Command::cargo_bin("mucm").unwrap();
        cmd.current_dir(temp_dir);
        cmd.arg("init");
        cmd.assert().success();

        // Test methodologies command
        let mut cmd = Command::cargo_bin("mucm").unwrap();
        cmd.current_dir(temp_dir);
        cmd.arg("methodologies");
        let output = cmd.assert().success();
        
        // Just check that it runs without checking exact content for now
        output.stdout(predicate::str::contains("methodologies"));
    });
}

#[test] 
fn test_cli_create_with_methodology_basic() {
    with_temp_dir(|temp_dir| {
        // Initialize project first
        let mut cmd = Command::cargo_bin("mucm").unwrap();
        cmd.current_dir(temp_dir);
        cmd.arg("init");
        cmd.assert().success();

        // Create use case with business methodology
        let mut cmd = Command::cargo_bin("mucm").unwrap();
        cmd.current_dir(temp_dir);
        cmd.arg("create")
            .arg("Test Use Case")
            .arg("--category").arg("Testing")
            .arg("--methodology").arg("business");
        cmd.assert().success();
    });
}

#[test]
fn test_cli_help_includes_methodology_commands() {
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("methodologies"))
        .stdout(predicate::str::contains("methodology-info"))
        .stdout(predicate::str::contains("regenerate"));
}