use crate::cli::standard::CliRunner;
use crate::controller::DisplayResult;
use crate::presentation::DisplayResultFormatter;
use anyhow::Result;

/// Handles the 'init' CLI command.
///
/// Initializes a new use case manager project in the current directory.
/// When `finalize` is false, sets up the project structure, configuration files,
/// and default settings based on the provided language and methodologies.
/// When `finalize` is true, completes the initialization process.
/// Progress messages are printed to stdout.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for project initialization.
/// * `language` - Optional programming language to configure for the project.
/// * `methodologies` - List of methodologies to enable for the project.
/// * `storage` - Storage backend to use (toml or sqlite).
/// * `finalize` - Whether to finalize the initialization (true) or perform initial setup (false).
///
/// # Returns
/// Returns `Ok(())` on successful initialization, or an error if setup fails.
pub fn handle_init_command(
    runner: &mut CliRunner,
    language: Option<String>,
    methodologies: Vec<String>,
    storage: String,
    finalize: bool,
) -> Result<()> {
    if finalize {
        println!("Finalizing initialization...");
        match runner.finalize_init() {
            Ok(result) => DisplayResultFormatter::display(&result),
            Err(e) => DisplayResultFormatter::display(&DisplayResult::error(e.to_string())),
        }
    } else {
        println!("Initializing use case manager project...");
        match runner.init_project_with_storage(language, methodologies, storage) {
            Ok(result) => DisplayResultFormatter::display(&result),
            Err(e) => DisplayResultFormatter::display(&DisplayResult::error(e.to_string())),
        }
    }
    Ok(())
}

/// Handles the 'status' CLI command.
///
/// Displays the current status of the use case manager project,
/// including information about initialized state, configured settings,
/// and available use cases. The status output is printed to stdout.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for retrieving project status.
///
/// # Returns
/// Returns `Ok(())` on successful status display, or an error if status retrieval fails.
pub fn handle_status_command(runner: &mut CliRunner) -> Result<()> {
    runner.show_status()
}
