// src/core/processors/methodologies/feature_processor.rs
use super::super::methodology_processor::{MethodologyProcessor, ProcessedScenarios, UseCaseContext};
use crate::core::models::Scenario;
use serde_json::json;
use std::collections::HashMap;

/// Feature-focused methodology processor
/// Optimized for product managers and feature teams
pub struct FeatureProcessor {
    display_name: String,
    description: String,
}

impl FeatureProcessor {
    pub fn new() -> Self {
        Self {
            display_name: "Feature Development".to_string(),
            description: "Feature development methodology for product managers and stakeholders. Emphasizes user value, business impact, and implementation planning.".to_string(),
        }
    }
}

impl MethodologyProcessor for FeatureProcessor {
    fn display_name(&self) -> &str {
        &self.display_name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn process_scenarios(&self, scenarios: &[Scenario], _context: &UseCaseContext) -> ProcessedScenarios {
        use super::super::methodology_processor::utils::categorize_scenarios;
        let (primary, alternative, error) = categorize_scenarios(scenarios);
        
        // Feature-specific metadata
        let mut methodology_data = HashMap::new();
        methodology_data.insert("methodology_type".to_string(), json!("feature"));
        methodology_data.insert("focus_area".to_string(), json!("User Value & Business Impact"));
        methodology_data.insert("target_audience".to_string(), json!("Product Managers, Stakeholders, Design Teams"));
        methodology_data.insert("key_sections".to_string(), json!(vec![
            "User Value Proposition",
            "Target User Groups", 
            "Success Metrics",
            "Implementation Strategy"
        ]));
        
        // Add feature-specific processing hints
        methodology_data.insert("primary_concern".to_string(), json!("What problem does this solve for users?"));
        methodology_data.insert("success_measure".to_string(), json!("User adoption and business value metrics"));
        
        ProcessedScenarios {
            primary_flows: primary,
            alternative_flows: alternative,
            error_flows: error,
            methodology_data,
        }
    }
}

impl Default for FeatureProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Scenario, ScenarioType};
    
    #[test]
    fn test_feature_processor_basic() {
        let processor = FeatureProcessor::new();
        assert_eq!(processor.display_name(), "Feature Development");
        assert!(processor.description().contains("Feature development"));
    }
    
    #[test]
    fn test_feature_process_scenarios() {
        let processor = FeatureProcessor::new();
        let scenarios = vec![
            Scenario::new_with_type("S-001".to_string(), "User Login".to_string(), "Happy path".to_string(), ScenarioType::Primary, vec![]),
            Scenario::new_with_type("S-002".to_string(), "Password Reset".to_string(), "Alternative flow".to_string(), ScenarioType::Alternative, vec![]),
        ];
        
        let context = UseCaseContext {
            use_case_id: "UC-001".to_string(),
            category: "Feature".to_string(),
            business_context: HashMap::new(),
        };
        
        let result = processor.process_scenarios(&scenarios, &context);
        
        assert_eq!(result.primary_flows.len(), 1);
        assert_eq!(result.alternative_flows.len(), 1);
        assert_eq!(result.error_flows.len(), 0);
        
        // Check feature-specific metadata
        assert_eq!(result.methodology_data.get("methodology_type").unwrap(), &json!("feature"));
        assert_eq!(result.methodology_data.get("focus_area").unwrap(), &json!("User Value & Business Impact"));
    }
}