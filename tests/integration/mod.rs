// Integration tests module organization
//
// Tests are organized into logical subdirectories for better maintainability:
// - cli/: Command-line interface tests
// - config/: Configuration and settings tests
// - core/: Core functionality (filesystem, regeneration, initialization, errors)
// - templates/: Template system tests (management, languages, code preservation)

// Test utilities
pub mod test_helpers;

// CLI integration tests
pub mod cli;

// Configuration integration tests
pub mod config;

// Core functionality tests
pub mod core;

// Template system tests
pub mod templates;
