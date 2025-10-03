use anyhow::Result;
use crate::cli::runner::CliRunner;

pub fn handle_add_scenario_command(
    runner: &mut CliRunner,
    use_case_id: String,
    title: String,
    description: Option<String>,
) -> Result<()> {
    let result = runner.add_scenario(use_case_id, title, description)?;
    println!("{}", result);
    Ok(())
}

pub fn handle_update_status_command(
    runner: &mut CliRunner,
    scenario_id: String,
    status: String,
) -> Result<()> {
    let result = runner.update_scenario_status(scenario_id, status)?;
    println!("{}", result);
    Ok(())
}