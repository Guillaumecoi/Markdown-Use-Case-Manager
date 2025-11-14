use crate::core::domain::entities::{Actor, Scenario, UseCase};
use std::collections::{HashMap, HashSet};

/// Domain service for actor-related business logic
pub struct ActorService;

impl ActorService {
    /// Extract all unique actors from a use case
    pub fn extract_actors_from_use_case(use_case: &UseCase) -> Vec<Actor> {
        let mut actors = HashSet::new();

        for scenario in &use_case.scenarios {
            for actor in scenario.actors() {
                actors.insert(actor);
            }
        }

        let mut result: Vec<Actor> = actors.into_iter().collect();
        result.sort_by(|a, b| a.name().cmp(b.name()));
        result
    }

    /// Extract all unique actors from a scenario
    pub fn extract_actors_from_scenario(scenario: &Scenario) -> Vec<Actor> {
        scenario.actors()
    }

    /// Get usage statistics for actors across multiple scenarios
    pub fn get_actor_statistics(scenarios: &[Scenario]) -> HashMap<Actor, ActorStats> {
        let mut stats: HashMap<Actor, ActorStats> = HashMap::new();

        for scenario in scenarios {
            for step in &scenario.steps {
                let actor_stats = stats
                    .entry(step.actor.clone())
                    .or_insert_with(|| ActorStats {
                        actor: step.actor.clone(),
                        total_steps: 0,
                        scenarios: HashSet::new(),
                    });

                actor_stats.total_steps += 1;
                actor_stats.scenarios.insert(scenario.id.clone());
            }
        }

        stats
    }

    /// Suggest primary actor for a use case based on step frequency
    pub fn suggest_primary_actor(use_case: &UseCase) -> Option<Actor> {
        let stats = Self::get_actor_statistics(&use_case.scenarios);

        stats
            .values()
            .filter(|s| s.actor.is_human())
            .max_by_key(|s| s.total_steps)
            .map(|s| s.actor.clone())
    }

    /// Check if an actor participates in a scenario
    pub fn actor_participates_in_scenario(actor: &Actor, scenario: &Scenario) -> bool {
        scenario.steps.iter().any(|step| &step.actor == actor)
    }

    /// Get all scenarios where an actor participates
    pub fn get_scenarios_for_actor<'a>(actor: &Actor, use_case: &'a UseCase) -> Vec<&'a Scenario> {
        use_case
            .scenarios
            .iter()
            .filter(|scenario| Self::actor_participates_in_scenario(actor, scenario))
            .collect()
    }
}

/// Statistics about an actor's participation
#[derive(Debug, Clone)]
pub struct ActorStats {
    pub actor: Actor,
    pub total_steps: usize,
    pub scenarios: HashSet<String>,
}

impl ActorStats {
    /// Get the number of scenarios this actor participates in
    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::{ScenarioStep, ScenarioType};

    fn create_test_use_case() -> UseCase {
        UseCase::new(
            "UC-TEST-001".to_string(),
            "Test".to_string(),
            "A test use case".to_string(),
            "Test description".to_string(),
            "medium".to_string(),
        )
        .unwrap()
    }

    fn create_test_scenario(id: &str, title: &str) -> Scenario {
        Scenario::new(
            id.to_string(),
            title.to_string(),
            "Test scenario".to_string(),
            ScenarioType::HappyPath,
        )
    }

