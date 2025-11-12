use crate::cli::standard::CliRunner;
use crate::controller::DisplayResult;
use crate::presentation::DisplayResultFormatter;
use anyhow::Result;

/// Handles the 'create' CLI command.
///
/// Creates a new use case with the specified title, category, and optional details.
/// If a methodology is provided, uses that methodology's templates and structure
/// for generating documentation. Otherwise, uses the project's default methodology.
/// The creation result is printed to stdout.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for use case creation.
/// * `title` - The title of the use case to create.
/// * `category` - The category under which the use case should be organized.
/// * `description` - Optional detailed description of the use case.
/// * `methodology` - Optional methodology to use for documentation generation.
///
/// # Returns
/// Returns `Ok(())` on successful creation, or an error if creation fails.
pub fn handle_create_command(
    runner: &mut CliRunner,
    title: String,
    category: String,
    description: Option<String>,
    methodology: Option<String>,
) -> Result<()> {
    let result = match methodology {
        Some(methodology) => {
            match runner.create_use_case_with_methodology(title, category, description, methodology)
            {
                Ok(display_result) => display_result,
                Err(e) => DisplayResult::error(e.to_string()),
            }
        }
        None => match runner.create_use_case(title, category, description) {
            Ok(display_result) => display_result,
            Err(e) => DisplayResult::error(e.to_string()),
        },
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
