//! Output manager for generating filenames for use case documentation.
//!
//! Handles filename generation logic for both single-view and multi-view use cases:
//! - Single view: `UC-001.md` (no suffix)
//! - Multiple views: `UC-001-feat-s.md`, `UC-001-bus-n.md` (with methodology-level suffix)

use crate::core::{MethodologyView, UseCase};

/// Manages output filenames for use case documentation.
pub struct OutputManager;

impl OutputManager {
    /// Generates the markdown filename for a use case.
    ///
    /// **Single View** (views is empty): Returns `{use_case_id}.md`
    /// - Example: `UC-001.md`
    ///
    /// **Multiple Views**: Returns `{use_case_id}-{methodology_abbr}-{level_abbr}.md`
    /// - Example: `UC-001-feat-s.md`, `UC-001-bus-n.md`
    ///
    /// # Arguments
    /// * `use_case` - The use case entity
    /// * `view` - Optional view for multi-view scenarios. If None, generates single-view filename.
    pub fn generate_filename(use_case: &UseCase, view: Option<&MethodologyView>) -> String {
        // Single view: no suffix
        if !use_case.is_multi_view() {
            return format!("{}.md", use_case.id);
        }

        // Multi-view: add suffix
        if let Some(view) = view {
            format!("{}-{}.md", use_case.id, view.key())
        } else {
            // Fallback for when view is None but use case has views
            // This shouldn't happen in normal usage
            format!("{}.md", use_case.id)
        }
    }

    /// Generates all filenames for a use case based on its enabled views.
    ///
    /// Returns a vector of (filename, view) tuples for each enabled view.
    /// If the use case has no views (single-view mode), returns a single filename without a view.
    pub fn generate_all_filenames(use_case: &UseCase) -> Vec<(String, Option<MethodologyView>)> {
        if !use_case.is_multi_view() {
            // Single view: just the main filename
            vec![(format!("{}.md", use_case.id), None)]
        } else {
            // Multiple views: one filename per enabled view
            use_case
                .enabled_views()
                .map(|view| {
                    let filename = format!("{}-{}.md", use_case.id, view.key());
                    (filename, Some(view.clone()))
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{MethodologyView, UseCase};

    #[test]
    fn test_single_view_filename() {
        let use_case = UseCase::new(
            "UC-001".to_string(),
            "Test Use Case".to_string(),
            "testing".to_string(),
            "Description".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        let filename = OutputManager::generate_filename(&use_case, None);
        assert_eq!(filename, "UC-001.md");
    }

    #[test]
    fn test_multi_view_filename_with_view() {
        let mut use_case = UseCase::new(
            "UC-001".to_string(),
            "Test Use Case".to_string(),
            "testing".to_string(),
            "Description".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        let view = MethodologyView::new("feature".to_string(), "simple".to_string());
        use_case.add_view(view.clone());

        let filename = OutputManager::generate_filename(&use_case, Some(&view));
        assert_eq!(filename, "UC-001-feature-simple.md");
    }

    #[test]
    fn test_multi_view_filename_without_view_specified() {
        let mut use_case = UseCase::new(
            "UC-001".to_string(),
            "Test Use Case".to_string(),
            "testing".to_string(),
            "Description".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        let view = MethodologyView::new("feature".to_string(), "simple".to_string());
        use_case.add_view(view);

        // When view is None but use case has views, fallback to no suffix
        let filename = OutputManager::generate_filename(&use_case, None);
        assert_eq!(filename, "UC-001.md");
    }

    #[test]
    fn test_generate_all_filenames_single_view() {
        let use_case = UseCase::new(
            "UC-001".to_string(),
            "Test Use Case".to_string(),
            "testing".to_string(),
            "Description".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        let filenames = OutputManager::generate_all_filenames(&use_case);

        assert_eq!(filenames.len(), 1);
        assert_eq!(filenames[0].0, "UC-001.md");
        assert!(filenames[0].1.is_none());
    }

    #[test]
    fn test_generate_all_filenames_multi_view() {
        let mut use_case = UseCase::new(
            "UC-001".to_string(),
            "Test Use Case".to_string(),
            "testing".to_string(),
            "Description".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        use_case.add_view(MethodologyView::new(
            "feature".to_string(),
            "simple".to_string(),
        ));
        use_case.add_view(MethodologyView::new(
            "business".to_string(),
            "normal".to_string(),
        ));

        let filenames = OutputManager::generate_all_filenames(&use_case);

        assert_eq!(filenames.len(), 2);

        // Check that both filenames are present (order may vary)
        let filename_strings: Vec<String> = filenames.iter().map(|(f, _)| f.clone()).collect();
        assert!(filename_strings.contains(&"UC-001-feature-simple.md".to_string()));
        assert!(filename_strings.contains(&"UC-001-business-normal.md".to_string()));

        // All should have associated views
        assert!(filenames.iter().all(|(_, view)| view.is_some()));
    }

    #[test]
    fn test_generate_all_filenames_with_disabled_view() {
        let mut use_case = UseCase::new(
            "UC-001".to_string(),
            "Test Use Case".to_string(),
            "testing".to_string(),
            "Description".to_string(),
            "medium".to_string(),
        )
        .unwrap();

        use_case.add_view(MethodologyView::new(
            "feature".to_string(),
            "simple".to_string(),
        ));

        let mut disabled_view = MethodologyView::new("business".to_string(), "normal".to_string());
        disabled_view.enabled = false;
        use_case.add_view(disabled_view);

        let filenames = OutputManager::generate_all_filenames(&use_case);

        // Should only include the enabled view
        assert_eq!(filenames.len(), 1);
        assert_eq!(filenames[0].0, "UC-001-feature-simple.md");
    }
}
