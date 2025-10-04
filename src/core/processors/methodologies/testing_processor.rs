// src/core/processors/methodologies/testing_processor.rs
use super::super::methodology_processor::{
    MethodologyProcessor, ProcessedScenarios, UseCaseContext,
    utils::categorize_scenarios
};
use crate::core::models::Scenario;
use serde_json::Value;
use std::collections::HashMap;

/// Testing methodology processor - focuses on test automation and quality assurance
pub struct TestingProcessor;

impl TestingProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl MethodologyProcessor for TestingProcessor {
    fn display_name(&self) -> &str {
        "Testing & QA"
    }
    
    fn description(&self) -> &str {
        "Test-driven approach focusing on automated testing, quality assurance, and comprehensive coverage."
    }
    
    fn process_scenarios(&self, scenarios: &[Scenario], _context: &UseCaseContext) -> ProcessedScenarios {
        let (primary_flows, alternative_flows, error_flows) = categorize_scenarios(scenarios);
        
        // Add testing-specific metadata
        let mut methodology_data = HashMap::new();
        methodology_data.insert("automation_focus".to_string(), Value::Bool(true));
        methodology_data.insert("coverage_target".to_string(), Value::String("90%".to_string()));
        methodology_data.insert("test_pyramid_layer".to_string(), Value::String("Integration".to_string()));
        
        // Calculate testing complexity based on scenario count
        let total_scenarios = scenarios.len();
        let complexity = if total_scenarios > 10 {
            "High"
        } else if total_scenarios > 5 {
            "Medium"
        } else {
            "Low"
        };
        methodology_data.insert("test_complexity".to_string(), Value::String(complexity.to_string()));
        
        ProcessedScenarios {
            primary_flows,
            alternative_flows,
            error_flows,
            methodology_data,
        }
    }
}

impl Default for TestingProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::ScenarioType;
    
    #[test]
    fn test_testing_processor_basic() {
        let processor = TestingProcessor::new();
        
        assert_eq!(processor.display_name(), "Testing & QA");
        assert!(!processor.description().is_empty());
    }
    
    #[test]
    fn test_testing_process_scenarios_with_automation_tags() {
        let processor = TestingProcessor::new();
        let context = UseCaseContext {
            use_case_id: "UC-TEST-001".to_string(),
            category: "Testing".to_string(),
            business_context: HashMap::new(),
        };
        
        let scenarios = vec![
            Scenario::new_with_type(
                "S-001".to_string(),
                "Automated Login Test".to_string(),
                "Verify login functionality".to_string(),
                ScenarioType::Primary,
                vec![],
            ),
            Scenario::new_with_type(
                "S-002".to_string(),
                "Error Handling Test".to_string(),
                "Test error scenarios".to_string(),
                ScenarioType::Exception,
                vec![],
            ),
        ];
        
        let processed = processor.process_scenarios(&scenarios, &context);
        
        assert_eq!(processed.primary_flows.len(), 1);
        assert_eq!(processed.error_flows.len(), 1);
        assert!(processed.methodology_data.contains_key("automation_focus"));
        assert!(processed.methodology_data.contains_key("test_complexity"));
    }
}