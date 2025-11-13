//! # Use Case Controller
//!
//! This module provides the controller for use case management operations.
//! It handles the coordination between CLI commands and the use case application
//! services, providing a clean interface for creating, listing, and regenerating
//! use case documentation.
//!
//! ## Responsibilities
//!
//! - Use case creation with methodology selection
//! - Use case listing and status display
//! - Markdown regeneration for individual or all use cases
//! - Methodology switching and regeneration
//! - Data retrieval for interactive selection prompts
//!
//! ## Use Case Lifecycle
//!
//! 1. **Creation**: Use cases are created with a title, category, and optional methodology
//! 2. **Storage**: Raw data is stored as TOML files, generated markdown in separate directory
//! 3. **Regeneration**: Markdown can be regenerated when templates or data change
//! 4. **Methodology Changes**: Use cases can be regenerated with different methodologies

use anyhow::Result;

use super::dto::{DisplayResult, SelectionOptions};
use crate::config::Config;
use crate::core::UseCaseApplicationService;
use crate::presentation::{StatusFormatter, UseCaseFormatter};

/// Controller for use case operations and management.
///
/// Manages all use case-related operations including creation, listing,
/// regeneration, and status reporting. Acts as the coordination layer
/// between CLI commands and the use case application services.
pub struct UseCaseController {
    /// Application service for use case business logic
    app_service: UseCaseApplicationService,
}

impl UseCaseController {
    /// Create a new use case controller instance.
    ///
    /// Initializes the controller with a loaded use case application service.
    /// The application service handles all the core business logic for use cases.
    ///
    /// # Returns
    /// A new UseCaseController instance ready for use
    ///
    /// # Errors
    /// Returns error if the application service cannot be loaded
    pub fn new() -> Result<Self> {
        let app_service = UseCaseApplicationService::load()?;
        Ok(Self { app_service })
    }

