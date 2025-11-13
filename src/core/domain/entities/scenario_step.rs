use serde::{Deserialize, Serialize};

/// A single step in a scenario flow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScenarioStep {
    /// Step number (1, 2, 3, etc.)
    pub order: usize,

    /// Who performs the action (e.g., "User", "System", "Server")
    pub actor: String,

    /// What action is performed (e.g., "enters", "verifies", "returns")
    pub action: String,

    /// Full description of what happens
    pub description: String,

    /// Additional notes or technical details
    #[serde(default)]
    pub notes: Option<String>,
}

impl ScenarioStep {
    pub fn new(order: usize, actor: String, action: String, description: String) -> Self {
        Self {
            order,
            actor,
            action,
            description,
            notes: None,
        }
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_step_creation() {
        let step = ScenarioStep::new(
            1,
            "User".to_string(),
            "enters".to_string(),
            "username and password".to_string(),
        );

        assert_eq!(step.order, 1);
        assert_eq!(step.actor, "User");
        assert_eq!(step.action, "enters");
        assert_eq!(step.description, "username and password");
        assert!(step.notes.is_none());
    }

    #[test]
    fn test_scenario_step_with_notes() {
        let step = ScenarioStep::new(
            2,
            "System".to_string(),
            "verifies".to_string(),
            "credentials".to_string(),
        )
        .with_notes("Check against database".to_string());

        assert_eq!(step.order, 2);
        assert_eq!(step.actor, "System");
        assert_eq!(step.action, "verifies");
        assert_eq!(step.description, "credentials");
        assert_eq!(step.notes, Some("Check against database".to_string()));
    }

    #[test]
    fn test_scenario_step_serialization() {
        let step = ScenarioStep::new(
            1,
            "User".to_string(),
            "clicks".to_string(),
            "submit button".to_string(),
        )
        .with_notes("AJAX request".to_string());

        let serialized = serde_json::to_string(&step).unwrap();
        let deserialized: ScenarioStep = serde_json::from_str(&serialized).unwrap();

        assert_eq!(step, deserialized);
    }

    #[test]
    fn test_scenario_step_equality() {
        let step1 = ScenarioStep::new(
            1,
            "User".to_string(),
            "enters".to_string(),
            "data".to_string(),
        );

        let step2 = ScenarioStep::new(
            1,
            "User".to_string(),
            "enters".to_string(),
            "data".to_string(),
        );

        let step3 = ScenarioStep::new(
            2,
            "User".to_string(),
            "enters".to_string(),
            "data".to_string(),
        );

        assert_eq!(step1, step2);
        assert_ne!(step1, step3);
    }
}
