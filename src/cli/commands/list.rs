use crate::cli::runner::CliRunner;
use anyhow::Result;

pub fn handle_list_command(runner: &mut CliRunner) -> Result<()> {
    runner.list_use_cases()
}

pub fn handle_languages_command() -> Result<()> {
    let result = CliRunner::show_languages()?;
    println!("{}", result);
    Ok(())
}
