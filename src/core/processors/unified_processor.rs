// src/core/processors/unified_processor.rs
use super::methodology_processor::{
    MethodologyProcessor, ProcessedScenarios, UseCaseContext,
    utils::categorize_scenarios
};
use crate::core::models::Scenario;
use serde_json::Value;
use std::collections::HashMap;

/// Configuration for a methodology processor
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MethodologyConfig {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub metadata_fields: HashMap<String, Value>,
    pub processor_fn: Option<ProcessorFunction>,
}

/// Function type for custom scenario processing
pub type ProcessorFunction = fn(&[Scenario], &UseCaseContext, &HashMap<String, Value>) -> HashMap<String, Value>;

/// Unified processor that can handle any methodology based on configuration
pub struct UnifiedProcessor {
    config: MethodologyConfig,
}

impl UnifiedProcessor {
    pub fn new(config: MethodologyConfig) -> Self {
        Self { config }
    }
    
    /// Create a simple methodology processor
    pub fn simple() -> Self {
        let config = MethodologyConfig {
            id: "simple".to_string(),
            display_name: "Simple".to_string(),
            description: "Lightweight, flexible approach for rapid development and small teams. Minimal overhead with maximum clarity.".to_string(),
            metadata_fields: HashMap::new(),
            processor_fn: None,
        };
        Self::new(config)
    }
    
    /// Create a business methodology processor
    pub fn business() -> Self {
        let mut metadata_fields = HashMap::new();
        metadata_fields.insert("business_priority".to_string(), Value::String("High".to_string()));
        metadata_fields.insert("stakeholder_impact".to_string(), Value::String("Critical".to_string()));
        metadata_fields.insert("business_value_focus".to_string(), Value::Bool(true));
        
        let config = MethodologyConfig {
            id: "business".to_string(),
            display_name: "Business Analysis".to_string(),
            description: "Comprehensive business-focused approach emphasizing stakeholder value, business requirements, and strategic alignment.".to_string(),
            metadata_fields,
            processor_fn: Some(business_processor_fn),
        };
        Self::new(config)
    }
    
    /// Create a testing methodology processor
    pub fn testing() -> Self {
        let mut metadata_fields = HashMap::new();
        metadata_fields.insert("automation_focus".to_string(), Value::Bool(true));
        metadata_fields.insert("coverage_target".to_string(), Value::String("90%".to_string()));
        metadata_fields.insert("test_pyramid_layer".to_string(), Value::String("Integration".to_string()));
        
        let config = MethodologyConfig {
            id: "testing".to_string(),
            display_name: "Testing & QA".to_string(),
            description: "Test-driven approach focusing on automated testing, quality assurance, and comprehensive coverage.".to_string(),
            metadata_fields,
            processor_fn: Some(testing_processor_fn),
        };
        Self::new(config)
    }
    
    /// Create a custom methodology processor from configuration
    #[allow(dead_code)]
    pub fn custom(config: MethodologyConfig) -> Self {
        Self::new(config)
    }
    
    #[allow(dead_code)]
    pub fn config(&self) -> &MethodologyConfig {
        &self.config
    }
}

impl MethodologyProcessor for UnifiedProcessor {
    fn display_name(&self) -> &str {
        &self.config.display_name
    }
    
    fn description(&self) -> &str {
        &self.config.description
    }
    
    fn process_scenarios(&self, scenarios: &[Scenario], context: &UseCaseContext) -> ProcessedScenarios {
        let (primary_flows, alternative_flows, error_flows) = categorize_scenarios(scenarios);
        
        // Start with base metadata fields
        let mut methodology_data = self.config.metadata_fields.clone();
        
        // Apply custom processor function if available
        if let Some(processor_fn) = self.config.processor_fn {
            let additional_data = processor_fn(scenarios, context, &methodology_data);
            methodology_data.extend(additional_data);
        }
        
        ProcessedScenarios {
            primary_flows,
            alternative_flows,
            error_flows,
            methodology_data,
        }
    }
}

