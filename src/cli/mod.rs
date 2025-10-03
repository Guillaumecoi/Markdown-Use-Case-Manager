pub mod args;
pub mod commands;
pub mod interactive;
pub mod runner;

use anyhow::Result;
use clap::Parser;

use args::{Cli, Commands};
use commands::*;
use interactive::session::InteractiveSession;
use runner::CliRunner;

/// Main CLI entry point
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Check if interactive mode is requested
    if cli.interactive
        || matches!(cli.command, Some(Commands::Interactive))
        || cli.command.is_none()
    {
        let mut session = InteractiveSession::new();
        return session.run();
    }

    // Handle regular commands
    let mut runner = CliRunner::new();

    match cli.command.unwrap() {
        Commands::Init { language } => handle_init_command(&mut runner, language),
        Commands::Create {
            title,
            category,
            description,
        } => handle_create_command(&mut runner, title, category, description),
        Commands::AddScenario {
            use_case_id,
            title,
            description,
        } => handle_add_scenario_command(&mut runner, use_case_id, title, description),
        Commands::UpdateStatus {
            scenario_id,
            status,
        } => handle_update_status_command(&mut runner, scenario_id, status),
        Commands::Persona { action } => handle_persona_command(&mut runner, action),
        Commands::List => handle_list_command(&mut runner),
        Commands::Languages => handle_languages_command(),
        Commands::Status => handle_status_command(&mut runner),
        Commands::Interactive => {
            // This case is handled above, but included for completeness
            let mut session = InteractiveSession::new();
            session.run()
        }
    }
}
