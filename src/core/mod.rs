// Core layer - Business logic and domain

// Clean architecture modules
pub mod application;
pub mod domain;
pub mod infrastructure;

// Legacy modules (still being migrated)
pub mod processors;
pub mod utils;

// Re-export commonly used types for convenience
