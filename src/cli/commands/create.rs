use crate::cli::runner::CliRunner;
use anyhow::Result;

pub fn handle_create_command(
    runner: &mut CliRunner,
    title: String,
    category: String,
    description: Option<String>,
    methodology: Option<String>,
) -> Result<()> {
    let result = match methodology {
        Some(methodology) => {
            runner.create_use_case_with_methodology(title, category, description, methodology)?
        }
        None => runner.create_use_case(title, category, description)?,
    };
    println!("{}", result);
    Ok(())
}
