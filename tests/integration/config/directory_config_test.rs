// Integration tests for directory configuration options
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

/// Test default directory structure with separate TOML location
#[test]
#[serial]
fn test_default_directory_structure() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project with defaults
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Create a use case
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "feature", "New Feature"]);
    cmd.assert().success();

    // Verify default structure: TOML in use-cases-data, MD in docs/use-cases
    let toml_path = temp_dir.path().join("use-cases-data/feature/UC-FEA-001.toml");
    let md_path = temp_dir.path().join("docs/use-cases/feature/UC-FEA-001.md");
    
    assert!(toml_path.exists(), "TOML should be in use-cases-data by default");
    assert!(md_path.exists(), "MD should be in docs/use-cases");
    
    // Verify list works
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UC-FEA-001"))
        .stdout(predicate::str::contains("New Feature"));
}

/// Test custom TOML directory configuration
#[test]
#[serial]
fn test_custom_toml_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize project
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Update config to use different TOML directory
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path).unwrap();
    
    // Replace the default toml_dir
    config_content = config_content.replace(
        "toml_dir = \"use-cases-data\"",
        "toml_dir = \"data/sources\""
    );
    
    fs::write(&config_path, config_content).unwrap();

    // Create a use case
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "auth", "User Login"]);
    cmd.assert().success();

    // Verify TOML file is in custom directory
    let toml_path = temp_dir.path().join("data/sources/auth/UC-AUT-001.toml");
    assert!(toml_path.exists(), "TOML file should be in custom directory");

    // Verify MD file is still in standard location
    let md_path = temp_dir.path().join("docs/use-cases/auth/UC-AUT-001.md");
    assert!(md_path.exists(), "MD file should be in docs/use-cases");

    // Verify list command works
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UC-AUT-001"))
        .stdout(predicate::str::contains("User Login"));
}

/// Test regenerate command with custom TOML directory
#[test]
#[serial]
fn test_regenerate_with_custom_toml_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize and configure
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path).unwrap();
    config_content = config_content.replace(
        "toml_dir = \"use-cases-data\"",
        "toml_dir = \"sources/toml\""
    );
    fs::write(&config_path, config_content).unwrap();

    // Create a use case
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "test", "Test Case"]);
    cmd.assert().success();

    // Delete the markdown file
    let md_path = temp_dir.path().join("docs/use-cases/test/UC-TES-001.md");
    fs::remove_file(&md_path).unwrap();
    assert!(!md_path.exists());

    // Regenerate
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("regenerate");
    cmd.assert().success();

    // Verify markdown was regenerated
    assert!(md_path.exists(), "MD file should be regenerated");

    // Verify TOML is still in custom directory
    let toml_path = temp_dir.path().join("sources/toml/test/UC-TES-001.toml");
    assert!(toml_path.exists(), "TOML file should remain in custom directory");
}

/// Test colocated TOML and markdown (legacy mode)
#[test]
#[serial]
fn test_colocated_toml_and_markdown() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Remove toml_dir to use colocated mode
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path).unwrap();
    
    // Remove the toml_dir line entirely
    let lines: Vec<&str> = config_content.lines().collect();
    let filtered: Vec<&str> = lines.into_iter()
        .filter(|line| !line.contains("toml_dir"))
        .collect();
    config_content = filtered.join("\n");
    
    fs::write(&config_path, config_content).unwrap();

    // Create a use case
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "api", "API Endpoint"]);
    cmd.assert().success();

    // Both files should be in the same directory (use_case_dir)
    let toml_path = temp_dir.path().join("docs/use-cases/api/UC-API-001.toml");
    let md_path = temp_dir.path().join("docs/use-cases/api/UC-API-001.md");
    
    assert!(toml_path.exists(), "TOML should be in use_case_dir when toml_dir is not set");
    assert!(md_path.exists(), "MD should be in use_case_dir");

    // Verify they're truly colocated
    assert_eq!(
        toml_path.parent().unwrap(),
        md_path.parent().unwrap(),
        "TOML and MD should be in the same directory"
    );
}

/// Test custom use_case_dir configuration
#[test]
#[serial]
fn test_custom_use_case_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Update config to use custom directories
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path).unwrap();
    
    config_content = config_content.replace(
        "use_case_dir = \"docs/use-cases\"",
        "use_case_dir = \"documentation/requirements\""
    );
    config_content = config_content.replace(
        "toml_dir = \"use-cases-data\"",
        "toml_dir = \"data/requirements\""
    );
    
    fs::write(&config_path, config_content).unwrap();

    // Create a use case
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "requirement", "System Requirement"]);
    cmd.assert().success();

    // Verify files are in custom locations
    let toml_path = temp_dir.path().join("data/requirements/requirement/UC-REQ-001.toml");
    let md_path = temp_dir.path().join("documentation/requirements/requirement/UC-REQ-001.md");
    
    assert!(toml_path.exists(), "TOML should be in custom data directory");
    assert!(md_path.exists(), "MD should be in custom documentation directory");
}

/// Test that TOML directory is created automatically on first use
#[test]
#[serial]
fn test_toml_directory_auto_creation() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Update to a deeply nested custom path
    let config_path = temp_dir.path().join(".config/.mucm/mucm.toml");
    let mut config_content = fs::read_to_string(&config_path).unwrap();
    
    config_content = config_content.replace(
        "toml_dir = \"use-cases-data\"",
        "toml_dir = \"project/data/sources/use-cases\""
    );
    
    fs::write(&config_path, config_content).unwrap();

    // Create a use case (should auto-create the deep path)
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.args(["create", "--category", "data", "Data Model"]);
    cmd.assert().success();

    // Verify deep directory structure was created
    let toml_dir = temp_dir.path().join("project/data/sources/use-cases");
    assert!(toml_dir.exists(), "Deep TOML directory should be created automatically");
    
    let toml_file = toml_dir.join("data/UC-DAT-001.toml");
    assert!(toml_file.exists(), "TOML file should be in deeply nested directory");
}

/// Test directory configuration in multi-category project
#[test]
#[serial]
fn test_multi_category_directory_structure() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Create use cases in multiple categories
    for category in ["auth", "api", "database", "ui"] {
        let mut cmd = Command::cargo_bin("mucm").unwrap();
        cmd.current_dir(&temp_dir);
        cmd.args(["create", "--category", category, &format!("{} Feature", category.to_uppercase())]);
        cmd.assert().success();
    }

    // Verify structure is maintained across all categories
    // Just check that files exist in the correct directories
    let toml_dir = temp_dir.path().join("use-cases-data");
    let md_dir = temp_dir.path().join("docs/use-cases");
    
    for category in ["auth", "api", "database", "ui"] {
        let toml_cat_dir = toml_dir.join(category);
        let md_cat_dir = md_dir.join(category);
        
        assert!(toml_cat_dir.exists(), "TOML category dir should exist for {}", category);
        assert!(md_cat_dir.exists(), "MD category dir should exist for {}", category);
        
        // Count files - should be 1 of each type
        let toml_count = std::fs::read_dir(&toml_cat_dir).unwrap().count();
        let md_count = std::fs::read_dir(&md_cat_dir).unwrap().count();
        
        assert_eq!(toml_count, 1, "Should have 1 TOML file in {}", category);
        assert_eq!(md_count, 1, "Should have 1 MD file in {}", category);
    }

    // Verify list shows all
    let mut cmd = Command::cargo_bin("mucm").unwrap();
    cmd.current_dir(&temp_dir);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("auth"))
        .stdout(predicate::str::contains("api"))
        .stdout(predicate::str::contains("database"))
        .stdout(predicate::str::contains("ui"));
}
