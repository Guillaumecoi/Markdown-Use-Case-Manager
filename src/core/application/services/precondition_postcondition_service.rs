use crate::core::{UseCase, UseCaseRepository};
use anyhow::Result;

/// Service for managing preconditions and postconditions on use cases
pub struct PreconditionPostconditionService<'a> {
    repository: &'a Box<dyn UseCaseRepository>,
    use_cases: &'a mut Vec<UseCase>,
}

impl<'a> PreconditionPostconditionService<'a> {
    pub fn new(
        repository: &'a Box<dyn UseCaseRepository>,
        use_cases: &'a mut Vec<UseCase>,
    ) -> Self {
        Self {
            repository,
            use_cases,
        }
    }

    /// Add a precondition to a use case
    pub fn add_precondition(&mut self, use_case_id: &str, precondition: String) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();
        use_case.add_precondition(precondition);
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;
        Ok(())
    }

    /// Remove a precondition from a use case
    pub fn remove_precondition(&mut self, use_case_id: &str, index: usize) -> Result<()> {
        let index_in_vec = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index_in_vec].clone();

        // Convert 1-based index to 0-based
        let zero_based_index = index.saturating_sub(1);
        if zero_based_index >= use_case.preconditions.len() {
            return Err(anyhow::anyhow!(
                "Precondition index {} is out of bounds",
                index
            ));
        }

        use_case.preconditions.remove(zero_based_index);
        self.repository.save(&use_case)?;
        self.use_cases[index_in_vec] = use_case;
        Ok(())
    }

    /// Add a postcondition to a use case
    pub fn add_postcondition(&mut self, use_case_id: &str, postcondition: String) -> Result<()> {
        let index = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index].clone();
        use_case.add_postcondition(postcondition);
        self.repository.save(&use_case)?;
        self.use_cases[index] = use_case;
        Ok(())
    }

    /// Remove a postcondition from a use case
    pub fn remove_postcondition(&mut self, use_case_id: &str, index: usize) -> Result<()> {
        let index_in_vec = self.find_use_case_index(use_case_id)?;
        let mut use_case = self.use_cases[index_in_vec].clone();

        // Convert 1-based index to 0-based
        let zero_based_index = index.saturating_sub(1);
        if zero_based_index >= use_case.postconditions.len() {
            return Err(anyhow::anyhow!(
                "Postcondition index {} is out of bounds",
                index
            ));
        }

        use_case.postconditions.remove(zero_based_index);
        self.repository.save(&use_case)?;
        self.use_cases[index_in_vec] = use_case;
        Ok(())
    }

    // Helper methods
    fn find_use_case_index(&self, use_case_id: &str) -> Result<usize> {
        self.use_cases
            .iter()
            .position(|uc| uc.id == use_case_id)
            .ok_or_else(|| anyhow::anyhow!("Use case '{}' not found", use_case_id))
    }
}
