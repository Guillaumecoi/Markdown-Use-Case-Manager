use crate::cli::runner::CliRunner;
use anyhow::Result;

pub fn handle_create_command(
    runner: &mut CliRunner,
    title: String,
    category: String,
    description: Option<String>,
) -> Result<()> {
    let result = runner.create_use_case(title, category, description)?;
    println!("{}", result);
    Ok(())
}
