use super::ReferenceType;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a precondition or postcondition, optionally referencing a use case or scenario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Condition {
    /// The condition text (e.g., "User must be authenticated")
    pub text: String,

    /// Optional reference to a use case or scenario that must be satisfied
    #[serde(default)]
    pub target_type: Option<ReferenceType>,

    /// Optional target ID (e.g., "UC-AUTH-001" or "UC-AUTH-001-S01")
    #[serde(default)]
    pub target_id: Option<String>,

    /// Optional relationship type (e.g., "depends_on", "requires", "must_complete")
    #[serde(default)]
    pub relationship: Option<String>,
}

impl Condition {
    /// Create a simple condition with just text (no reference)
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            target_type: None,
            target_id: None,
            relationship: None,
        }
    }

    /// Create a condition with a use case reference
    pub fn with_use_case(
        text: impl Into<String>,
        target_id: impl Into<String>,
        relationship: impl Into<Option<String>>,
    ) -> Self {
        Self {
            text: text.into(),
            target_type: Some(ReferenceType::UseCase),
            target_id: Some(target_id.into()),
            relationship: relationship.into(),
        }
    }

    /// Create a condition with a scenario reference
    pub fn with_scenario(
        text: impl Into<String>,
        target_id: impl Into<String>,
        relationship: impl Into<Option<String>>,
    ) -> Self {
        Self {
            text: text.into(),
            target_type: Some(ReferenceType::Scenario),
            target_id: Some(target_id.into()),
            relationship: relationship.into(),
        }
    }

    /// Check if this condition has a reference
    pub fn has_reference(&self) -> bool {
        self.target_type.is_some() && self.target_id.is_some()
    }

    /// Check if this condition references a use case
    pub fn references_use_case(&self) -> bool {
        matches!(self.target_type, Some(ReferenceType::UseCase))
    }

    /// Check if this condition references a scenario
    pub fn references_scenario(&self) -> bool {
        matches!(self.target_type, Some(ReferenceType::Scenario))
    }

    /// Get the full reference string (e.g., "UC-AUTH-001 must be complete")
    pub fn reference_display(&self) -> Option<String> {
        if let (Some(target_id), Some(rel)) = (&self.target_id, &self.relationship) {
            Some(format!("{} {}", target_id, rel))
        } else {
            self.target_id.clone()
        }
    }

    /// Check if this is a dependency relationship
    pub fn is_dependency(&self) -> bool {
        self.relationship
            .as_ref()
            .map(|r| r == "depends_on" || r == "requires")
            .unwrap_or(false)
    }

    /// Check if this condition references a specific target
    pub fn references_target(&self, target_id: &str) -> bool {
        self.target_id.as_deref() == Some(target_id)
    }
}

// Display implementation for UI
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref_display) = self.reference_display() {
            write!(f, "{} ({})", self.text, ref_display)
        } else {
            write!(f, "{}", self.text)
        }
    }
}

// Helper for backward compatibility during migration
impl From<String> for Condition {
    fn from(text: String) -> Self {
        Condition::new(text)
    }
}

impl From<&str> for Condition {
    fn from(text: &str) -> Self {
        Condition::new(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_condition() {
        let condition = Condition::new("User must be logged in".to_string());

        assert_eq!(condition.text, "User must be logged in");
        assert!(!condition.has_reference());
        assert!(!condition.references_use_case());
        assert!(!condition.references_scenario());
        assert_eq!(condition.reference_display(), None);
    }

    #[test]
    fn test_condition_with_use_case_reference() {
        let condition = Condition::with_use_case(
            "Authentication must be complete".to_string(),
            "UC-AUTH-001".to_string(),
            Some("must_complete".to_string()),
        );

        assert_eq!(condition.text, "Authentication must be complete");
        assert!(condition.has_reference());
        assert!(condition.references_use_case());
        assert!(!condition.references_scenario());
        assert_eq!(
            condition.reference_display(),
            Some("UC-AUTH-001 must_complete".to_string())
        );
    }

    #[test]
    fn test_condition_with_scenario_reference() {
        let condition = Condition::with_scenario(
            "Login scenario must succeed".to_string(),
            "UC-AUTH-001-S01".to_string(),
            Some("depends_on".to_string()),
        );

        assert_eq!(condition.text, "Login scenario must succeed");
        assert!(condition.has_reference());
        assert!(condition.references_scenario());
        assert!(!condition.references_use_case());
        assert!(condition.is_dependency());
    }

    #[test]
    fn test_references_target() {
        let condition = Condition::with_use_case(
            "Test".to_string(),
            "UC-001".to_string(),
            Some("depends_on".to_string()),
        );

        assert!(condition.references_target("UC-001"));
        assert!(!condition.references_target("UC-002"));
    }

    #[test]
    fn test_is_dependency() {
        let dep1 = Condition::with_use_case(
            "Test".to_string(),
            "UC-001".to_string(),
            Some("depends_on".to_string()),
        );
        assert!(dep1.is_dependency());

        let dep2 = Condition::with_use_case(
            "Test".to_string(),
            "UC-001".to_string(),
            Some("requires".to_string()),
        );
        assert!(dep2.is_dependency());

        let not_dep = Condition::with_use_case(
            "Test".to_string(),
            "UC-001".to_string(),
            Some("extends".to_string()),
        );
        assert!(!not_dep.is_dependency());

        let simple = Condition::new("Test".to_string());
        assert!(!simple.is_dependency());
    }

    #[test]
    fn test_from_string() {
        let condition: Condition = "User is authenticated".to_string().into();
        assert_eq!(condition.text, "User is authenticated");
        assert!(!condition.has_reference());

        let condition2: Condition = "System is online".into();
        assert_eq!(condition2.text, "System is online");
    }

    #[test]
    fn test_serialization() {
        let condition = Condition::with_use_case(
            "Auth complete".to_string(),
            "UC-AUTH-001".to_string(),
            Some("must_complete".to_string()),
        );

        let json = serde_json::to_string(&condition).unwrap();
        let deserialized: Condition = serde_json::from_str(&json).unwrap();

        assert_eq!(condition, deserialized);
    }

    #[test]
    fn test_serialization_simple_condition() {
        let condition = Condition::new("Simple condition".to_string());

        let json = serde_json::to_string(&condition).unwrap();
        let deserialized: Condition = serde_json::from_str(&json).unwrap();

        assert_eq!(condition, deserialized);
        assert_eq!(deserialized.target_type, None);
        assert_eq!(deserialized.target_id, None);
        assert_eq!(deserialized.relationship, None);
    }
}
