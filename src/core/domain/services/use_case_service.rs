// Domain service for use case business logic
use crate::core::domain::entities::{Priority, UseCase};
use crate::core::infrastructure::template_engine::to_snake_case;
use std::path::Path;

/// Core business logic for use case management
/// This service focuses purely on domain operations without I/O concerns
#[derive(Clone)]
pub struct UseCaseService;

impl UseCaseService {
    pub fn new() -> Self {
        Self
    }

    /// Generate a use case ID based on category and existing use cases
    /// Generate a unique use case ID that checks both in-memory use cases and filesystem
    pub fn generate_unique_use_case_id(
        &self,
        category: &str,
        use_cases: &[UseCase],
        use_case_dir: &str,
    ) -> String {
        let category_prefix = category.to_uppercase().chars().take(3).collect::<String>();
        let category_dir = Path::new(use_case_dir).join(to_snake_case(category));

        // Find the highest existing number by checking both in-memory and filesystem
        let mut max_number = 0;

        // Check in-memory use cases
        for uc in use_cases.iter() {
            if uc.category.to_lowercase() == category.to_lowercase() {
                if let Some(num) = self.extract_number_from_id(&uc.id, &category_prefix) {
                    max_number = max_number.max(num);
                }
            }
        }

        // Check filesystem for existing files
        if category_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&category_dir) {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".md")
                            && file_name.starts_with(&format!("UC-{}-", category_prefix))
                        {
                            let id_part = file_name.trim_end_matches(".md");
                            if let Some(num) =
                                self.extract_number_from_id(id_part, &category_prefix)
                            {
                                max_number = max_number.max(num);
                            }
                        }
                    }
                }
            }
        }

        format!("UC-{}-{:03}", category_prefix, max_number + 1)
    }

    /// Extract number from ID like "UC-CON-001" -> Some(1)
    fn extract_number_from_id(&self, id: &str, category_prefix: &str) -> Option<usize> {
        let expected_prefix = format!("UC-{}-", category_prefix);
        if id.starts_with(&expected_prefix) {
            let number_part = &id[expected_prefix.len()..];
            number_part.parse::<usize>().ok()
        } else {
            None
        }
    }

    /// Create a new use case with the given parameters
    pub fn create_use_case(
        &self,
        id: String,
        title: String,
        category: String,
        description: String,
    ) -> UseCase {
        UseCase::new(id, title, category, description, Priority::Medium)
    }
}

impl Default for UseCaseService {
    fn default() -> Self {
        Self::new()
    }
}
