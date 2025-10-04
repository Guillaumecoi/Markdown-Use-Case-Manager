// src/core/models/scenario_types.rs
use serde::{Deserialize, Serialize};

/// Semantic categorization of scenarios
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ScenarioType {
    /// Main success path - the primary way to achieve the use case goal
    #[default]
    Primary,
    /// Alternative ways to achieve the same goal
    Alternative, 
    /// Error conditions and failure scenarios
    Exception,
    /// Additional functionality or edge cases
    Extension,
}

/// Additional metadata from text parsing
impl std::fmt::Display for ScenarioType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScenarioType::Primary => write!(f, "primary"),
            ScenarioType::Alternative => write!(f, "alternative"),
            ScenarioType::Exception => write!(f, "exception"),
            ScenarioType::Extension => write!(f, "extension"),
        }
    }
}

impl std::str::FromStr for ScenarioType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" | "main" | "happy" => Ok(ScenarioType::Primary),
            "alternative" | "alt" => Ok(ScenarioType::Alternative),
            "exception" | "error" | "failure" => Ok(ScenarioType::Exception),
            "extension" | "ext" | "edge" => Ok(ScenarioType::Extension),
            _ => Err(format!("Unknown scenario type: {}", s)),
        }
    }
}

/// Individual step within a scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStep {
    pub sequence: u32,
    pub actor: String,           // "User", "System", "External Service"
    pub action: String,          // "clicks login button"
    pub expected_result: String, // "login form is displayed"
    pub step_type: StepType,     // Type of step for processing
}

/// Type of action in a scenario step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    /// User performs an action
    UserAction,
    /// System responds or processes
    SystemAction, 
    /// Check or verify something
    Validation,
    /// Branching/decision point
    Decision,
    /// External system interaction
    ExternalAction,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scenario_type_parsing() {
        assert_eq!("primary".parse::<ScenarioType>().unwrap(), ScenarioType::Primary);
        assert_eq!("happy".parse::<ScenarioType>().unwrap(), ScenarioType::Primary);
        assert_eq!("alternative".parse::<ScenarioType>().unwrap(), ScenarioType::Alternative);
        assert_eq!("error".parse::<ScenarioType>().unwrap(), ScenarioType::Exception);
        assert_eq!("extension".parse::<ScenarioType>().unwrap(), ScenarioType::Extension);
    }
}