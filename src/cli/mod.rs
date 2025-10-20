pub mod args;
pub mod commands;
pub mod interactive;
pub mod runner;

use anyhow::Result;
use clap::Parser;

use args::{Cli, Commands};
#[allow(clippy::wildcard_imports)]
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

    let Some(command) = cli.command else {
        // This shouldn't happen due to clap validation, but handle gracefully
        anyhow::bail!("No command specified. Use --help for available commands.");
    };

    match command {
        Commands::Init { language, methodology } => handle_init_command(&mut runner, language, methodology),
        Commands::Create {
            title,
            category,
            description,
            methodology,
        } => handle_create_command(&mut runner, title, category, description, methodology),
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
        Commands::Methodologies => handle_list_methodologies_command(&mut runner),
        Commands::MethodologyInfo { name } => handle_methodology_info_command(&mut runner, name),
        Commands::Regenerate { use_case_id, methodology, all } => handle_regenerate_command(&mut runner, use_case_id, methodology, all),
        Commands::Status => handle_status_command(&mut runner),
        Commands::Interactive => {
            // This case is handled above, but included for completeness
            let mut session = InteractiveSession::new();
            session.run()
        }
    }
}
