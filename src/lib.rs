// src/lib.rs
//! Use Case Manager - A library for managing use cases, scenarios, and generating tests

pub mod config;
pub mod core;

pub use core::manager::UseCaseManager;
pub use core::models::{UseCase, Scenario, Status, Priority};

use anyhow::Result;

/// Initialize a new use case manager project
pub fn init_project() -> Result<()> {
    config::Config::init_project()?;
    Ok(())
}