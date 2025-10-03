use anyhow::Result;
use crate::cli::runner::CliRunner;

pub fn handle_status_command(runner: &mut CliRunner) -> Result<()> {
    runner.show_status()
}