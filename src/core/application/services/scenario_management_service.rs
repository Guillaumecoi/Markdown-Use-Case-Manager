use crate::core::application::creators::ScenarioCreator;
use crate::core::utils::suggest_alternatives;
use crate::core::{
    domain::{Scenario, ScenarioReference, ScenarioType},
    ReferenceType, ScenarioReferenceValidator, Status, UseCase, UseCaseRepository,
};
use anyhow::Result;

/// Service for managing scenarios within use cases
///
/// This service handles CRUD operations for scenarios, scenario steps,
/// and scenario references.
pub struct ScenarioManagementService<'a> {
    repository: &'a Box<dyn UseCaseRepository>,
    use_cases: &'a mut Vec<UseCase>,
    scenario_creator: &'a ScenarioCreator,
}

impl<'a> ScenarioManagementService<'a> {
    pub fn new(
        repository: &'a Box<dyn UseCaseRepository>,
        use_cases: &'a mut Vec<UseCase>,
        scenario_creator: &'a ScenarioCreator,
    ) -> Self {
        Self {
            repository,
            use_cases,
            scenario_creator,
        }
    }

    /// Add a scenario to a use case
    pub fn add_scenario(
        &mut self,
        use_case_id: &str,
        title: String,
        scenario_type: ScenarioType,
        description: Option<String>,
        preconditions: Vec<String>,
        postconditions: Vec<String>,
        actors: Vec<String>,
    ) -> Result<String> {
        let index = self.find_use_case_index(use_case_id)?;
        let use_case = &self.use_cases[index];

        let scenario = self.scenario_creator.create_scenario(
            use_case,
            title,
            scenario_type,
            description,
            preconditions,
            postconditions,
            actors,
        );

        let mut updated_use_case = self.use_cases[index].clone();
        updated_use_case.add_scenario(scenario.clone());
        self.repository.save(&updated_use_case)?;
        self.use_cases[index] = updated_use_case;

        Ok(scenario.id)
    }

    /// Get all scenarios for a use case
    pub fn get_scenarios(&self, use_case_id: &str) -> Result<Vec<Scenario>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        Ok(use_case.scenarios.clone())
    }

    /// Update the status of a scenario
    pub fn update_scenario_status(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        new_status: Status,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        use_case.update_scenario_status(scenario_id, new_status)?;
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Add a step to an existing scenario
    pub fn add_scenario_step(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        order: u32,
        actor: String,
        action: String,
        expected_result: Option<String>,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        let step =
            self.scenario_creator
                .create_scenario_step(order, actor, action, expected_result);

        use_case.add_step_to_scenario(scenario_id, step)?;
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Remove a step from a scenario
    pub fn remove_scenario_step(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        step_order: u32,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        use_case.remove_step_from_scenario(scenario_id, step_order)?;
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Add a reference to a scenario
    pub fn add_scenario_reference(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        reference: ScenarioReference,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        let scenario_index = use_case
            .scenarios
            .iter()
            .position(|s| s.id == scenario_id)
            .ok_or_else(|| anyhow::anyhow!("Scenario with ID '{}' not found", scenario_id))?;

        // Validate no circular reference for scenario-to-scenario references
        if matches!(reference.ref_type, ReferenceType::Scenario) {
            ScenarioReferenceValidator::validate_no_circular_reference(
                &use_case,
                scenario_id,
                &reference.target_id,
            )?;
        }

        use_case.scenarios[scenario_index].add_reference(reference);
        use_case.metadata.touch();

        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Remove a reference from a scenario
    pub fn remove_scenario_reference(
        &mut self,
        use_case_id: &str,
        scenario_id: &str,
        target_id: &str,
        relationship: &str,
    ) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();

        let scenario_index = use_case
            .scenarios
            .iter()
            .position(|s| s.id == scenario_id)
            .ok_or_else(|| anyhow::anyhow!("Scenario with ID '{}' not found", scenario_id))?;

        use_case.scenarios[scenario_index].remove_reference(target_id, relationship);
        use_case.metadata.touch();
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;

        Ok(())
    }

    /// Get all scenarios referenced by a scenario
    pub fn get_scenario_references(
        &self,
        use_case_id: &str,
        scenario_id: &str,
    ) -> Result<Vec<ScenarioReference>> {
        let use_case = self.find_use_case_by_id(use_case_id)?;
        let scenario = use_case
            .scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .ok_or_else(|| {
                let available_ids: Vec<String> =
                    use_case.scenarios.iter().map(|s| s.id.clone()).collect();
                let error_msg = suggest_alternatives(scenario_id, &available_ids, "Scenario");
                anyhow::anyhow!("{}", error_msg)
            })?;

        Ok(scenario.references.clone())
    }

    // Helper methods
    fn find_use_case_index(&self, use_case_id: &str) -> Result<usize> {
        self.use_cases
            .iter()
            .position(|uc| uc.id == use_case_id)
            .ok_or_else(|| {
                let available_ids: Vec<String> =
                    self.use_cases.iter().map(|uc| uc.id.clone()).collect();
                let error_msg = suggest_alternatives(use_case_id, &available_ids, "Use case");
                anyhow::anyhow!("{}", error_msg)
            })
    }

    fn find_use_case_by_id(&self, use_case_id: &str) -> Result<&UseCase> {
        let index = self.find_use_case_index(use_case_id)?;
        Ok(&self.use_cases[index])
    }
}
