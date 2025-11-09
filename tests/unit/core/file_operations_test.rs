// tests/unit/core/file_operations_test.rs
use markdown_use_case_manager::config::Config;
use markdown_use_case_manager::core::domain::entities::UseCase;
use markdown_use_case_manager::core::infrastructure::persistence::file_operations::FileOperations;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_test_file_exists() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create a test config with the temp directory as test_dir
    let mut config = Config::default();
    config.directories.test_dir = temp_path.to_string_lossy().to_string();

    // Create FileOperations instance
    let file_ops = FileOperations::new(config);

    // Create a test use case
    let use_case = UseCase::new(
        "UC-TEST-001".to_string(),
        "Test Use Case".to_string(),
        "Test".to_string(),
        "A test use case".to_string(),
        markdown_use_case_manager::core::domain::entities::Priority::Medium,
    );

    // Test that file doesn't exist initially
    assert!(!file_ops.test_file_exists(&use_case, "py"));

    // Create the category directory and test file manually
    let category_dir = temp_path.join("test");
    fs::create_dir_all(&category_dir).expect("Failed to create category dir");
    let test_file_path = category_dir.join("uc_test_001.py");
    fs::write(&test_file_path, "test content").expect("Failed to write test file");

    // Test that file now exists
    assert!(file_ops.test_file_exists(&use_case, "py"));

    // Test with different extension
    assert!(!file_ops.test_file_exists(&use_case, "js"));

    // Test with different use case
    let other_use_case = UseCase::new(
        "UC-OTHER-001".to_string(),
        "Other Use Case".to_string(),
        "Test".to_string(),
        "Another test use case".to_string(),
        markdown_use_case_manager::core::domain::entities::Priority::Medium,
    );
    assert!(!file_ops.test_file_exists(&other_use_case, "py"));
}

#[test]
fn test_save_test_file_creates_category_directory() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create a test config with the temp directory as test_dir
    let mut config = Config::default();
    config.directories.test_dir = temp_path.to_string_lossy().to_string();

    // Create FileOperations instance
    let file_ops = FileOperations::new(config);

    // Create a test use case
    let use_case = UseCase::new(
        "UC-TEST-002".to_string(),
        "Test Use Case 2".to_string(),
        "Feature".to_string(),
        "A test use case".to_string(),
        markdown_use_case_manager::core::domain::entities::Priority::Medium,
    );

    // Save a test file
    let test_content = "# Generated test file\nprint('Hello, World!')\n";
    file_ops
        .save_test_file(&use_case, test_content, "py")
        .expect("Failed to save test file");

    // Check that the category directory was created
    let category_dir = temp_path.join("feature");
    assert!(category_dir.exists());
    assert!(category_dir.is_dir());

    // Check that the test file was created with correct content
    let test_file_path = category_dir.join("uc_test_002.py");
    assert!(test_file_path.exists());
    assert!(test_file_path.is_file());

    let read_content = fs::read_to_string(&test_file_path).expect("Failed to read test file");
    assert_eq!(read_content, test_content);
}
