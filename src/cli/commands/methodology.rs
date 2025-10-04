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

/// Handle the regenerate use case with methodology command
pub fn handle_regenerate_command(
    runner: &mut CliRunner,
    use_case_id: String,
    methodology: String,
) -> Result<()> {
    let result = runner.regenerate_use_case_with_methodology(use_case_id, methodology)?;
    println!("{}", result);
    Ok(())
}