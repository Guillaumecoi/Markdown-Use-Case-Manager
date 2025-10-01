// Unit tests for Priority enum and related functionality
use use_case_manager::core::models::Priority;
use std::str::FromStr;

/// Test Priority enum variants exist and have correct debug representation
#[test]
fn test_priority_enum_variants() {
    let priority = Priority::Low;
    assert_eq!(format!("{:?}", priority), "Low");
    
    let priority = Priority::Medium;
    assert_eq!(format!("{:?}", priority), "Medium");
    
    let priority = Priority::High;
    assert_eq!(format!("{:?}", priority), "High");
    
    let priority = Priority::Critical;
    assert_eq!(format!("{:?}", priority), "Critical");
}

/// Test Priority Display trait implementation
#[test]
fn test_priority_display() {
    assert_eq!(Priority::Low.to_string(), "LOW");
    assert_eq!(Priority::Medium.to_string(), "MEDIUM");
    assert_eq!(Priority::High.to_string(), "HIGH");
    assert_eq!(Priority::Critical.to_string(), "CRITICAL");
}

/// Test Priority FromStr trait implementation with valid inputs
#[test]
fn test_priority_from_str_valid() {
    assert_eq!(Priority::from_str("low").unwrap(), Priority::Low);
    assert_eq!(Priority::from_str("LOW").unwrap(), Priority::Low);
    assert_eq!(Priority::from_str("Low").unwrap(), Priority::Low);
    
    assert_eq!(Priority::from_str("medium").unwrap(), Priority::Medium);
    assert_eq!(Priority::from_str("MEDIUM").unwrap(), Priority::Medium);
    assert_eq!(Priority::from_str("Medium").unwrap(), Priority::Medium);
    
    assert_eq!(Priority::from_str("high").unwrap(), Priority::High);
    assert_eq!(Priority::from_str("HIGH").unwrap(), Priority::High);
    assert_eq!(Priority::from_str("High").unwrap(), Priority::High);
    
    assert_eq!(Priority::from_str("critical").unwrap(), Priority::Critical);
    assert_eq!(Priority::from_str("CRITICAL").unwrap(), Priority::Critical);
    assert_eq!(Priority::from_str("Critical").unwrap(), Priority::Critical);
}

/// Test Priority FromStr trait implementation with invalid inputs
#[test]
fn test_priority_from_str_invalid() {
    assert!(Priority::from_str("invalid").is_err());
    assert!(Priority::from_str("urgent").is_err());
    assert!(Priority::from_str("").is_err());
    assert!(Priority::from_str("123").is_err());
    
    let error = Priority::from_str("invalid").unwrap_err();
    assert!(error.contains("Invalid priority: invalid"));
}

/// Test Priority equality and PartialEq
#[test]
fn test_priority_equality() {
    assert_eq!(Priority::Low, Priority::Low);
    assert_eq!(Priority::Medium, Priority::Medium);
    assert_eq!(Priority::High, Priority::High);
    assert_eq!(Priority::Critical, Priority::Critical);
    
    assert_ne!(Priority::Low, Priority::Medium);
    assert_ne!(Priority::Medium, Priority::High);
    assert_ne!(Priority::High, Priority::Critical);
}

/// Test Priority clone functionality
#[test]
fn test_priority_clone() {
    let priority = Priority::High;
    let cloned = priority.clone();
    assert_eq!(priority, cloned);
}

/// Test Priority serialization and deserialization
#[test]
fn test_priority_serialization() {
    let priority = Priority::Critical;
    let serialized = serde_json::to_string(&priority).expect("Failed to serialize");
    let deserialized: Priority = serde_json::from_str(&serialized).expect("Failed to deserialize");
    assert_eq!(priority, deserialized);
    
    // Test all variants
    let priorities = vec![Priority::Low, Priority::Medium, Priority::High, Priority::Critical];
    for original in priorities {
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Priority = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }
}

/// Test Priority in collections (Hash trait)
#[test]
fn test_priority_in_collections() {
    use std::collections::HashMap;
    
    let mut priority_map = HashMap::new();
    priority_map.insert(Priority::Low, "Low priority tasks");
    priority_map.insert(Priority::Medium, "Medium priority tasks");
    priority_map.insert(Priority::High, "High priority tasks");
    priority_map.insert(Priority::Critical, "Critical priority tasks");
    
    assert_eq!(priority_map.get(&Priority::Low), Some(&"Low priority tasks"));
    assert_eq!(priority_map.get(&Priority::Critical), Some(&"Critical priority tasks"));
    assert_eq!(priority_map.len(), 4);
}

/// Test Priority in vector operations
#[test]
fn test_priority_in_vectors() {
    let priorities = [Priority::Medium, Priority::High, Priority::Low, Priority::Critical];
    
    assert!(priorities.contains(&Priority::Medium));
    assert!(priorities.contains(&Priority::Critical));
    assert!(priorities.contains(&Priority::Low)); // Simplified assertion
    
    let high_priority_count = priorities.iter().filter(|&p| *p == Priority::High).count();
    assert_eq!(high_priority_count, 1);
}

/// Test Priority case insensitive parsing edge cases
#[test]
fn test_priority_case_insensitive_edge_cases() {
    assert_eq!(Priority::from_str("lOw").unwrap(), Priority::Low);
    assert_eq!(Priority::from_str("mEdIuM").unwrap(), Priority::Medium);
    assert_eq!(Priority::from_str("HiGh").unwrap(), Priority::High);
    assert_eq!(Priority::from_str("cRiTiCaL").unwrap(), Priority::Critical);
}