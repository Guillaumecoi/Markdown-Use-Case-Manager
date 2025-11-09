use crate::cli::runner::CliRunner;
use anyhow::Result;

/// Handle the list methodologies command
pub fn handle_list_methodologies_command(runner: &mut CliRunner) -> Result<()> {
    let result = runner.list_methodologies()?;
    println!("{}", result);
    Ok(())
}

/// Handle the methodology info command
pub fn handle_methodology_info_command(runner: &mut CliRunner, name: String) -> Result<()> {
    let result = runner.get_methodology_info(name)?;
    println!("{}", result);
    Ok(())
}

/// Handle the regenerate command
///
/// Supports three modes:
/// 1. No args or --all: Regenerate all use cases with current methodology
/// 2. With use_case_id: Regenerate single use case with current methodology
/// 3. With use_case_id + --methodology: Regenerate with different methodology
pub fn handle_regenerate_command(
    runner: &mut CliRunner,
    use_case_id: Option<String>,
    methodology: Option<String>,
    all: bool,
) -> Result<()> {
    match (use_case_id, methodology, all) {
        // No args or --all flag: regenerate all use cases
        (None, None, _) | (None, Some(_), true) => {
            runner.regenerate_all_use_cases()?;
            println!("✅ Regenerated all use case documentation");
            Ok(())
        }
        // Use case ID + methodology: regenerate with different methodology
        (Some(id), Some(method), _) => {
            let result = runner.regenerate_use_case_with_methodology(id, method)?;
            println!("{}", result);
            Ok(())
        }
        // Use case ID only: regenerate with current methodology
        (Some(id), None, _) => {
            runner.regenerate_use_case(&id)?;
            println!("✅ Regenerated documentation for {}", id);
            Ok(())
        }
        // --all with methodology but no ID: error (doesn't make sense)
        (None, Some(_), false) => {
            anyhow::bail!("Cannot specify --methodology without a use case ID. To regenerate all, use: mucm regenerate")
        }
    }
}
