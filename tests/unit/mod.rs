// Unit tests module organization

// Model unit tests
pub mod metadata_test;
pub mod priority_test;
pub mod scenario_test;
pub mod status_test;
pub mod use_case_test;

// Core functionality unit tests
pub mod config_test;
pub mod modular_language_test;
pub mod template_engine_test;

// Service unit tests (new refactored architecture)
pub mod use_case_service_test;

// CLI unit tests (new modular CLI architecture)
pub mod cli_runner_test;
pub mod cli_interactive_test;
