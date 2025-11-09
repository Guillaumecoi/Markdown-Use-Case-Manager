// Unit tests module organization
//
// Tests are organized into logical subdirectories for better maintainability:
// - cli/: Command-line interface unit tests
// - models/: Data model tests (UseCase, Status, Priority, Metadata)
// - core/: Core functionality (config, coordinator, templates, processors, language)
// - services/: Service layer tests
// - features/: Extended feature tests

// CLI unit tests
pub mod cli;

// Model unit tests
pub mod models;

// Core functionality unit tests
pub mod core;

// Service unit tests
pub mod services;

// Extended functionality unit tests
pub mod features;
