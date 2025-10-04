// src/core/processors/methodologies/business_processor.rs
use super::super::methodology_processor::{
    MethodologyProcessor, ProcessedScenarios, UseCaseContext,
    utils::categorize_scenarios
};
use crate::core::models::Scenario;
use serde_json::Value;
use std::collections::HashMap;

/// Business methodology processor - focuses on stakeholder value and business context
pub struct BusinessProcessor;

impl BusinessProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl MethodologyProcessor for BusinessProcessor {
    fn display_name(&self) -> &str {
        "Business Analysis"
    }
    
    fn description(&self) -> &str {
        "Comprehensive business-focused approach emphasizing stakeholder value, business requirements, and strategic alignment."
    }
    
    fn process_scenarios(&self, scenarios: &[Scenario], context: &UseCaseContext) -> ProcessedScenarios {
        let (primary_flows, alternative_flows, error_flows) = categorize_scenarios(scenarios);
        
        // Add business-specific metadata
        let mut methodology_data = HashMap::new();
        methodology_data.insert("business_priority".to_string(), Value::String("High".to_string()));
        methodology_data.insert("stakeholder_impact".to_string(), Value::String("Critical".to_string()));
        methodology_data.insert("business_value_focus".to_string(), Value::Bool(true));
        
        // Extract business context from use case context
        for (key, value) in &context.business_context {
            methodology_data.insert(format!("business_{}", key), Value::String(value.clone()));
        }
        
        ProcessedScenarios {
            primary_flows,
            alternative_flows,
            error_flows,
            methodology_data,
        }
    }
}

impl Default for BusinessProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::ScenarioType;
    
    #[test]
    fn test_business_processor_basic() {
        let processor = BusinessProcessor::new();
        
        assert_eq!(processor.display_name(), "Business Analysis");
        assert!(!processor.description().is_empty());
    }
    
    #[test]
    fn test_business_process_scenarios_with_tags() {
        let processor = BusinessProcessor::new();
        
        let mut business_context = HashMap::new();
        business_context.insert("roi_impact".to_string(), "High".to_string());
        business_context.insert("customer_segment".to_string(), "Enterprise".to_string());
        
        let context = UseCaseContext {
            use_case_id: "UC-BIZ-001".to_string(),
            category: "Business".to_string(),
            business_context,
        };
        
        let scenarios = vec![
            Scenario::new_with_type(
                "S-001".to_string(),
                "Revenue Generation".to_string(),
                "Generate revenue through upselling".to_string(),
                ScenarioType::Primary,
                vec![],
            ),
            Scenario::new_with_type(
                "S-002".to_string(),
                "Risk Mitigation".to_string(),
                "Handle compliance failure".to_string(),
                ScenarioType::Exception,
                vec![],
            ),
        ];
        
        let processed = processor.process_scenarios(&scenarios, &context);
        
        assert_eq!(processed.primary_flows.len(), 1);
        assert_eq!(processed.error_flows.len(), 1);
        assert!(processed.methodology_data.contains_key("business_priority"));
        assert!(processed.methodology_data.contains_key("business_roi_impact"));
    }
}