use anyhow::Result;

use super::dto::{DisplayResult, SelectionOptions};
use crate::config::Config;
use crate::core::UseCaseApplicationService;
use crate::presentation::{StatusFormatter, UseCaseFormatter};

/// Controller for use case operations
pub struct UseCaseController {
    app_service: UseCaseApplicationService,
}

impl UseCaseController {
    /// Create a new controller
    pub fn new() -> Result<Self> {
        let app_service = UseCaseApplicationService::load()?;
        Ok(Self { app_service })
    }

    /// Create a new use case with the default methodology
    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<DisplayResult> {
        let config = Config::load()?;
        let default_methodology = config.templates.default_methodology.clone();

        self.create_use_case_with_methodology(title, category, description, default_methodology)
    }

    /// Create a new use case with a specific methodology
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: String,
    ) -> Result<DisplayResult> {
        let use_case_id = self.app_service.create_use_case_with_methodology(
            title,
            category,
            description,
            &methodology,
        )?;

        // Display using formatter
        UseCaseFormatter::display_created(&use_case_id, &methodology);

        Ok(DisplayResult::success(format!(
            "Created use case: {} with {} methodology",
            use_case_id, methodology
        )))
    }

    /// List all use cases
    pub fn list_use_cases(&mut self) -> Result<()> {
        let use_cases = self.app_service.get_all_use_cases();
        UseCaseFormatter::display_list(use_cases);
        Ok(())
    }

    /// Show project status
    pub fn show_status(&mut self) -> Result<()> {
        let use_cases = self.app_service.get_all_use_cases();
        StatusFormatter::display_project_status(use_cases);
        Ok(())
    }

    /// Get all use case IDs for selection prompts
    /// TODO: Use this in interactive mode for auto-completion
    #[allow(dead_code)]
    pub fn get_use_case_ids(&mut self) -> Result<SelectionOptions> {
        let ids = self.app_service.get_all_use_case_ids()?;
        Ok(SelectionOptions::new(ids))
    }

    /// Get all categories in use
    pub fn get_categories(&mut self) -> Result<SelectionOptions> {
        let categories = self.app_service.get_all_categories()?;
        Ok(SelectionOptions::new(categories))
    }

    /// Regenerate use case with different methodology
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<DisplayResult> {
        self.app_service
            .regenerate_use_case_with_methodology(&use_case_id, &methodology)?;

        // Display using formatter
        UseCaseFormatter::display_regenerated(&use_case_id, &methodology);

        Ok(DisplayResult::success(format!(
            "Regenerated use case {} with {} methodology",
            use_case_id, methodology
        )))
    }

    /// Regenerate markdown for a single use case
    pub fn regenerate_use_case(&mut self, use_case_id: &str) -> Result<()> {
        self.app_service.regenerate_markdown(use_case_id)?;
        UseCaseFormatter::display_markdown_regenerated(use_case_id);
        Ok(())
    }

    /// Regenerate markdown for all use cases
    pub fn regenerate_all_use_cases(&mut self) -> Result<()> {
        let count = self.app_service.get_all_use_cases().len();
        self.app_service.regenerate_all_markdown()?;
        UseCaseFormatter::display_all_regenerated(count);
        Ok(())
    }
}
