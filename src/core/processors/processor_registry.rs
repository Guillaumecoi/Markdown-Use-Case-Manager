// src/core/processors/processor_registry.rs
use super::unified_processor::UnifiedProcessor;
use super::methodology_processor::MethodologyRegistry;

/// Creates a default registry with all built-in methodology processors using the unified system
pub fn create_default_registry() -> MethodologyRegistry {
    let mut registry = MethodologyRegistry::empty();
    
    // Register the three core methodologies using unified processors
    registry.register("simple".to_string(), Box::new(UnifiedProcessor::simple()));
    registry.register("business".to_string(), Box::new(UnifiedProcessor::business()));
    registry.register("testing".to_string(), Box::new(UnifiedProcessor::testing()));
    
    registry
}

/// List all available methodology processors - kept for potential future use
#[allow(dead_code)]
pub fn list_available_methodologies() -> Vec<String> {
    vec![
        "simple".to_string(),
        "business".to_string(), 
        "testing".to_string(),
    ]
}

/// Helper function to get methodology display information - kept for potential future use
#[allow(dead_code)]
pub fn get_methodology_info() -> Vec<(String, String, String)> {
    let _registry = create_default_registry();
    
    vec![
        (
            "simple".to_string(),
            "Simple Use Cases".to_string(),
            "Rapid development with minimal ceremony. Ideal for prototyping and agile teams focused on speed.".to_string(),
        ),
        (
            "business".to_string(), 
            "Business Analysis".to_string(),
            "Comprehensive business analysis combining Cockburn and RUP approaches. Perfect for enterprise projects.".to_string(),
        ),
        (
            "testing".to_string(),
            "Test-Driven (BDD)".to_string(),
            "Behavior-Driven Development with executable specifications. Ideal for automated testing and CI/CD.".to_string(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_default_registry() {
        let registry = create_default_registry();
        
        assert!(registry.get_processor("simple").is_some());
        assert!(registry.get_processor("business").is_some());
        assert!(registry.get_processor("testing").is_some());
        assert!(registry.get_processor("nonexistent").is_none());
    }
    
    #[test]
    fn test_list_available_methodologies() {
        let methodologies = list_available_methodologies();
        
        assert_eq!(methodologies.len(), 3);
        assert!(methodologies.contains(&"simple".to_string()));
        assert!(methodologies.contains(&"business".to_string()));
        assert!(methodologies.contains(&"testing".to_string()));
    }
    
    #[test]
    fn test_get_methodology_info() {
        let info = get_methodology_info();
        
        assert_eq!(info.len(), 3);
        
        // Check that each methodology has proper info
        for (id, name, description) in info {
            assert!(!id.is_empty());
            assert!(!name.is_empty());
            assert!(!description.is_empty());
        }
    }
}