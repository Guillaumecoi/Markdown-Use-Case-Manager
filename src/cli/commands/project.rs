use crate::cli::runner::CliRunner;
use anyhow::Result;

/// Handles the 'init' CLI command.
///
/// Initializes a new use case manager project in the current directory.
/// When `finalize` is false, sets up the project structure, configuration files,
/// and default settings based on the provided language and methodology.
/// When `finalize` is true, completes the initialization process.
/// Progress messages are printed to stdout.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for project initialization.
/// * `language` - Optional programming language to configure for the project.
/// * `methodology` - Optional methodology to set as default for the project.
/// * `finalize` - Whether to finalize the initialization (true) or perform initial setup (false).
///
/// # Returns
/// Returns `Ok(())` on successful initialization, or an error if setup fails.
pub fn handle_init_command(
    runner: &mut CliRunner,
    language: Option<String>,
    methodology: Option<String>,
    finalize: bool,
) -> Result<()> {
    if finalize {
        println!("Finalizing initialization...");
        let result = runner.finalize_init()?;
        println!("{}", result);
    } else {
        println!("Initializing use case manager project...");
        let result = runner.init_project(language, methodology)?;
        println!("{}", result);
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
