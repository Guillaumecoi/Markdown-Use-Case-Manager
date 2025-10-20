// src/core/processors/methodologies/developer_processor.rs
use super::super::methodology_processor::{MethodologyProcessor, ProcessedScenarios, UseCaseContext};
use crate::core::models::Scenario;
use serde_json::json;
use std::collections::HashMap;

/// Developer-focused methodology processor
/// Optimized for technical teams and implementation planning
pub struct DeveloperProcessor {
    display_name: String,
    description: String,
}

impl DeveloperProcessor {
    pub fn new() -> Self {
        Self {
            display_name: "Technical Development".to_string(),
            description: "Technical implementation documentation for development teams. Focuses on architecture, technical requirements, and implementation details.".to_string(),
        }
    }
}

impl MethodologyProcessor for DeveloperProcessor {
    fn display_name(&self) -> &str {
        &self.display_name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn process_scenarios(&self, scenarios: &[Scenario], _context: &UseCaseContext) -> ProcessedScenarios {
        use super::super::methodology_processor::utils::categorize_scenarios;
        let (primary, alternative, error) = categorize_scenarios(scenarios);
        
        // Developer-specific metadata
        let mut methodology_data = HashMap::new();
        methodology_data.insert("methodology_type".to_string(), json!("developer"));
        methodology_data.insert("focus_area".to_string(), json!("Technical Architecture & Implementation"));
        methodology_data.insert("target_audience".to_string(), json!("Developers, Architects, Technical Leads"));
        methodology_data.insert("key_sections".to_string(), json!(vec![
            "Technical Requirements",
            "Architecture Design", 
            "Implementation Plan",
            "Testing Strategy",
            "Performance Considerations"
        ]));
        
        // Add developer-specific processing hints
        methodology_data.insert("primary_concern".to_string(), json!("How will this be implemented technically?"));
        methodology_data.insert("success_measure".to_string(), json!("Code quality, performance, and maintainability metrics"));
        methodology_data.insert("technical_complexity".to_string(), json!("Medium")); // Default complexity
        
        ProcessedScenarios {
            primary_flows: primary,
            alternative_flows: alternative,
            error_flows: error,
            methodology_data,
        }
    }
}

impl Default for DeveloperProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Scenario, ScenarioType};
    
    #[test]
    fn test_developer_processor_basic() {
        let processor = DeveloperProcessor::new();
        assert_eq!(processor.display_name(), "Technical Development");
        assert!(processor.description().contains("Technical implementation"));
    }
    
    #[test]
    fn test_developer_process_scenarios() {
        let processor = DeveloperProcessor::new();
        let scenarios = vec![
            Scenario::new_with_type("S-001".to_string(), "API Endpoint".to_string(), "Primary implementation".to_string(), ScenarioType::Primary, vec![]),
            Scenario::new_with_type("S-002".to_string(), "Database Error".to_string(), "Error handling".to_string(), ScenarioType::Exception, vec![]),
        ];
        
        let context = UseCaseContext {
            use_case_id: "UC-001".to_string(),
            category: "Technical".to_string(),
            business_context: HashMap::new(),
        };
        
        let result = processor.process_scenarios(&scenarios, &context);
        
        assert_eq!(result.primary_flows.len(), 1);
        assert_eq!(result.alternative_flows.len(), 0);
        assert_eq!(result.error_flows.len(), 1);
        
        // Check developer-specific metadata
        assert_eq!(result.methodology_data.get("methodology_type").unwrap(), &json!("developer"));
        assert_eq!(result.methodology_data.get("focus_area").unwrap(), &json!("Technical Architecture & Implementation"));
        assert_eq!(result.methodology_data.get("technical_complexity").unwrap(), &json!("Medium"));
    }
}