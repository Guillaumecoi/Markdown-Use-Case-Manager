//! Unified persistence layer tests
//!
//! This module tests both TOML and SQLite backends with identical test suites
//! to ensure feature parity and correctness.

use markdown_use_case_manager::core::{
    SqliteUseCaseRepository, UseCase, UseCaseRepository,
};
use serial_test::serial;
use std::env;
use tempfile::TempDir;

/// Test helper: Create a test use case
fn create_test_use_case() -> UseCase {
    UseCase::new(
        "UC-TEST-001".to_string(),
        "Test Use Case".to_string(),
        "test".to_string(),
        "A test use case for persistence testing".to_string(),
        "medium".to_string(),
    ).unwrap()
}

/// Test helper: Create a test use case with extra fields
fn create_test_use_case_with_extra() -> UseCase {
    let mut use_case = create_test_use_case();
    use_case.extra.insert("custom_field".to_string(), serde_json::json!("custom_value"));
    use_case.extra.insert("number_field".to_string(), serde_json::json!(42));
    use_case
}

/// Test helper: Create TOML repository for testing
fn create_toml_repository() -> (TempDir, Box<dyn UseCaseRepository>) {
    let temp_dir = TempDir::new().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    // Initialize basic config structure using the same approach as config tests
    use markdown_use_case_manager::config::ConfigFileManager;
    let config = markdown_use_case_manager::config::Config::default();
    ConfigFileManager::save_in_dir(&config, ".").unwrap();

    let config = markdown_use_case_manager::config::Config::load().unwrap();
    let repo = Box::new(markdown_use_case_manager::core::TomlUseCaseRepository::new(config)) as Box<dyn UseCaseRepository>;

    (temp_dir, repo)
}

/// Test helper: Create SQLite repository for testing
fn create_sqlite_repository() -> (TempDir, Box<dyn UseCaseRepository>) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let repo = Box::new(SqliteUseCaseRepository::new(&db_path).unwrap()) as Box<dyn UseCaseRepository>;

    // Create markdown directory that the SQLite repository expects
    std::fs::create_dir_all(temp_dir.path().join("markdown")).unwrap();

    (temp_dir, repo)
}

/// Run all persistence tests against a repository
fn run_all_tests(repo: &dyn UseCaseRepository) {
    test_backend_name(repo);
    test_health_check(repo);
    test_save_and_load(repo);
    test_save_with_extra_fields(repo);
    test_delete(repo);
    test_exists(repo);
    test_find_by_category(repo);
    test_find_by_priority(repo);
    test_search_by_title(repo);
    test_save_batch(repo);
    test_delete_batch(repo);
    test_load_all(repo);
    test_save_markdown(repo);
}

#[test]
#[serial]
fn test_toml_backend() {
    let (_temp_dir, repo) = create_toml_repository();
    run_all_tests(&*repo);
}

#[test]
#[serial]
fn test_sqlite_backend() {
    let (_temp_dir, repo) = create_sqlite_repository();
    run_all_tests(&*repo);
}

// ===== Individual Test Functions =====

fn test_backend_name(repo: &dyn UseCaseRepository) {
    // Test backend identification
    let name = repo.backend_name();
    assert!(name == "toml" || name == "sqlite", "Backend name should be 'toml' or 'sqlite', got: {}", name);
}

fn test_health_check(repo: &dyn UseCaseRepository) {
    // Test health check
    repo.health_check().expect("Health check should pass");
}

fn test_save_and_load(repo: &dyn UseCaseRepository) {
    let use_case = create_test_use_case();

    // Save
    repo.save(&use_case).expect("Save should succeed");

    // Load by ID
    let loaded = repo.load_by_id(&use_case.id).expect("Load by ID should succeed");
    assert!(loaded.is_some(), "Use case should be found");
    let loaded = loaded.unwrap();

    // Verify fields
    assert_eq!(loaded.id, use_case.id);
    assert_eq!(loaded.title, use_case.title);
    assert_eq!(loaded.category, use_case.category);
    assert_eq!(loaded.description, use_case.description);
    assert_eq!(loaded.priority, use_case.priority);
    assert_eq!(loaded.status(), use_case.status());
}

