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
use crate::controller::DisplayResult;
use crate::presentation::DisplayResultFormatter;

/// Execute a command with proper error handling and colored output
fn execute_command<F>(command_fn: F)
where
    F: FnOnce() -> Result<()>,
{
    match command_fn() {
        Ok(()) => {}
        Err(e) => {
            DisplayResultFormatter::display(&DisplayResult::error(e.to_string()));
            std::process::exit(1);
        }
    }
}

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
        } => {
            execute_command(|| handle_init_command(&mut runner, language, methodology, finalize));
            Ok(())
        }
        Commands::Create {
            title,
            category,
            description,
            methodology,
        } => {
            execute_command(|| handle_create_command(&mut runner, title, category, description, methodology));
            Ok(())
        }
        Commands::List => {
            execute_command(|| handle_list_command(&mut runner));
            Ok(())
        }
        Commands::Languages => {
            execute_command(|| handle_languages_command());
            Ok(())
        }
        Commands::Methodologies => {
            execute_command(|| handle_list_methodologies_command(&mut runner));
            Ok(())
        }
        Commands::MethodologyInfo { name } => {
            execute_command(|| handle_methodology_info_command(&mut runner, name));
            Ok(())
        }
        Commands::Regenerate {
            use_case_id,
            methodology,
            all,
        } => {
            execute_command(|| handle_regenerate_command(&mut runner, use_case_id, methodology, all));
            Ok(())
        }
        Commands::Status => {
            execute_command(|| handle_status_command(&mut runner));
            Ok(())
        }
        Commands::Interactive => {
            // This case is handled above, but included for completeness
            let mut session = InteractiveSession::new();
            session.run()
        }
    }
}
