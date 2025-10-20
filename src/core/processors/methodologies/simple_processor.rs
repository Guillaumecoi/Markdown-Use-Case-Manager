// src/core/processors/methodologies/simple_processor.rs
use super::super::methodology_processor::{
    MethodologyProcessor, ProcessedScenarios, UseCaseContext,
    utils::categorize_scenarios
};
use crate::core::models::Scenario;
use std::collections::HashMap;

/// Simple methodology processor - focuses on minimal overhead and quick documentation
#[allow(dead_code)] // Part of methodology framework
pub struct SimpleProcessor;

impl SimpleProcessor {
    #[allow(dead_code)] // Part of public API
    pub fn new() -> Self {
        Self
    }
}

impl MethodologyProcessor for SimpleProcessor {
    fn display_name(&self) -> &str {
        "Simple"
    }
    
    fn description(&self) -> &str {
        "Lightweight, flexible approach for rapid development and small teams. Minimal overhead with maximum clarity."
    }
    
    fn process_scenarios(&self, scenarios: &[Scenario], _context: &UseCaseContext) -> ProcessedScenarios {
        let (primary_flows, alternative_flows, error_flows) = categorize_scenarios(scenarios);
        
        // Simple methodology doesn't add complex processing
        let methodology_data = HashMap::new();
        
        ProcessedScenarios {
            primary_flows,
            alternative_flows,
            error_flows,
            methodology_data,
        }
    }
}

impl Default for SimpleProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::ScenarioType;
    
    #[test]
    fn test_simple_processor_basic() {
        let processor = SimpleProcessor::new();
        
        assert_eq!(processor.display_name(), "Simple");
        assert!(!processor.description().is_empty());
    }
    
    #[test]
    fn test_process_scenarios() {
        let processor = SimpleProcessor::new();
        let context = UseCaseContext {
            use_case_id: "UC-001".to_string(),
            category: "Test".to_string(),
            business_context: HashMap::new(),
        };
        
        let scenarios = vec![
            Scenario::new_with_type(
                "S-001".to_string(),
                "Happy Path".to_string(),
                "Primary use case flow".to_string(),
                ScenarioType::Primary,
                vec![],
            ),
            Scenario::new_with_type(
                "S-002".to_string(),
                "Error Case".to_string(),
                "Error handling".to_string(),
                ScenarioType::Exception,
                vec![],
            ),
        ];
        
        let processed = processor.process_scenarios(&scenarios, &context);
        
        assert_eq!(processed.primary_flows.len(), 1);
        assert_eq!(processed.alternative_flows.len(), 0);
        assert_eq!(processed.error_flows.len(), 1);
    }
}