fn test_save_with_extra_fields(repo: &dyn UseCaseRepository) {
    let use_case = create_test_use_case_with_extra();

    // Save
    repo.save(&use_case).expect("Save with extra fields should succeed");

    // Load and verify extra fields
    let loaded = repo.load_by_id(&use_case.id).expect("Load should succeed").unwrap();

    assert_eq!(loaded.extra.get("custom_field"), Some(&serde_json::json!("custom_value")));
    assert_eq!(loaded.extra.get("number_field"), Some(&serde_json::json!(42)));
}

fn test_delete(repo: &dyn UseCaseRepository) {
    let use_case = create_test_use_case();

    // Save then delete
    repo.save(&use_case).expect("Save should succeed");
    assert!(repo.exists(&use_case.id).unwrap(), "Use case should exist before delete");

    repo.delete(&use_case.id).expect("Delete should succeed");
    assert!(!repo.exists(&use_case.id).unwrap(), "Use case should not exist after delete");
}

fn test_exists(repo: &dyn UseCaseRepository) {
    let use_case = create_test_use_case();

    assert!(!repo.exists(&use_case.id).unwrap(), "Use case should not exist initially");

    repo.save(&use_case).expect("Save should succeed");
    assert!(repo.exists(&use_case.id).unwrap(), "Use case should exist after save");
}

