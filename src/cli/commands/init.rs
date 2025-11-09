use crate::cli::runner::CliRunner;
use anyhow::Result;

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
