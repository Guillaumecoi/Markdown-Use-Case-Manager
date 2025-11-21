// Formatter for displaying project status
use crate::core::{Status, UseCase};
use colored::Colorize;
use std::collections::HashMap;

/// Handles formatting and display of project status
pub struct StatusFormatter;

impl StatusFormatter {
    /// Display comprehensive project status
    pub fn display_project_status(use_cases: &[UseCase]) {
        let total_use_cases = use_cases.len();

        let mut status_counts: HashMap<Status, usize> = HashMap::new();
        for use_case in use_cases {
            *status_counts.entry(use_case.status()).or_insert(0) += 1;
        }

        println!("\n{}", "📊 Project Status".bold().blue());
        println!("{}", "━".repeat(50));
        println!("Total Use Cases: {}", total_use_cases.to_string().cyan());
        println!();

        for (status, count) in status_counts {
            println!("{}: {}", status, count.to_string().cyan());
        }
    }
}