fn test_find_by_category(repo: &dyn UseCaseRepository) {
    let use_case1 = UseCase::new(
        "UC-CAT-001".to_string(),
        "Category Test 1".to_string(),
        "test_cat".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();
    let use_case2 = UseCase::new(
        "UC-CAT-002".to_string(),
        "Category Test 2".to_string(),
        "test_cat".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();
    let use_case3 = UseCase::new(
        "UC-CAT-003".to_string(),
        "Category Test 3".to_string(),
        "other_cat".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();

    // Save all
    repo.save(&use_case1).unwrap();
    repo.save(&use_case2).unwrap();
    repo.save(&use_case3).unwrap();

    // Find by category
    let found = repo.find_by_category("test_cat").unwrap();
    assert_eq!(found.len(), 2, "Should find 2 use cases in test_cat category");

    let ids: Vec<String> = found.iter().map(|uc| uc.id.clone()).collect();
    assert!(ids.contains(&"UC-CAT-001".to_string()));
    assert!(ids.contains(&"UC-CAT-002".to_string()));
}

fn test_find_by_priority(repo: &dyn UseCaseRepository) {
    let use_case1 = UseCase::new(
        "UC-PRI-001".to_string(),
        "Priority Test 1".to_string(),
        "test".to_string(),
        "".to_string(),
        "high".to_string(),
    ).unwrap();
    let use_case2 = UseCase::new(
        "UC-PRI-002".to_string(),
        "Priority Test 2".to_string(),
        "test".to_string(),
        "".to_string(),
        "high".to_string(),
    ).unwrap();
    let use_case3 = UseCase::new(
        "UC-PRI-003".to_string(),
        "Priority Test 3".to_string(),
        "test".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();

    // Save all
    repo.save(&use_case1).unwrap();
    repo.save(&use_case2).unwrap();
    repo.save(&use_case3).unwrap();

    // Find by priority
    let found = repo.find_by_priority("high").unwrap();
    assert_eq!(found.len(), 2, "Should find 2 high priority use cases");

    let ids: Vec<String> = found.iter().map(|uc| uc.id.clone()).collect();
    assert!(ids.contains(&"UC-PRI-001".to_string()));
    assert!(ids.contains(&"UC-PRI-002".to_string()));
}

fn test_search_by_title(repo: &dyn UseCaseRepository) {
    let use_case1 = UseCase::new(
        "UC-SEARCH-001".to_string(),
        "User Authentication Flow".to_string(),
        "auth".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();
    let use_case2 = UseCase::new(
        "UC-SEARCH-002".to_string(),
        "Data Processing Pipeline".to_string(),
        "data".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();
    let use_case3 = UseCase::new(
        "UC-SEARCH-003".to_string(),
        "User Profile Management".to_string(),
        "user".to_string(),
        "".to_string(),
        "medium".to_string(),
    ).unwrap();

    // Save all
    repo.save(&use_case1).unwrap();
    repo.save(&use_case2).unwrap();
    repo.save(&use_case3).unwrap();

    // Search by title
    let found = repo.search_by_title("user").unwrap();
    assert_eq!(found.len(), 2, "Should find 2 use cases containing 'user'");

    let ids: Vec<String> = found.iter().map(|uc| uc.id.clone()).collect();
    assert!(ids.contains(&"UC-SEARCH-001".to_string()));
    assert!(ids.contains(&"UC-SEARCH-003".to_string()));
}

fn test_save_batch(repo: &dyn UseCaseRepository) {
    let use_cases = vec![
        UseCase::new("UC-BATCH-001".to_string(), "Batch Test 1".to_string(), "batch".to_string(), "".to_string(), "medium".to_string()).unwrap(),
        UseCase::new("UC-BATCH-002".to_string(), "Batch Test 2".to_string(), "batch".to_string(), "".to_string(), "medium".to_string()).unwrap(),
        UseCase::new("UC-BATCH-003".to_string(), "Batch Test 3".to_string(), "batch".to_string(), "".to_string(), "medium".to_string()).unwrap(),
    ];

    // Save batch
    repo.save_batch(&use_cases).unwrap();

    // Verify all were saved
    for use_case in &use_cases {
        assert!(repo.exists(&use_case.id).unwrap(), "Use case {} should exist", use_case.id);
    }

    // Verify load_all includes them
    let all = repo.load_all().unwrap();
    assert!(all.len() >= 3, "Should have at least 3 use cases");
}

fn test_delete_batch(repo: &dyn UseCaseRepository) {
    let use_cases = vec![
        UseCase::new("UC-DEL-001".to_string(), "Delete Test 1".to_string(), "delete".to_string(), "".to_string(), "medium".to_string()).unwrap(),
        UseCase::new("UC-DEL-002".to_string(), "Delete Test 2".to_string(), "delete".to_string(), "".to_string(), "medium".to_string()).unwrap(),
        UseCase::new("UC-DEL-003".to_string(), "Delete Test 3".to_string(), "delete".to_string(), "".to_string(), "medium".to_string()).unwrap(),
    ];

    // Save all first
    for use_case in &use_cases {
        repo.save(use_case).unwrap();
    }

    // Verify they exist
    for use_case in &use_cases {
        assert!(repo.exists(&use_case.id).unwrap());
    }

    // Delete batch
    let ids: Vec<&str> = use_cases.iter().map(|uc| uc.id.as_str()).collect();
    repo.delete_batch(&ids).unwrap();

    // Verify they're gone
    for use_case in &use_cases {
        assert!(!repo.exists(&use_case.id).unwrap(), "Use case {} should be deleted", use_case.id);
    }
}

fn test_load_all(repo: &dyn UseCaseRepository) {
    let use_cases = vec![
        UseCase::new("UC-ALL-001".to_string(), "Load All Test 1".to_string(), "load".to_string(), "".to_string(), "medium".to_string()).unwrap(),
        UseCase::new("UC-ALL-002".to_string(), "Load All Test 2".to_string(), "load".to_string(), "".to_string(), "medium".to_string()).unwrap(),
    ];

    // Save all
    for use_case in &use_cases {
        repo.save(use_case).unwrap();
    }

    // Load all
    let all = repo.load_all().unwrap();
    assert!(all.len() >= 2, "Should have at least 2 use cases");

    // Verify our test cases are included
    let ids: Vec<String> = all.iter().map(|uc| uc.id.clone()).collect();
    assert!(ids.contains(&"UC-ALL-001".to_string()));
    assert!(ids.contains(&"UC-ALL-002".to_string()));
}

fn test_save_markdown(repo: &dyn UseCaseRepository) {
    let use_case = create_test_use_case();
    let markdown_content = "# Test Markdown\n\nThis is a test use case.".to_string();

    // Save use case first
    repo.save(&use_case).unwrap();

    // Save markdown
    repo.save_markdown(&use_case.id, &markdown_content).unwrap();

    // Note: We can't easily verify markdown content without backend-specific code
    // This test mainly ensures the method doesn't panic

    // Clean up: delete the use case (which also removes markdown files for TOML backend)
    repo.delete(&use_case.id).unwrap();
}