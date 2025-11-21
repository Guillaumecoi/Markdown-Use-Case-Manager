use super::Actor;
use serde::{Deserialize, Serialize};

/// A single step in a scenario flow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScenarioStep {
    /// Step number (1, 2, 3, etc.)
    pub order: usize,

    /// Technical actor performing the action (sender)
    pub actor: Actor,

    /// Optional receiving actor (who/what receives the action)
    #[serde(default)]
    pub receiver: Option<Actor>,

    /// What action is performed (e.g., "enters", "verifies", "returns")
    pub action: String,

    /// Full description of what happens
    pub description: String,

    /// Additional notes or technical details
    #[serde(default)]
    pub notes: Option<String>,
}

impl ScenarioStep {
    /// Create a new scenario step with sender and optional receiver
    pub fn new(order: usize, actor: Actor, action: String, description: String) -> Self {
        Self {
            order,
            actor,
            receiver: None,
            action,
            description,
            notes: None,
        }
    }

    /// Create a new scenario step with both sender and receiver
    pub fn with_receiver(
        order: usize,
        sender: Actor,
        receiver: Actor,
        action: String,
        description: String,
    ) -> Self {
        Self {
            order,
            actor: sender,
            receiver: Some(receiver),
            action,
            description,
            notes: None,
        }
    }

    /// Get the sender actor
    pub fn sender(&self) -> &Actor {
        &self.actor
    }

    /// Get the receiver actor if present
    pub fn receiver(&self) -> Option<&Actor> {
        self.receiver.as_ref()
    }

    /// Set the receiver actor
    pub fn set_receiver(&mut self, receiver: Actor) {
        self.receiver = Some(receiver);
    }

    /// Clear the receiver actor
    pub fn clear_receiver(&mut self) {
        self.receiver = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_step_creation() {
        let step = ScenarioStep::new(
            1,
            Actor::User,
            "enters".to_string(),
            "username and password".to_string(),
        );

        assert_eq!(step.order, 1);
        assert_eq!(step.actor, Actor::User);
        assert_eq!(step.receiver, None);
        assert_eq!(step.action, "enters");
        assert_eq!(step.description, "username and password");
        assert!(step.notes.is_none());
    }

    #[test]
    fn test_scenario_step_with_receiver() {
        let step = ScenarioStep::with_receiver(
            1,
            Actor::User,
            Actor::System,
            "submits".to_string(),
            "login form".to_string(),
        );

        assert_eq!(step.order, 1);
        assert_eq!(step.actor, Actor::User);
        assert_eq!(step.receiver, Some(Actor::System));
        assert_eq!(step.action, "submits");
        assert_eq!(step.description, "login form");
    }

    #[test]
    fn test_scenario_step_receiver_methods() {
        let mut step = ScenarioStep::new(1, Actor::User, "enters".to_string(), "data".to_string());

        assert_eq!(step.receiver(), None);

        step.set_receiver(Actor::Database);
        assert_eq!(step.receiver(), Some(&Actor::Database));

        step.clear_receiver();
        assert_eq!(step.receiver(), None);
    }

    #[test]
    fn test_scenario_step_equality() {
        let step1 = ScenarioStep::new(1, Actor::User, "enters".to_string(), "data".to_string());

        let step2 = ScenarioStep::new(1, Actor::User, "enters".to_string(), "data".to_string());

        let step3 = ScenarioStep::new(2, Actor::User, "enters".to_string(), "data".to_string());

        assert_eq!(step1, step2);
        assert_ne!(step1, step3);
    }

    #[test]
    fn test_scenario_step_equality_with_receiver() {
        let step1 = ScenarioStep::with_receiver(
            1,
            Actor::User,
            Actor::System,
            "submits".to_string(),
            "form".to_string(),
        );

        let step2 = ScenarioStep::with_receiver(
            1,
            Actor::User,
            Actor::System,
            "submits".to_string(),
            "form".to_string(),
        );

        let step3 = ScenarioStep::new(1, Actor::User, "submits".to_string(), "form".to_string());

        assert_eq!(step1, step2);
        assert_ne!(step1, step3); // Different because receiver is missing
    }

    #[test]
    fn test_scenario_step_custom_actor() {
        let step = ScenarioStep::new(
            1,
            Actor::custom("PaymentGateway"),
            "processes".to_string(),
            "payment transaction".to_string(),
        );

        assert_eq!(step.actor, Actor::Custom("PaymentGateway".to_string()));
    }

    #[test]
    fn test_scenario_step_sender_getter() {
        let step = ScenarioStep::new(1, Actor::User, "action".to_string(), "desc".to_string());
        assert_eq!(step.sender(), &Actor::User);
    }
}
