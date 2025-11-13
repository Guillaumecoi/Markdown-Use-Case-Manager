// UseCaseReference entity - represents relationships between use cases
use serde::{Deserialize, Serialize};

/// Reference to another use case with relationship type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UseCaseReference {
    /// Target use case ID (e.g., "UC-AUTH-001")
    pub target_id: String,

    /// Type of relationship
    pub relationship: String,

    /// Optional description of the relationship
    #[serde(default)]
    pub description: Option<String>,
}

impl UseCaseReference {
    /// Create a new use case reference
    pub fn new(target_id: String, relationship: String) -> Self {
        Self {
            target_id,
            relationship,
            description: None,
        }
    }

    /// Create a reference with a description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if this is a dependency relationship
    pub fn is_dependency(&self) -> bool {
        self.relationship == "depends_on"
    }

    /// Check if this is an extension relationship
    pub fn is_extension(&self) -> bool {
        self.relationship == "extends"
    }

    /// Check if this is an inclusion relationship
    pub fn is_inclusion(&self) -> bool {
        self.relationship == "includes"
    }

    /// Check if this is an alternative relationship
    pub fn is_alternative(&self) -> bool {
        self.relationship == "alternative_to"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_case_reference_creation() {
        let reference = UseCaseReference::new("UC-AUTH-001".to_string(), "depends_on".to_string());

        assert_eq!(reference.target_id, "UC-AUTH-001");
        assert_eq!(reference.relationship, "depends_on");
        assert_eq!(reference.description, None);
    }

    #[test]
    fn test_use_case_reference_with_description() {
        let reference = UseCaseReference::new("UC-AUTH-001".to_string(), "depends_on".to_string())
            .with_description("Requires authentication".to_string());

        assert_eq!(
            reference.description,
            Some("Requires authentication".to_string())
        );
    }

    #[test]
    fn test_relationship_checks() {
        let dep = UseCaseReference::new("UC-001".to_string(), "depends_on".to_string());
        assert!(dep.is_dependency());
        assert!(!dep.is_extension());
        assert!(!dep.is_inclusion());
        assert!(!dep.is_alternative());

        let ext = UseCaseReference::new("UC-002".to_string(), "extends".to_string());
        assert!(ext.is_extension());
        assert!(!ext.is_dependency());

        let inc = UseCaseReference::new("UC-003".to_string(), "includes".to_string());
        assert!(inc.is_inclusion());

        let alt = UseCaseReference::new("UC-004".to_string(), "alternative_to".to_string());
        assert!(alt.is_alternative());
    }

    #[test]
    fn test_serialization() {
        let reference = UseCaseReference::new("UC-AUTH-001".to_string(), "depends_on".to_string())
            .with_description("Requires authentication".to_string());

        let json = serde_json::to_string(&reference).unwrap();
        let deserialized: UseCaseReference = serde_json::from_str(&json).unwrap();

        assert_eq!(reference, deserialized);
    }

    #[test]
    fn test_serialization_without_description() {
        let reference = UseCaseReference::new("UC-AUTH-001".to_string(), "depends_on".to_string());

        let json = serde_json::to_string(&reference).unwrap();
        let deserialized: UseCaseReference = serde_json::from_str(&json).unwrap();

        assert_eq!(reference, deserialized);
    }
}
