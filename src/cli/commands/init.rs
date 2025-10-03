use crate::cli::runner::CliRunner;
use anyhow::Result;

pub fn handle_init_command(runner: &mut CliRunner, language: Option<String>) -> Result<()> {
    println!("Initializing use case manager project...");
    let result = runner.init_project(language)?;
    println!("{}", result);
    Ok(())
}