/// Business-specific processing function
fn business_processor_fn(
    _scenarios: &[Scenario], 
    context: &UseCaseContext, 
    _base_metadata: &HashMap<String, Value>
) -> HashMap<String, Value> {
    let mut additional_data = HashMap::new();
    
    // Extract business context from use case context
    for (key, value) in &context.business_context {
        additional_data.insert(format!("business_{}", key), Value::String(value.clone()));
    }
    
    additional_data
}

/// Testing-specific processing function
fn testing_processor_fn(
    scenarios: &[Scenario], 
    _context: &UseCaseContext, 
    _base_metadata: &HashMap<String, Value>
) -> HashMap<String, Value> {
    let mut additional_data = HashMap::new();
    
    // Calculate testing complexity based on scenario count
    let total_scenarios = scenarios.len();
    let complexity = if total_scenarios > 10 {
        "High"
    } else if total_scenarios > 5 {
        "Medium"
    } else {
        "Low"
    };
    additional_data.insert("test_complexity".to_string(), Value::String(complexity.to_string()));
    
    additional_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::ScenarioType;
    
    #[test]
    fn test_unified_processor_simple() {
        let processor = UnifiedProcessor::simple();
        
        assert_eq!(processor.display_name(), "Simple");
        assert!(!processor.description().is_empty());
        assert_eq!(processor.config().id, "simple");
    }
    
    #[test]
    fn test_unified_processor_business() {
        let processor = UnifiedProcessor::business();
        
        assert_eq!(processor.display_name(), "Business Analysis");
        assert!(!processor.description().is_empty());
        assert_eq!(processor.config().id, "business");
        assert!(processor.config().metadata_fields.contains_key("business_priority"));
    }
    
    #[test]
    fn test_unified_processor_testing() {
        let processor = UnifiedProcessor::testing();
        
        assert_eq!(processor.display_name(), "Testing & QA");
        assert!(!processor.description().is_empty());
        assert_eq!(processor.config().id, "testing");
        assert!(processor.config().metadata_fields.contains_key("automation_focus"));
    }
    
    #[test]
    fn test_custom_processor() {
        let mut metadata = HashMap::new();
        metadata.insert("custom_field".to_string(), Value::String("custom_value".to_string()));
        
        let config = MethodologyConfig {
            id: "custom".to_string(),
            display_name: "Custom Methodology".to_string(),
            description: "A custom methodology for testing".to_string(),
            metadata_fields: metadata,
            processor_fn: None,
        };
        
        let processor = UnifiedProcessor::custom(config);
        
        assert_eq!(processor.display_name(), "Custom Methodology");
        assert_eq!(processor.config().id, "custom");
    }
    
    #[test]
    fn test_business_processor_with_context() {
        let processor = UnifiedProcessor::business();
        
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
        ];
        
        let processed = processor.process_scenarios(&scenarios, &context);
        
        assert_eq!(processed.primary_flows.len(), 1);
        assert!(processed.methodology_data.contains_key("business_priority"));
        assert!(processed.methodology_data.contains_key("business_roi_impact"));
    }
    
    #[test]
    fn test_testing_processor_complexity() {
        let processor = UnifiedProcessor::testing();
        let context = UseCaseContext {
            use_case_id: "UC-TEST-001".to_string(),
            category: "Testing".to_string(),
            business_context: HashMap::new(),
        };
        
        // Test with many scenarios for high complexity
        let scenarios = (0..12).map(|i| {
            Scenario::new_with_type(
                format!("S-{:03}", i),
                format!("Test Scenario {}", i),
                "Test description".to_string(),
                ScenarioType::Primary,
                vec![],
            )
        }).collect::<Vec<_>>();
        
        let processed = processor.process_scenarios(&scenarios, &context);
        
        assert!(processed.methodology_data.contains_key("test_complexity"));
        if let Some(Value::String(complexity)) = processed.methodology_data.get("test_complexity") {
            assert_eq!(complexity, "High");
        }
    }
}