use crate::cli::standard::CliRunner;
use crate::controller::DisplayResult;
use crate::presentation::DisplayResultFormatter;
use anyhow::Result;

/// Handles the 'create' CLI command.
///
/// Creates a new use case with the specified title, category, and optional details.
/// Supports both single-view (legacy) and multi-view creation:
/// - If `views` is provided, creates a multi-view use case
/// - If `methodology` is provided, creates a single-view use case (legacy)
/// - Otherwise, uses the project's default methodology
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for use case creation.
/// * `title` - The title of the use case to create.
/// * `category` - The category under which the use case should be organized.
/// * `description` - Optional detailed description of the use case.
/// * `methodology` - Optional methodology to use for documentation generation (legacy).
/// * `views` - Optional comma-separated list of methodology:level pairs (e.g., "feature:simple,business:normal").
///
/// # Returns
/// Returns `Ok(())` on successful creation, or an error if creation fails.
pub fn handle_create_command(
    runner: &mut CliRunner,
    title: String,
    category: String,
    description: Option<String>,
    methodology: Option<String>,
    views: Option<String>,
) -> Result<()> {
    let result = if let Some(views_str) = views {
        // Multi-view creation
        match runner.create_use_case_with_views(title, category, description, views_str) {
            Ok(display_result) => display_result,
            Err(e) => DisplayResult::error(e.to_string()),
        }
    } else if let Some(methodology) = methodology {
        // Single-view with specific methodology (legacy)
        match runner.create_use_case_with_methodology(title, category, description, methodology) {
            Ok(display_result) => display_result,
            Err(e) => DisplayResult::error(e.to_string()),
        }
    } else {
        // Single-view with default methodology
        match runner.create_use_case(title, category, description) {
            Ok(display_result) => display_result,
            Err(e) => DisplayResult::error(e.to_string()),
        }
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'list' CLI command.
///
/// Retrieves and displays a list of all existing use cases in the project,
/// including their titles, categories, and current status.
/// The formatted list is printed to stdout for user reference.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for listing use cases.
///
/// # Returns
/// Returns `Ok(())` on successful display, or an error if retrieval fails.
pub fn handle_list_command(runner: &mut CliRunner) -> Result<()> {
    runner.list_use_cases()
}