    /// Create a new use case using the project's default methodology.
    ///
    /// Creates a use case using the project's default methodology, allowing
    /// users to quickly create use cases without specifying a methodology.
    ///
    /// # Arguments
    /// * `title` - The title of the use case
    /// * `category` - The category under which to organize the use case
    /// * `description` - Optional detailed description of the use case
    ///
    /// # Returns
    /// DisplayResult with success message and use case information
    ///
    /// # Errors
    /// Returns error if use case creation fails or configuration cannot be loaded
    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<DisplayResult> {
        match (|| -> Result<DisplayResult> {
            let config = Config::load()?;
            let default_methodology = config.templates.default_methodology.clone();
            self.create_use_case_with_methodology(title, category, description, default_methodology)
        })() {
            Ok(result) => Ok(result),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Create a new use case with a specific methodology.
    ///
    /// Creates a use case using the specified methodology, allowing users to
    /// override the default methodology for individual use cases.
    ///
    /// # Arguments
    /// * `title` - The title of the use case
    /// * `category` - The category under which to organize the use case
    /// * `description` - Optional detailed description of the use case
    /// * `methodology` - The methodology to use for this use case
    ///
    /// # Returns
    /// DisplayResult with success message and use case information
    ///
    /// # Errors
    /// Returns error if use case creation fails or methodology is invalid
    pub fn create_use_case_with_methodology(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
        methodology: String,
    ) -> Result<DisplayResult> {
        match self.app_service.create_use_case_with_methodology(
            title,
            category,
            description,
            &methodology,
        ) {
            Ok(use_case_id) => {
                // Display using formatter
                UseCaseFormatter::display_created(&use_case_id, &methodology);

                Ok(DisplayResult::success(format!(
                    "Created use case: {} with {} methodology",
                    use_case_id, methodology
                )))
            }
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// List all use cases in the project.
    ///
    /// Retrieves and displays a formatted list of all existing use cases
    /// in the project, including their titles, categories, and current status.
    ///
    /// # Returns
    /// Ok(()) on successful display
    ///
    /// # Errors
    /// Returns error if use case retrieval fails
    pub fn list_use_cases(&mut self) -> Result<()> {
        let use_cases = self.app_service.get_all_use_cases();
        UseCaseFormatter::display_list(use_cases);
        Ok(())
    }

    /// Show project status and statistics.
    ///
    /// Displays comprehensive project status including use case counts,
    /// categories, methodologies used, and other project metrics.
    ///
    /// # Returns
    /// Ok(()) on successful display
    ///
    /// # Errors
    /// Returns error if status retrieval fails
    pub fn show_status(&mut self) -> Result<()> {
        let use_cases = self.app_service.get_all_use_cases();
        StatusFormatter::display_project_status(use_cases);
        Ok(())
    }

    /// Get all categories currently in use.
    ///
    /// Retrieves a list of all categories that have use cases, useful for
    /// filtering and organization displays.
    ///
    /// # Returns
    /// SelectionOptions containing all category names
    ///
    /// # Errors
    /// Returns error if category retrieval fails
    pub fn get_categories(&mut self) -> Result<SelectionOptions> {
        let categories = self.app_service.get_all_categories()?;
        Ok(SelectionOptions::new(categories))
    }

    /// Regenerate use case with different methodology.
    ///
    /// Changes the methodology of an existing use case and regenerates its
    /// documentation using the new methodology's templates.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to regenerate
    /// * `methodology` - The new methodology to apply
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if regeneration fails or use case doesn't exist
    pub fn regenerate_use_case_with_methodology(
        &mut self,
        use_case_id: String,
        methodology: String,
    ) -> Result<DisplayResult> {
        match self
            .app_service
            .regenerate_use_case_with_methodology(&use_case_id, &methodology)
        {
            Ok(_) => {
                // Display using formatter
                UseCaseFormatter::display_regenerated(&use_case_id, &methodology);

                Ok(DisplayResult::success(format!(
                    "Regenerated use case {} with {} methodology",
                    use_case_id, methodology
                )))
            }
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Regenerate markdown for a single use case.
    ///
    /// Regenerates the markdown documentation for a specific use case using
    /// its current methodology and data.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to regenerate
    ///
    /// # Returns
    /// Ok(()) on successful regeneration
    ///
    /// # Errors
    /// Returns error if regeneration fails or use case doesn't exist
    pub fn regenerate_use_case(&mut self, use_case_id: &str) -> Result<()> {
        self.app_service.regenerate_markdown(use_case_id)?;
        UseCaseFormatter::display_markdown_regenerated(use_case_id);
        Ok(())
    }

    /// Regenerate markdown for all use cases.
    ///
    /// Regenerates the markdown documentation for all use cases in the project.
    /// Useful after template changes or bulk updates.
    ///
    /// # Returns
    /// Ok(()) on successful regeneration
    ///
    /// # Errors
    /// Returns error if any regeneration fails
    pub fn regenerate_all_use_cases(&mut self) -> Result<()> {
        let count = self.app_service.get_all_use_cases().len();
        self.app_service.regenerate_all_markdown()?;
        UseCaseFormatter::display_all_regenerated(count);
        Ok(())
    }

    /// Add a precondition to a use case.
    ///
    /// Adds a new precondition to the specified use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    /// * `precondition` - The precondition text to add
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if use case not found or precondition cannot be added
    pub fn add_precondition(
        &mut self,
        use_case_id: String,
        precondition: String,
    ) -> Result<DisplayResult> {
        match self
            .app_service
            .add_precondition(&use_case_id, precondition)
        {
            Ok(_) => Ok(DisplayResult::success(format!(
                "Added precondition to use case: {}",
                use_case_id
            ))),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// List preconditions for a use case.
    ///
    /// Retrieves and displays all preconditions for the specified use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    ///
    /// # Returns
    /// DisplayResult with preconditions list
    ///
    /// # Errors
    /// Returns error if use case not found
    pub fn list_preconditions(&mut self, use_case_id: String) -> Result<DisplayResult> {
        match self.app_service.get_preconditions(&use_case_id) {
            Ok(preconditions) => {
                let mut result = format!("Preconditions for {}:\n", use_case_id);
                if preconditions.is_empty() {
                    result.push_str("  No preconditions found.");
                } else {
                    for (i, precondition) in preconditions.iter().enumerate() {
                        result.push_str(&format!("  {}. {}\n", i + 1, precondition));
                    }
                }
                Ok(DisplayResult::success(result))
            }
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Remove a precondition from a use case.
    ///
    /// Removes the precondition at the specified index from the use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    /// * `index` - The 1-based index of the precondition to remove
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if use case not found or index is invalid
    pub fn remove_precondition(
        &mut self,
        use_case_id: String,
        index: usize,
    ) -> Result<DisplayResult> {
        match self.app_service.remove_precondition(&use_case_id, index) {
            Ok(_) => Ok(DisplayResult::success(format!(
                "Removed precondition {} from use case: {}",
                index, use_case_id
            ))),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Add a postcondition to a use case.
    ///
    /// Adds a new postcondition to the specified use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    /// * `postcondition` - The postcondition text to add
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if use case not found or postcondition cannot be added
    pub fn add_postcondition(
        &mut self,
        use_case_id: String,
        postcondition: String,
    ) -> Result<DisplayResult> {
        match self
            .app_service
            .add_postcondition(&use_case_id, postcondition)
        {
            Ok(_) => Ok(DisplayResult::success(format!(
                "Added postcondition to use case: {}",
                use_case_id
            ))),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// List postconditions for a use case.
    ///
    /// Retrieves and displays all postconditions for the specified use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    ///
    /// # Returns
    /// DisplayResult with postconditions list
    ///
    /// # Errors
    /// Returns error if use case not found
    pub fn list_postconditions(&mut self, use_case_id: String) -> Result<DisplayResult> {
        match self.app_service.get_postconditions(&use_case_id) {
            Ok(postconditions) => {
                let mut result = format!("Postconditions for {}:\n", use_case_id);
                if postconditions.is_empty() {
                    result.push_str("  No postconditions found.");
                } else {
                    for (i, postcondition) in postconditions.iter().enumerate() {
                        result.push_str(&format!("  {}. {}\n", i + 1, postcondition));
                    }
                }
                Ok(DisplayResult::success(result))
            }
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Remove a postcondition from a use case.
    ///
    /// Removes the postcondition at the specified index from the use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    /// * `index` - The 1-based index of the postcondition to remove
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if use case not found or index is invalid
    pub fn remove_postcondition(
        &mut self,
        use_case_id: String,
        index: usize,
    ) -> Result<DisplayResult> {
        match self.app_service.remove_postcondition(&use_case_id, index) {
            Ok(_) => Ok(DisplayResult::success(format!(
                "Removed postcondition {} from use case: {}",
                index, use_case_id
            ))),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Add a reference to a use case.
    ///
    /// Adds a new reference relationship to the specified use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    /// * `target_id` - The ID of the target use case
    /// * `relationship` - The type of relationship (dependency, extension, inclusion, alternative)
    /// * `description` - Optional description of the relationship
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if use case not found or reference cannot be added
    pub fn add_reference(
        &mut self,
        use_case_id: String,
        target_id: String,
        relationship: String,
        description: Option<String>,
    ) -> Result<DisplayResult> {
        match self
            .app_service
            .add_reference(&use_case_id, target_id, relationship, description)
        {
            Ok(_) => Ok(DisplayResult::success(format!(
                "Added reference to use case: {}",
                use_case_id
            ))),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// List references for a use case.
    ///
    /// Retrieves and displays all references for the specified use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    ///
    /// # Returns
    /// DisplayResult with references list
    ///
    /// # Errors
    /// Returns error if use case not found
    pub fn list_references(&mut self, use_case_id: String) -> Result<DisplayResult> {
        match self.app_service.get_references(&use_case_id) {
            Ok(references) => {
                let mut result = format!("References for {}:\n", use_case_id);
                if references.is_empty() {
                    result.push_str("  No references found.");
                } else {
                    for reference in &references {
                        result.push_str(&format!(
                            "  - {} ({})",
                            reference.target_id, reference.relationship
                        ));
                        if let Some(desc) = &reference.description {
                            result.push_str(&format!(": {}", desc));
                        }
                        result.push('\n');
                    }
                }
                Ok(DisplayResult::success(result))
            }
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }

    /// Remove a reference from a use case.
    ///
    /// Removes the reference to the specified target use case.
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case
    /// * `target_id` - The ID of the target use case to remove reference to
    ///
    /// # Returns
    /// DisplayResult with success message
    ///
    /// # Errors
    /// Returns error if use case not found or reference doesn't exist
    pub fn remove_reference(
        &mut self,
        use_case_id: String,
        target_id: String,
    ) -> Result<DisplayResult> {
        match self.app_service.remove_reference(&use_case_id, &target_id) {
            Ok(_) => Ok(DisplayResult::success(format!(
                "Removed reference to {} from use case: {}",
                target_id, use_case_id
            ))),
            Err(e) => Ok(DisplayResult::error(e.to_string())),
        }
    }
}
