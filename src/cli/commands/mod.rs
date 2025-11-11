/// CLI command handlers.
///
/// These modules contain thin wrapper functions for CLI commands. They handle
/// command-line specific concerns like output formatting, but delegate all
/// business logic to the CliRunner. This separation keeps the CLI layer
/// focused on user interaction while the runner manages domain operations.
// Private modules
mod language;
mod methodology;
mod project;
mod usecase;

// Explicit public exports
pub use language::handle_languages_command;
pub use methodology::{
    handle_list_methodologies_command, handle_methodology_info_command, handle_regenerate_command,
};
pub use project::{handle_init_command, handle_status_command};
pub use usecase::{handle_create_command, handle_list_command};
