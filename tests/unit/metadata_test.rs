// Unit tests for Metadata struct and related functionality
use markdown_use_case_manager::core::models::Metadata;
use chrono::Utc;

/// Test Metadata::new() creates metadata with correct initial values
#[test]
fn test_metadata_new() {
    let metadata = Metadata::new();
    
    // Timestamps should be recent (within last few seconds)
    let now = Utc::now();
    let created_diff = now.signed_duration_since(metadata.created_at).num_seconds();
    let updated_diff = now.signed_duration_since(metadata.updated_at).num_seconds();
    
    assert!((0..3).contains(&created_diff), "Created timestamp should be recent");
    assert!((0..3).contains(&updated_diff), "Updated timestamp should be recent");
    
    // Initially, created and updated should be the same (or very close)
    let time_diff = metadata.updated_at.signed_duration_since(metadata.created_at).num_milliseconds();
    assert!(time_diff.abs() < 100, "Created and updated should be nearly the same initially");
}

/// Test Metadata::default() works the same as new()
#[test]
fn test_metadata_default() {
    let metadata1 = Metadata::new();
    let metadata2 = Metadata::default();
    
    // Timestamps should be very close (within a few milliseconds)
    let time_diff = metadata2.created_at.signed_duration_since(metadata1.created_at).num_milliseconds();
    assert!(time_diff.abs() < 100, "Default and new should create similar timestamps");
}

/// Test Metadata::touch() updates timestamp
#[test]
fn test_metadata_touch() {
    let mut metadata = Metadata::new();
    let original_created = metadata.created_at;
    let original_updated = metadata.updated_at;
    
    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    metadata.touch();
    
    // Created timestamp should not change
    assert_eq!(metadata.created_at, original_created);
    
    // Updated timestamp should change
    assert!(metadata.updated_at > original_updated);
    
    // Updated timestamp should be recent
    let now = Utc::now();
    let updated_diff = now.signed_duration_since(metadata.updated_at).num_seconds();
    assert!((0..2).contains(&updated_diff));
}

/// Test multiple touch() calls update timestamp correctly
#[test]
fn test_metadata_multiple_touch() {
    let mut metadata = Metadata::new();
    let mut last_updated = metadata.updated_at;
    
    for _ in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(1));
        metadata.touch();
        assert!(metadata.updated_at > last_updated);
        last_updated = metadata.updated_at;
    }
}

/// Test Metadata clone functionality
#[test]
fn test_metadata_clone() {
    let metadata = Metadata::new();
    let cloned = metadata.clone();
    
    assert_eq!(metadata.created_at, cloned.created_at);
    assert_eq!(metadata.updated_at, cloned.updated_at);
}

/// Test Metadata serialization and deserialization
#[test]
fn test_metadata_serialization() {
    let metadata = Metadata::new();
    
    // Test serialization (used internally)
    let json = serde_json::to_string(&metadata).expect("Failed to serialize");
    let deserialized: Metadata = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(metadata.created_at, deserialized.created_at);
    assert_eq!(metadata.updated_at, deserialized.updated_at);
}

/// Test Metadata timestamp tracking
#[test]
fn test_metadata_version_tracking() {
    let mut metadata = Metadata::new();
    let initial_updated = metadata.updated_at;
    
    // Simulate multiple modifications
    for _ in 1..=5 {
        std::thread::sleep(std::time::Duration::from_millis(1));
        metadata.touch();
    }
    
    // Final updated should be later than initial
    assert!(metadata.updated_at > initial_updated);
}

/// Test Metadata timestamp consistency
#[test]
fn test_metadata_timestamp_consistency() {
    let mut metadata = Metadata::new();
    let original_created = metadata.created_at;
    
    // Multiple touches should never change created_at
    for _ in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(1));
        metadata.touch();
        assert_eq!(metadata.created_at, original_created, "created_at should never change");
    }
    
    // But updated_at should keep changing
    let first_update = metadata.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(1));
    metadata.touch();
    assert!(metadata.updated_at > first_update, "updated_at should keep updating");
}

/// Test Metadata debug formatting
#[test]
fn test_metadata_debug() {
    let metadata = Metadata::new();
    let debug_str = format!("{:?}", metadata);
    
    assert!(debug_str.contains("Metadata"));
    assert!(debug_str.contains("created_at"));
    assert!(debug_str.contains("updated_at"));
}

/// Test Metadata timestamp boundaries
#[test]
fn test_metadata_version_boundaries() {
    let metadata = Metadata::new();
    
    // Created timestamp should be valid
    assert!(metadata.created_at.timestamp() > 0);
    
    // Updated timestamp should be valid  
    assert!(metadata.updated_at.timestamp() > 0);
}