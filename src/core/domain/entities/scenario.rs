use super::{Condition, Metadata, ScenarioReference, ScenarioStep, ScenarioType, Status};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario ID (e.g., "UC-AUTH-001-S01")
    pub id: String,

    pub title: String,
    pub description: String,
    pub scenario_type: ScenarioType,
    pub status: Status,

    /// Persona this scenario is designed for (placeholder)
    #[serde(default)]
    pub persona: Option<String>,

    pub metadata: Metadata,

    /// Ordered steps in the scenario flow
    #[serde(default)]
    pub steps: Vec<ScenarioStep>,

    /// Scenario-specific preconditions (in addition to use case level, can reference use cases/scenarios)
    #[serde(default)]
    pub preconditions: Vec<Condition>,

    /// Scenario-specific postconditions (in addition to use case level, can reference use cases/scenarios)
    #[serde(default)]
    pub postconditions: Vec<Condition>,

    /// References to other scenarios or use cases
    #[serde(default)]
    pub references: Vec<ScenarioReference>,

    /// Flexible extra fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Scenario {
    pub fn new(
        id: String,
        title: String,
        description: String,
        scenario_type: ScenarioType,
    ) -> Self {
        Self {
            id,
            title,
            description,
            scenario_type,
            status: Status::Planned,
            persona: None,
            metadata: Metadata::new(),
            steps: Vec::new(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            references: Vec::new(),
            extra: HashMap::new(),
        }
    }

    /// Add a step to the scenario
    pub fn add_step(&mut self, step: ScenarioStep) {
        self.steps.push(step);
        self.steps.sort_by_key(|s| s.order);
        self.metadata.touch();
    }

    /// Add a precondition
    pub fn add_precondition(&mut self, condition: Condition) {
        // Check for duplicates based on text and target
        if !self.preconditions.iter().any(|c| {
            c.text == condition.text
                && c.target_id == condition.target_id
                && c.target_type == condition.target_type
        }) {
            self.preconditions.push(condition);
            self.metadata.touch();
        }
    }

    /// Add a postcondition
    pub fn add_postcondition(&mut self, condition: Condition) {
        // Check for duplicates based on text and target
        if !self.postconditions.iter().any(|c| {
            c.text == condition.text
                && c.target_id == condition.target_id
                && c.target_type == condition.target_type
        }) {
            self.postconditions.push(condition);
            self.metadata.touch();
        }
    }

    /// Remove a precondition by text
    pub fn remove_precondition(&mut self, text: &str) {
        self.preconditions.retain(|c| c.text != text);
        self.metadata.touch();
    }

    /// Remove a postcondition by text
    pub fn remove_postcondition(&mut self, text: &str) {
        self.postconditions.retain(|c| c.text != text);
        self.metadata.touch();
    }

    /// Update scenario status
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.metadata.touch();
    }

    /// Remove a step by order
    pub fn remove_step(&mut self, step_order: u32) {
        self.steps.retain(|step| step.order != step_order as usize);
        self.metadata.touch();
    }

    /// Add a reference to another scenario or use case
    pub fn add_reference(&mut self, reference: ScenarioReference) {
        // Prevent duplicate references
        if !self.references.iter().any(|r| {
            r.ref_type == reference.ref_type
                && r.target_id == reference.target_id
                && r.relationship == reference.relationship
        }) {
            self.references.push(reference);
            self.metadata.touch();
        }
    }

    /// Check if this scenario references another scenario
    pub fn references_scenario(&self, scenario_id: &str) -> bool {
        self.references.iter().any(|r| {
            matches!(r.ref_type, super::ReferenceType::Scenario) && r.target_id == scenario_id
        })
    }

    /// Check if this scenario depends on a use case
    pub fn depends_on_use_case(&self, use_case_id: &str) -> bool {
        self.references.iter().any(|r| {
            matches!(r.ref_type, super::ReferenceType::UseCase)
                && r.target_id == use_case_id
                && r.is_dependency()
        })
    }

    /// Remove a reference
    pub fn remove_reference(&mut self, target_id: &str, relationship: &str) {
        self.references
            .retain(|r| !(r.target_id == target_id && r.relationship == relationship));
        self.metadata.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::Actor;
    use serde_json::json;

    #[test]
    fn test_scenario_creation() {
        let scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in with valid credentials".to_string(),
            ScenarioType::HappyPath,
        );

        assert_eq!(scenario.id, "UC-AUTH-001-S01");
        assert_eq!(scenario.title, "Successful login");
        assert_eq!(
            scenario.description,
            "User successfully logs in with valid credentials"
        );
        assert_eq!(scenario.scenario_type, ScenarioType::HappyPath);
        assert_eq!(scenario.status, Status::Planned);
        assert!(scenario.persona.is_none());
        assert!(scenario.steps.is_empty());
        assert!(scenario.preconditions.is_empty());
        assert!(scenario.postconditions.is_empty());
        assert!(scenario.extra.is_empty());
    }

    #[test]
    fn test_scenario_add_step() {
        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        let step1 = ScenarioStep::new(
            2,
            Actor::User,
            "enters".to_string(),
            "credentials".to_string(),
        );
        let step2 = ScenarioStep::new(
            1,
            Actor::User,
            "navigates".to_string(),
            "to login page".to_string(),
        );

        scenario.add_step(step1);
        scenario.add_step(step2);

        // Steps should be sorted by order
        assert_eq!(scenario.steps.len(), 2);
        assert_eq!(scenario.steps[0].order, 1);
        assert_eq!(scenario.steps[1].order, 2);
    }

    #[test]
    fn test_scenario_add_precondition() {
        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        scenario.add_precondition(Condition::new("User has account".to_string()));
        scenario.add_precondition(Condition::new("User has account".to_string())); // duplicate

        assert_eq!(scenario.preconditions.len(), 1);
        assert_eq!(scenario.preconditions[0].text, "User has account");
    }

    #[test]
    fn test_scenario_add_postcondition() {
        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        scenario.add_postcondition(Condition::new("User is authenticated".to_string()));
        assert_eq!(scenario.postconditions.len(), 1);
        assert_eq!(scenario.postconditions[0].text, "User is authenticated");
    }

    #[test]
    fn test_scenario_set_status() {
        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        assert_eq!(scenario.status, Status::Planned);

        scenario.set_status(Status::Implemented);
        assert_eq!(scenario.status, Status::Implemented);
    }

    #[test]
    fn test_scenario_serialization() {
        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        scenario.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "enters".to_string(),
            "credentials".to_string(),
        ));
        scenario.add_precondition(Condition::new("Valid account".to_string()));
        scenario
            .extra
            .insert("test_field".to_string(), json!("test_value"));

        let serialized = serde_json::to_string(&scenario).unwrap();
        let deserialized: Scenario = serde_json::from_str(&serialized).unwrap();

        assert_eq!(scenario.id, deserialized.id);
        assert_eq!(scenario.title, deserialized.title);
        assert_eq!(scenario.scenario_type, deserialized.scenario_type);
        assert_eq!(scenario.steps.len(), deserialized.steps.len());
        assert_eq!(scenario.preconditions, deserialized.preconditions);
        assert_eq!(scenario.extra["test_field"], json!("test_value"));
    }
}
