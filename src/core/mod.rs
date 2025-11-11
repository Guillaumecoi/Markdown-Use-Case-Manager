// Core layer - Business logic and domain

// Private modules - implementation details
// pub(crate) needed because they're accessed from outside core module:
pub(crate) mod application;    // Used by: controller/use_case_controller.rs
pub(crate) mod domain;          // Used by: presentation/formatters
pub(crate) mod infrastructure;  // Used by: controller/project_controller.rs, config/mod.rs
pub(crate) mod processors;      // Used by: config/mod.rs

// Fully private - only used within core module:
mod utils;  // Used only by: core/application (internal to core)
