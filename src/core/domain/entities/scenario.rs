use super::{Actor, Metadata, ScenarioReference, ScenarioStep, ScenarioType, Status, UseCase};
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

    /// Scenario-specific preconditions (in addition to use case level)
    #[serde(default)]
    pub preconditions: Vec<String>,

    /// Scenario-specific postconditions (in addition to use case level)
    #[serde(default)]
    pub postconditions: Vec<String>,

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

    /// Get all effective preconditions (use case + scenario)
    pub fn effective_preconditions(&self, use_case: &UseCase) -> Vec<String> {
        let mut all = use_case.preconditions.clone();
        all.extend(self.preconditions.clone());
        all
    }

    /// Get all effective postconditions (use case + scenario)
    pub fn effective_postconditions(&self, use_case: &UseCase) -> Vec<String> {
        let mut all = use_case.postconditions.clone();
        all.extend(self.postconditions.clone());
        all
    }

    /// Get list of actors involved in this scenario
    pub fn actors(&self) -> Vec<Actor> {
        self.steps
            .iter()
            .map(|step| step.actor.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Add a step to the scenario
    pub fn add_step(&mut self, step: ScenarioStep) {
        self.steps.push(step);
        self.steps.sort_by_key(|s| s.order);
        self.metadata.touch();
    }

    /// Add a precondition
    pub fn add_precondition(&mut self, condition: String) {
        if !self.preconditions.contains(&condition) {
            self.preconditions.push(condition);
            self.metadata.touch();
        }
    }

    /// Add a postcondition
    pub fn add_postcondition(&mut self, condition: String) {
        if !self.postconditions.contains(&condition) {
            self.postconditions.push(condition);
            self.metadata.touch();
        }
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

    /// Get all scenario IDs this scenario references
    pub fn referenced_scenarios(&self) -> Vec<&str> {
        self.references
            .iter()
            .filter(|r| matches!(r.ref_type, super::ReferenceType::UseCase))
            .map(|r| r.target_id.as_str())
            .collect()
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

        scenario.add_precondition("User has account".to_string());
        scenario.add_precondition("User has account".to_string()); // duplicate

        assert_eq!(scenario.preconditions.len(), 1);
        assert_eq!(scenario.preconditions[0], "User has account");
    }

    #[test]
    fn test_scenario_add_postcondition() {
        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        scenario.add_postcondition("User is authenticated".to_string());
        assert_eq!(scenario.postconditions.len(), 1);
        assert_eq!(scenario.postconditions[0], "User is authenticated");
    }

    #[test]
    fn test_scenario_actors() {
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
        scenario.add_step(ScenarioStep::new(
            2,
            Actor::System,
            "verifies".to_string(),
            "credentials".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            3,
            Actor::User,
            "sees".to_string(),
            "dashboard".to_string(),
        ));

        let actors = scenario.actors();
        assert_eq!(actors.len(), 2);
        assert!(actors.contains(&Actor::User));
        assert!(actors.contains(&Actor::System));
    }

    #[test]
    fn test_scenario_effective_preconditions() {
        let mut use_case = UseCase::new(
            "UC-AUTH-001".to_string(),
            "User Authentication".to_string(),
            "Auth".to_string(),
            "Handle user login".to_string(),
            "high".to_string(),
        )
        .unwrap();

        use_case.add_precondition("Application is running".to_string());

        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        scenario.add_precondition("User has valid credentials".to_string());

        let effective = scenario.effective_preconditions(&use_case);
        assert_eq!(effective.len(), 2);
        assert!(effective.contains(&"Application is running".to_string()));
        assert!(effective.contains(&"User has valid credentials".to_string()));
    }

    #[test]
    fn test_scenario_effective_postconditions() {
        let mut use_case = UseCase::new(
            "UC-AUTH-001".to_string(),
            "User Authentication".to_string(),
            "Auth".to_string(),
            "Handle user login".to_string(),
            "high".to_string(),
        )
        .unwrap();

        use_case.add_postcondition("Audit log updated".to_string());

        let mut scenario = Scenario::new(
            "UC-AUTH-001-S01".to_string(),
            "Successful login".to_string(),
            "User successfully logs in".to_string(),
            ScenarioType::HappyPath,
        );

        scenario.add_postcondition("User session created".to_string());

        let effective = scenario.effective_postconditions(&use_case);
        assert_eq!(effective.len(), 2);
        assert!(effective.contains(&"Audit log updated".to_string()));
        assert!(effective.contains(&"User session created".to_string()));
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
        scenario.add_precondition("Valid account".to_string());
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
