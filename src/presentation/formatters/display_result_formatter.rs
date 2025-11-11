// Formatter for displaying DisplayResult objects
use colored::Colorize;

/// Handles formatting and display of DisplayResult objects
pub struct DisplayResultFormatter;

impl DisplayResultFormatter {
    /// Format a DisplayResult with appropriate colors for CLI output
    ///
    /// Success messages are displayed in green, error messages in red.
    ///
    /// # Arguments
    /// * `result` - The DisplayResult to format
    ///
    /// # Returns
    /// A colored string representation of the result
    pub fn format_colored(result: &crate::controller::DisplayResult) -> colored::ColoredString {
        // Force colors to be enabled
        colored::control::set_override(true);

        if result.success {
            result.message.green()
        } else {
            result.message.red()
        }
    }

    /// Display a DisplayResult to stdout (for success) or stderr (for error)
    ///
    /// # Arguments
    /// * `result` - The DisplayResult to display
    pub fn display(result: &crate::controller::DisplayResult) {
        if result.success {
            println!("{}", Self::format_colored(result));
        } else {
            eprintln!("{}", Self::format_colored(result));
        }
    }
}