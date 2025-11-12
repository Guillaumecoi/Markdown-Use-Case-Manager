//! # Interactive Menus Module
//!
//! Reusable menu system for interactive CLI mode.
//! Provides a generic, type-safe menu framework with pluggable actions.
//!
//! ## Architecture
//!
//! This module implements a flexible menu system using generic types and closures:
//!
//! - **`common`**: Core menu infrastructure with `MenuOption<T>` struct and `display_menu()` function
//! - **`menu`**: Main application menu with session management and navigation
//! - **`settings`**: Configuration menu for project settings and preferences
//!
//! ## Key Components
//!
//! - **`MenuOption<T>`**: Generic struct pairing display text with executable actions
//! - **`display_menu()`**: Generic function for rendering and handling menu interactions
//! - **Type Safety**: Menus are parameterized by context type (e.g., `CliRunner`, `Config`)
//! - **Action Closures**: Menu options execute closures that return `Result<bool>` to control flow
//!
//! ## Benefits
//!
//! - **Reusable**: Same infrastructure works for any menu type
//! - **Type-Safe**: Context types ensure correct data flow
//! - **Extensible**: Easy to add new menu options and menu types
//! - **Consistent**: Unified API across all interactive menus

pub mod common;
pub mod menu;
pub mod settings;
