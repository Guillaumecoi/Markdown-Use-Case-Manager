use crate::config::Config;
use crate::core::domain::{Actor, Scenario, ScenarioStep, ScenarioType, UseCase};

/// Handles scenario creation and management
pub struct ScenarioCreator {
    config: Config,
}

impl ScenarioCreator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Create a new scenario for a use case
    pub fn create_scenario(
        &self,
        use_case: &UseCase,
        title: String,
        scenario_type: ScenarioType,
        description: Option<String>,
        preconditions: Vec<String>,
        postconditions: Vec<String>,
        _actors: Vec<String>,
    ) -> Scenario {
        let scenario_id = use_case.next_scenario_id();

        let mut scenario = Scenario::new(
            scenario_id,
            title,
            description.unwrap_or_default(),
            scenario_type,
        );

        // Add preconditions and postconditions
        for precondition in preconditions {
            scenario.add_precondition(precondition);
        }
        for postcondition in postconditions {
            scenario.add_postcondition(postcondition);
        }

        scenario
    }

    /// Create a scenario step
    pub fn create_scenario_step(
        &self,
        order: u32,
        actor: String,
        action: String,
        expected_result: Option<String>,
    ) -> ScenarioStep {
        let actor_enum: Actor = actor.into(); // Convert String to Actor using From<String>
        let description = expected_result.unwrap_or_else(|| format!("{} {}", actor_enum.name(), action));
        ScenarioStep::new(order as usize, actor_enum, action, description)
    }
}
