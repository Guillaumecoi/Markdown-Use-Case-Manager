/// Cleanup command handlers for removing orphaned methodology fields.

use anyhow::Result;

use crate::cli::standard::runner::CliRunner;
use crate::presentation::DisplayResultFormatter;

/// Handle the cleanup command to remove orphaned methodology fields.
///
/// Scans use case TOML files and removes methodology sections that are no longer
/// used by any view. Supports both single use case cleanup and full project cleanup.
///
/// # Arguments
/// * `runner` - CLI runner instance
/// * `use_case_id` - Optional specific use case to clean (cleans all if None)
/// * `dry_run` - If true, shows what would be cleaned without making changes
pub fn handle_cleanup_command(
    runner: &mut CliRunner,
    use_case_id: Option<String>,
    dry_run: bool,
) -> Result<()> {
    let result = runner.cleanup_methodology_fields(use_case_id, dry_run)?;
    DisplayResultFormatter::display(&result);
    Ok(())
}
