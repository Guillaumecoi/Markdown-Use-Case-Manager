use crate::cli::standard::CliRunner;
use anyhow::Result;

/// Handles the 'languages' CLI command.
///
/// Retrieves and displays the list of supported programming languages
/// that can be used when initializing or configuring use case projects.
/// The output is printed to stdout for user reference.
///
/// # Returns
/// Returns `Ok(())` on successful display, or an error if language retrieval fails.
pub fn handle_languages_command() -> Result<()> {
    let result = CliRunner::show_languages()?;
    println!("{}", result);
    Ok(())
}
