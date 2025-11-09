use crate::cli::runner::CliRunner;
use anyhow::Result;

pub fn handle_status_command(runner: &mut CliRunner) -> Result<()> {
    runner.show_status()
}
