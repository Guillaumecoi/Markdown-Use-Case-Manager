// src/main.rs
use anyhow::Result;

mod cli;
mod config;
mod controller;
mod core;
mod presentation;

fn main() -> Result<()> {
    cli::run()
}
