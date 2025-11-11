// src/main.rs
use anyhow::Result;

mod cli;
mod config;
mod controller;
mod core;
mod presentation;

// Re-export types for integration tests only
#[cfg(test)]
pub use config::Config;
#[cfg(test)]
pub use core::LanguageRegistry;

fn main() -> Result<()> {
    cli::run()
}
