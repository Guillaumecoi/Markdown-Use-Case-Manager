// Integration tests module organization

// Test utilities
pub mod test_helpers;

// CLI integration tests
pub mod cli_comprehensive_test;
pub mod cli_modular_test; // New modular CLI tests
pub mod cli_auto_init_test; // Auto-init and settings tests

// File system and workflow integration tests
pub mod filesystem_comprehensive_test;

// Template management integration tests
pub mod template_management_test;

// Template language integration tests
pub mod template_language_tests;

// Code preservation integration tests
pub mod code_preservation_test;
