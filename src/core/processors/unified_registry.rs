// src/core/processors/unified_registry.rs
use super::unified_processor::UnifiedProcessor;
use super::methodology_processor::MethodologyRegistry;

/// Creates a default registry using the unified processor system
#[allow(dead_code)]
pub fn create_unified_registry() -> MethodologyRegistry {
    let mut registry = MethodologyRegistry::new();
    
    // Register the four core methodologies using unified processors
    registry.register("feature".to_string(), Box::new(UnifiedProcessor::simple()));
    registry.register("business".to_string(), Box::new(UnifiedProcessor::business()));
    registry.register("developer".to_string(), Box::new(UnifiedProcessor::simple()));
    registry.register("tester".to_string(), Box::new(UnifiedProcessor::testing()));
    
    registry
}

/// Configuration-based registry builder for easy extension
#[allow(dead_code)]
pub struct RegistryBuilder {
    registry: MethodologyRegistry,
}

#[allow(dead_code)]
impl RegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: MethodologyRegistry::empty(),
        }
    }
    
    /// Create with the default registry (includes built-in methodologies)
    pub fn with_defaults() -> Self {
        Self {
            registry: MethodologyRegistry::new(),
        }
    }
    
    /// Add a custom methodology
    pub fn with_custom(mut self, id: String, processor: UnifiedProcessor) -> Self {
        self.registry.register(id, Box::new(processor));
        self
    }
    
    /// Build the final registry
    pub fn build(self) -> MethodologyRegistry {
        self.registry
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::processors::unified_processor::MethodologyConfig;
    use std::collections::HashMap;
    
    #[test]
    fn test_create_unified_registry() {
        let registry = create_unified_registry();
        
        assert!(registry.get_processor("feature").is_some());
        assert!(registry.get_processor("business").is_some());
        assert!(registry.get_processor("developer").is_some());
        assert!(registry.get_processor("tester").is_some());
        assert!(registry.get_processor("nonexistent").is_none());
        
        // Test that the processors have the right display names
        assert_eq!(registry.get_processor("feature").unwrap().display_name(), "Simple");
        assert_eq!(registry.get_processor("business").unwrap().display_name(), "Business Analysis");
        assert_eq!(registry.get_processor("tester").unwrap().display_name(), "Testing & QA");
    }
    
    #[test]
    fn test_registry_builder_default() {
        let registry = RegistryBuilder::with_defaults().build();
        
        assert!(registry.get_processor("feature").is_some());
        assert!(registry.get_processor("business").is_some());
        assert!(registry.get_processor("developer").is_some());
        assert!(registry.get_processor("tester").is_some());
    }
    
    #[test]
    fn test_registry_builder_custom() {
        let config = MethodologyConfig {
            id: "agile".to_string(),
            display_name: "Agile Methodology".to_string(),
            description: "Agile-focused use case development".to_string(),
            metadata_fields: HashMap::new(),
            processor_fn: None,
        };
        
        let custom_processor = UnifiedProcessor::custom(config);
        
        let registry = RegistryBuilder::with_defaults()
            .with_custom("agile".to_string(), custom_processor)
            .build();
        
        assert!(registry.get_processor("feature").is_some());
        assert!(registry.get_processor("agile").is_some());
        assert_eq!(registry.get_processor("agile").unwrap().display_name(), "Agile Methodology");
    }
    
    #[test]
    fn test_registry_builder_only_custom() {
        let config1 = MethodologyConfig {
            id: "microservices".to_string(),
            display_name: "Microservices".to_string(),
            description: "Microservices architecture focused".to_string(),
            metadata_fields: HashMap::new(),
            processor_fn: None,
        };
        
        let config2 = MethodologyConfig {
            id: "devops".to_string(),
            display_name: "DevOps".to_string(),
            description: "DevOps and CI/CD focused".to_string(),
            metadata_fields: HashMap::new(),
            processor_fn: None,
        };
        
        let registry = RegistryBuilder::new()
            .with_custom("microservices".to_string(), UnifiedProcessor::custom(config1))
            .with_custom("devops".to_string(), UnifiedProcessor::custom(config2))
            .build();
        
        // Should only have the custom methodologies
        assert!(registry.get_processor("microservices").is_some());
        assert!(registry.get_processor("devops").is_some());
        assert!(registry.get_processor("feature").is_none());
        assert!(registry.get_processor("business").is_none());
        assert!(registry.get_processor("developer").is_none());
        assert!(registry.get_processor("tester").is_none());
    }
}