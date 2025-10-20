// src/core/processors/methodology_processor.rs
use crate::core::models::Scenario;
use serde_json::Value;
use std::collections::HashMap;

/// Context information for use case processing
#[derive(Debug, Clone)]
pub struct UseCaseContext {
    #[allow(dead_code)]
    pub use_case_id: String,
    #[allow(dead_code)]
    pub category: String,
    pub business_context: std::collections::HashMap<String, String>,
}

/// Processed scenarios grouped by methodology-specific categories
#[derive(Debug)]
pub struct ProcessedScenarios {
    pub primary_flows: Vec<Scenario>,
    pub alternative_flows: Vec<Scenario>,
    pub error_flows: Vec<Scenario>,
    pub methodology_data: HashMap<String, Value>,
}

/// Trait for methodology-specific scenario processing
pub trait MethodologyProcessor: Send + Sync {
    /// Get the display name for this methodology
    fn display_name(&self) -> &str;
    
    /// Get a description of this methodology
    fn description(&self) -> &str;
    
    /// Process scenarios according to this methodology's rules
    fn process_scenarios(&self, scenarios: &[Scenario], context: &UseCaseContext) -> ProcessedScenarios;
}

/// Registry for methodology processors
pub struct MethodologyRegistry {
    processors: HashMap<String, Box<dyn MethodologyProcessor>>,
}

impl std::fmt::Debug for MethodologyRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MethodologyRegistry")
            .field("processors", &self.processors.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl MethodologyRegistry {
    pub fn new() -> Self {
        let mut processors: HashMap<String, Box<dyn MethodologyProcessor>> = HashMap::new();
        
        // Register the four core methodologies with new names and dedicated processors
        processors.insert("feature".to_string(), Box::new(super::methodologies::FeatureProcessor::new()));
        processors.insert("business".to_string(), Box::new(super::methodologies::BusinessProcessor::new()));
        processors.insert("developer".to_string(), Box::new(super::methodologies::DeveloperProcessor::new()));
        processors.insert("tester".to_string(), Box::new(super::methodologies::TestingProcessor::new()));
        
        Self { processors }
    }
    
    /// Create an empty registry with no pre-registered methodologies
    pub fn empty() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }
    
    /// Get a processor by name
    pub fn get_processor(&self, methodology: &str) -> Option<&dyn MethodologyProcessor> {
        self.processors.get(methodology).map(|p| p.as_ref())
    }
    
    /// Get all available methodology names
    pub fn available_methodologies(&self) -> Vec<String> {
        self.processors.keys().cloned().collect()
    }
    
    /// Register a custom methodology processor
    pub fn register(&mut self, name: String, processor: Box<dyn MethodologyProcessor>) {
        self.processors.insert(name, processor);
    }
}

impl Default for MethodologyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for scenario processing
pub mod utils {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use crate::core::models::ScenarioType;
    
    /// Categorize scenarios by type
    pub fn categorize_scenarios(scenarios: &[Scenario]) -> (Vec<Scenario>, Vec<Scenario>, Vec<Scenario>) {
        let mut primary = Vec::new();
        let mut alternative = Vec::new();
        let mut exceptions = Vec::new();
        
        for scenario in scenarios {
            match scenario.scenario_type {
                ScenarioType::Primary => primary.push(scenario.clone()),
                ScenarioType::Alternative | ScenarioType::Extension => alternative.push(scenario.clone()),
                ScenarioType::Exception => exceptions.push(scenario.clone()),
            }
        }
        
        (primary, alternative, exceptions)
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use crate::core::models::{Scenario, ScenarioType};
    
    #[test]
    fn test_categorize_scenarios() {
        let scenarios = vec![
            Scenario::new_with_type("S-001".to_string(), "Happy Path".to_string(), "".to_string(), ScenarioType::Primary, vec![]),
            Scenario::new_with_type("S-002".to_string(), "Alternative".to_string(), "".to_string(), ScenarioType::Alternative, vec![]),
            Scenario::new_with_type("S-003".to_string(), "Error".to_string(), "".to_string(), ScenarioType::Exception, vec![]),
        ];
        
        let (primary, alternative, errors) = categorize_scenarios(&scenarios);
        
        assert_eq!(primary.len(), 1);
        assert_eq!(alternative.len(), 1);
        assert_eq!(errors.len(), 1);
    }
}