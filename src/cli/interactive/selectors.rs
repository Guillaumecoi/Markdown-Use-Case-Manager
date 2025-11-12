//! # Interactive Selectors
//!
//! Pure selection logic for interactive CLI operations.
//! Contains reusable selection functions that return data without UI interactions.

use anyhow::Result;

use super::runner::{InteractiveRunner, MethodologyInfo};

/// Get available programming languages
pub fn get_available_languages(runner: &mut InteractiveRunner) -> Result<Vec<String>> {
    runner.get_available_languages()
}

/// Get available methodologies with info
pub fn get_available_methodologies(runner: &mut InteractiveRunner) -> Result<Vec<MethodologyInfo>> {
    runner.get_available_methodologies()
}

/// Get methodology descriptions for display
pub fn get_methodology_descriptions(methodology_infos: &[MethodologyInfo]) -> Vec<String> {
    methodology_infos
        .iter()
        .map(|info| format!("{} - {}", info.display_name, info.description))
        .collect()
}
