//! # Interactive CLI Module
//!
//! Interactive command-line interface for the Markdown Use Case Manager.
//! Provides guided, menu-driven interaction for users who prefer wizard-style workflows.
//!
//! ## Architecture
//!
//! This module contains the interactive CLI implementation with a layered architecture:
//! - `runner.rs`: Business logic coordinator for interactive workflows
//! - `session.rs`: Interactive session management and main menu loop
//! - `initialization.rs`: Project initialization wizard
//! - `ui.rs`: Presentation layer for interactive prompts and displays
//! - `menu.rs`: Menu navigation and selection logic
//! - `settings.rs`: Interactive settings configuration
//! - `workflows.rs`: Specialized workflow handlers
//!
//! ## Interactive Workflows
//!
//! - **Project Initialization**: Guided setup wizard for new projects
//! - **Use Case Creation**: Interactive prompts for creating use cases
//! - **Settings Configuration**: Menu-driven settings management
//! - **Status Display**: Interactive project status viewing
//! - **Main Menu**: Central navigation hub for all interactive features

// Private modules - used only within CLI interactive
mod initialization;
mod menu;
mod runner;
mod session;
mod settings;
mod ui;
mod workflows;

// Public exports
pub use session::InteractiveSession;
