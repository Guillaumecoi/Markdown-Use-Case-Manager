// Integration tests module organization

// Test utilities
pub mod test_helpers;

// CLI integration tests
pub mod cli_auto_init_test;
pub mod cli_comprehensive_test;
pub mod cli_modular_test; // New modular CLI tests // Auto-init and settings tests
pub mod persona_cli_test; // Persona management CLI tests

// File system and workflow integration tests
pub mod filesystem_comprehensive_test;

// Template management integration tests
pub mod template_management_test;

// Template language integration tests
pub mod template_language_tests;

// CLI innovation tests showcasing TOML features
pub mod cli_toml_innovation_test;

// Code preservation integration tests
pub mod code_preservation_test;

// Error handling integration tests
pub mod error_handling_test;
