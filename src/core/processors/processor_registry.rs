// src/core/processors/processor_registry.rs
use super::methodology_processor::MethodologyRegistry;

/// Creates a default registry with all built-in methodology processors using the new system
pub fn create_default_registry() -> MethodologyRegistry {
    // Use the new MethodologyRegistry that has the updated names and processors
    MethodologyRegistry::new()
}

/// List all available methodology processors - kept for potential future use
#[allow(dead_code)]
pub fn list_available_methodologies() -> Vec<String> {
    vec![
        "feature".to_string(),
        "business".to_string(), 
        "developer".to_string(),
        "tester".to_string(),
    ]
}

/// Helper function to get methodology display information - kept for potential future use
#[allow(dead_code)]
pub fn get_methodology_info() -> Vec<(String, String, String)> {
    vec![
        ("feature".to_string(), "Feature Development".to_string(), "Feature-focused documentation for product managers and stakeholders".to_string()),
        ("business".to_string(), "Business Analysis".to_string(), "Business-focused methodology for strategic planning".to_string()),
        ("developer".to_string(), "Technical Development".to_string(), "Technical implementation documentation for development teams".to_string()),
        ("tester".to_string(), "Quality Assurance".to_string(), "Testing-focused methodology for quality assurance".to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_default_registry() {
        let registry = create_default_registry();
        
        assert!(registry.get_processor("feature").is_some());
        assert!(registry.get_processor("business").is_some());
        assert!(registry.get_processor("developer").is_some());
        assert!(registry.get_processor("tester").is_some());
        assert!(registry.get_processor("nonexistent").is_none());
    }
    
    #[test]
    fn test_list_available_methodologies() {
        let methodologies = list_available_methodologies();
        
        assert_eq!(methodologies.len(), 4);
        assert!(methodologies.contains(&"feature".to_string()));
        assert!(methodologies.contains(&"business".to_string()));
        assert!(methodologies.contains(&"developer".to_string()));
        assert!(methodologies.contains(&"tester".to_string()));
    }
    
    #[test]
    fn test_get_methodology_info() {
        let info = get_methodology_info();
        
        assert_eq!(info.len(), 4);
        
        // Check that we have the expected methodologies
        let methodology_names: Vec<String> = info.iter().map(|(name, _, _)| name.clone()).collect();
        assert!(methodology_names.contains(&"feature".to_string()));
        assert!(methodology_names.contains(&"business".to_string()));
        assert!(methodology_names.contains(&"developer".to_string()));
        assert!(methodology_names.contains(&"tester".to_string()));
    }
}