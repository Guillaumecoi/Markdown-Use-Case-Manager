use super::ReferenceType;
use serde::{Deserialize, Serialize};

/// Reference from one scenario to another scenario or use case
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScenarioReference {
    /// Type of reference (UseCase or Scenario)
    pub ref_type: ReferenceType,

    /// Target ID (e.g., "UC-AUTH-001" or "UC-AUTH-001-S01")
    pub target_id: String,

    /// Relationship type
    pub relationship: String, // "includes", "extends", "precedes", "depends_on", "alternative_to"

    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
}

impl ScenarioReference {
    pub fn new(ref_type: ReferenceType, target_id: String, relationship: String) -> Self {
        Self {
            ref_type,
            target_id,
            relationship,
            description: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if this is an "includes" relationship
    pub fn is_inclusion(&self) -> bool {
        self.relationship == "includes"
    }

    /// Check if this is an "extends" relationship
    pub fn is_extension(&self) -> bool {
        self.relationship == "extends"
    }

    /// Check if this is a "depends_on" relationship
    pub fn is_dependency(&self) -> bool {
        self.relationship == "depends_on"
    }

    /// Check if this is a "precedes" relationship
    pub fn is_precedence(&self) -> bool {
        self.relationship == "precedes"
    }

    /// Check if this is an "alternative_to" relationship
    pub fn is_alternative(&self) -> bool {
        self.relationship == "alternative_to"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_reference_creation() {
        let reference = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-AUTH-001-S01".to_string(),
            "extends".to_string(),
        );

        assert_eq!(reference.ref_type, ReferenceType::Scenario);
        assert_eq!(reference.target_id, "UC-AUTH-001-S01");
        assert_eq!(reference.relationship, "extends");
        assert!(reference.is_extension());
    }

    #[test]
    fn test_relationship_checks() {
        let includes = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-001-S01".to_string(),
            "includes".to_string(),
        );
        assert!(includes.is_inclusion());

        let extends = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-001-S02".to_string(),
            "extends".to_string(),
        );
        assert!(extends.is_extension());

        let depends = ScenarioReference::new(
            ReferenceType::UseCase,
            "UC-AUTH-001".to_string(),
            "depends_on".to_string(),
        );
        assert!(depends.is_dependency());

        let precedes = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-001-S03".to_string(),
            "precedes".to_string(),
        );
        assert!(precedes.is_precedence());

        let alternative = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-001-S04".to_string(),
            "alternative_to".to_string(),
        );
        assert!(alternative.is_alternative());
    }

    #[test]
    fn test_serialization() {
        let reference = ScenarioReference::new(
            ReferenceType::Scenario,
            "UC-AUTH-001-S01".to_string(),
            "extends".to_string(),
        )
        .with_description("Handles authentication failure".to_string());

        let json = serde_json::to_string(&reference).unwrap();
        let deserialized: ScenarioReference = serde_json::from_str(&json).unwrap();

        assert_eq!(reference, deserialized);
    }

    #[test]
    fn test_with_description() {
        let reference = ScenarioReference::new(
            ReferenceType::UseCase,
            "UC-AUTH-001".to_string(),
            "depends_on".to_string(),
        )
        .with_description("Requires authentication to be completed first".to_string());

        assert_eq!(
            reference.description,
            Some("Requires authentication to be completed first".to_string())
        );
    }
}
