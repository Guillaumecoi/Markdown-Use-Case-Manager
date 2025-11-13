//! # Data Transfer Objects
//!
//! This module defines Data Transfer Objects (DTOs) used for communication
//! between the controller layer and other components. DTOs provide clean,
//! structured data exchange without exposing internal implementation details.
//!
//! ## DTOs Overview
//!
//! - `DisplayResult`: Standardized results for user-facing operations
//! - `SelectionOptions`: Available options for interactive selection
//! - `MethodologyInfo`: Methodology metadata for display and selection

/// Result of an operation that can be displayed to the user.
///
/// Provides a standardized way to communicate operation outcomes to the
/// presentation layer, including success status and user-friendly messages.
#[derive(Debug)]
pub struct DisplayResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// User-friendly message describing the operation result
    pub message: String,
}

impl std::fmt::Display for DisplayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl DisplayResult {
    /// Create a successful operation result.
    ///
    /// # Arguments
    /// * `message` - Success message to display to the user
    ///
    /// # Returns
    /// A DisplayResult indicating successful operation
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
        }
    }

    /// Create an error operation result.
    ///
    /// # Arguments
    /// * `message` - Error message to display to the user
    ///
    /// # Returns
    /// A DisplayResult indicating failed operation
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }

    /// Check if the operation was successful.
    ///
    /// # Returns
    /// true if the operation succeeded, false otherwise
    pub fn is_success(&self) -> bool {
        self.success
    }
}

/// Available options for user selection in interactive prompts.
///
/// Contains a list of selectable items that can be presented to users
/// in command-line interfaces or interactive workflows.
#[derive(Debug, Clone)]
pub struct SelectionOptions {
    /// List of available options for selection
    pub items: Vec<String>,
}

impl SelectionOptions {
    /// Create a new selection options instance.
    ///
    /// # Arguments
    /// * `items` - Vector of option strings available for selection
    ///
    /// # Returns
    /// A SelectionOptions instance with the provided items
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}

/// Methodology information for display and selection.
///
/// Contains metadata about a methodology including its name, display name,
/// and description, used for presenting methodology options to users.
#[derive(Debug, Clone)]
pub struct MethodologyInfo {
    /// Internal methodology identifier
    pub name: String,
    /// Human-readable display name (typically capitalized)
    pub display_name: String,
    /// Description of when and how to use this methodology
    pub description: String,
}

impl MethodologyInfo {
    /// Format methodology info as a single line for list display.
    ///
    /// # Returns
    /// A formatted string combining display name and description
    ///
    /// TODO: Use this in `mucm methodologies` command for cleaner output
    pub fn to_display_string(&self) -> String {
        format!("{} - {}", self.display_name, self.description)
    }
}
