/// CLI module for the Markdown Use Case Manager.
///
/// This module provides the command-line interface, handling argument parsing,
/// command dispatching, and user interaction modes. It integrates Clap for
/// argument parsing and coordinates between interactive and regular command modes.
///
/// ## Modules
/// - `args`: Defines CLI argument structures using Clap.
/// - `commands`: Thin command handlers that delegate to business logic.
/// - `interactive`: Interactive session management for guided usage.
/// - `runner`: Core business logic and file operations.
///
/// ## Flow
/// 1. Parse CLI arguments with Clap.
/// 2. Check for interactive mode (flag, subcommand, or no args).
/// 3. For regular commands, create a runner and dispatch to handlers.
/// 4. Handlers perform CLI-specific tasks and call runner methods.
// Private modules
mod args;
mod commands;
mod interactive;
mod runner;

use anyhow::Result;
use clap::Parser;

use args::{Cli, Commands};
use commands::{
    handle_create_command, handle_init_command, handle_languages_command, handle_list_command,
    handle_list_methodologies_command, handle_methodology_info_command, handle_regenerate_command,
    handle_status_command,
};
use interactive::InteractiveSession;
use runner::CliRunner;

/// Main CLI entry point.
///
/// Parses command-line arguments and dispatches to the appropriate handler.
/// Supports both interactive mode (for guided usage) and direct command execution.
///
/// Interactive mode is activated when:
/// - The `--interactive` flag is used
/// - The `interactive` subcommand is specified
/// - No command is provided (defaults to interactive)
///
/// For regular commands, creates a CliRunner instance and delegates to
/// command-specific handlers in the `commands` module.
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
        Commands::Init {
            language,
            methodology,
            finalize,
        } => handle_init_command(&mut runner, language, methodology, finalize),
        Commands::Create {
            title,
            category,
            description,
            methodology,
        } => handle_create_command(&mut runner, title, category, description, methodology),
        Commands::List => handle_list_command(&mut runner),
        Commands::Languages => handle_languages_command(),
        Commands::Methodologies => handle_list_methodologies_command(&mut runner),
        Commands::MethodologyInfo { name } => handle_methodology_info_command(&mut runner, name),
        Commands::Regenerate {
            use_case_id,
            methodology,
            all,
        } => handle_regenerate_command(&mut runner, use_case_id, methodology, all),
        Commands::Status => handle_status_command(&mut runner),
        Commands::Interactive => {
            // This case is handled above, but included for completeness
            let mut session = InteractiveSession::new();
            session.run()
        }
    }
}