    #[test]
    fn test_extract_actors_from_scenario() {
        let mut scenario = create_test_scenario("UC-TEST-001-S01", "Test");
        scenario.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            2,
            Actor::System,
            "responds".to_string(),
            "success".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            3,
            Actor::User,
            "sees".to_string(),
            "result".to_string(),
        ));

        let actors = ActorService::extract_actors_from_scenario(&scenario);
        assert_eq!(actors.len(), 2);
        assert!(actors.contains(&Actor::User));
        assert!(actors.contains(&Actor::System));
    }

    #[test]
    fn test_extract_actors_from_use_case() {
        let mut use_case = create_test_use_case();

        let mut scenario1 = create_test_scenario("UC-TEST-001-S01", "Scenario 1");
        scenario1.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario1.add_step(ScenarioStep::new(
            2,
            Actor::System,
            "responds".to_string(),
            "success".to_string(),
        ));

        let mut scenario2 = create_test_scenario("UC-TEST-001-S02", "Scenario 2");
        scenario2.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "navigates".to_string(),
            "page".to_string(),
        ));
        scenario2.add_step(ScenarioStep::new(
            2,
            Actor::Database,
            "queries".to_string(),
            "data".to_string(),
        ));

        use_case.add_scenario(scenario1);
        use_case.add_scenario(scenario2);

        let actors = ActorService::extract_actors_from_use_case(&use_case);
        assert_eq!(actors.len(), 3);
        assert!(actors.contains(&Actor::User));
        assert!(actors.contains(&Actor::System));
        assert!(actors.contains(&Actor::Database));
    }

    #[test]
    fn test_get_actor_statistics() {
        let mut scenario1 = create_test_scenario("UC-TEST-001-S01", "Scenario 1");
        scenario1.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario1.add_step(ScenarioStep::new(
            2,
            Actor::User,
            "types".to_string(),
            "text".to_string(),
        ));
        scenario1.add_step(ScenarioStep::new(
            3,
            Actor::System,
            "responds".to_string(),
            "success".to_string(),
        ));

        let mut scenario2 = create_test_scenario("UC-TEST-001-S02", "Scenario 2");
        scenario2.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "navigates".to_string(),
            "page".to_string(),
        ));
        scenario2.add_step(ScenarioStep::new(
            2,
            Actor::Database,
            "queries".to_string(),
            "data".to_string(),
        ));

        let scenarios = vec![scenario1, scenario2];
        let stats = ActorService::get_actor_statistics(&scenarios);

        assert_eq!(stats.len(), 3);

        let user_stats = stats.get(&Actor::User).unwrap();
        assert_eq!(user_stats.total_steps, 3);
        assert_eq!(user_stats.scenario_count(), 2);

        let system_stats = stats.get(&Actor::System).unwrap();
        assert_eq!(system_stats.total_steps, 1);
        assert_eq!(system_stats.scenario_count(), 1);
    }

    #[test]
    fn test_suggest_primary_actor() {
        let mut use_case = create_test_use_case();

        let mut scenario1 = create_test_scenario("UC-TEST-001-S01", "Scenario 1");
        scenario1.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario1.add_step(ScenarioStep::new(
            2,
            Actor::User,
            "types".to_string(),
            "text".to_string(),
        ));
        scenario1.add_step(ScenarioStep::new(
            3,
            Actor::System,
            "responds".to_string(),
            "success".to_string(),
        ));

        let mut scenario2 = create_test_scenario("UC-TEST-001-S02", "Scenario 2");
        scenario2.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "navigates".to_string(),
            "page".to_string(),
        ));
        scenario2.add_step(ScenarioStep::new(
            2,
            Actor::System,
            "loads".to_string(),
            "data".to_string(),
        ));

        use_case.add_scenario(scenario1);
        use_case.add_scenario(scenario2);

        let primary = ActorService::suggest_primary_actor(&use_case);
        assert_eq!(primary, Some(Actor::User));
    }

    #[test]
    fn test_actor_participates_in_scenario() {
        let mut scenario = create_test_scenario("UC-TEST-001-S01", "Test");
        scenario.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "clicks".to_string(),
            "button".to_string(),
        ));
        scenario.add_step(ScenarioStep::new(
            2,
            Actor::System,
            "responds".to_string(),
            "success".to_string(),
        ));

        assert!(ActorService::actor_participates_in_scenario(
            &Actor::User,
            &scenario
        ));
        assert!(ActorService::actor_participates_in_scenario(
            &Actor::System,
            &scenario
        ));
        assert!(!ActorService::actor_participates_in_scenario(
            &Actor::Database,
            &scenario
        ));
    }

    #[test]
    fn test_get_scenarios_for_actor() {
        let mut use_case = create_test_use_case();

        let mut scenario1 = create_test_scenario("UC-TEST-001-S01", "Scenario 1");
        scenario1.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "clicks".to_string(),
            "button".to_string(),
        ));

        let mut scenario2 = create_test_scenario("UC-TEST-001-S02", "Scenario 2");
        scenario2.add_step(ScenarioStep::new(
            1,
            Actor::System,
            "processes".to_string(),
            "request".to_string(),
        ));

        let mut scenario3 = create_test_scenario("UC-TEST-001-S03", "Scenario 3");
        scenario3.add_step(ScenarioStep::new(
            1,
            Actor::User,
            "navigates".to_string(),
            "page".to_string(),
        ));

        use_case.add_scenario(scenario1);
        use_case.add_scenario(scenario2);
        use_case.add_scenario(scenario3);

        let user_scenarios = ActorService::get_scenarios_for_actor(&Actor::User, &use_case);
        assert_eq!(user_scenarios.len(), 2);
        assert!(user_scenarios.iter().any(|s| s.id == "UC-TEST-001-S01"));
        assert!(user_scenarios.iter().any(|s| s.id == "UC-TEST-001-S03"));

        let system_scenarios = ActorService::get_scenarios_for_actor(&Actor::System, &use_case);
        assert_eq!(system_scenarios.len(), 1);
        assert!(system_scenarios.iter().any(|s| s.id == "UC-TEST-001-S02"));
    }
}
