// Formatter for displaying use case information
use crate::core::domain::entities::UseCase;
use colored::Colorize;

/// Handles formatting and display of use cases
pub struct UseCaseFormatter;

impl UseCaseFormatter {
    /// Display a list of use cases
    pub fn display_list(use_cases: &[UseCase]) {
        if use_cases.is_empty() {
            println!("No use cases found. Create one with 'mucm create'");
            return;
        }

        println!("\n{}", "📋 Use Cases".bold().blue());
        println!("{}", "━".repeat(50));

        for use_case in use_cases {
            let status_display = format!("{}", use_case.status());
            println!(
                "{} {} [{}] - {}",
                status_display,
                use_case.id.cyan(),
                use_case.category.yellow(),
                use_case.title.bold()
            );
            println!();
        }
    }

    /// Display a success message for use case creation
    pub fn display_created(use_case_id: &str, methodology: &str) {
        println!("💾 Saved {} with {} methodology", use_case_id, methodology);
    }

    /// Display a success message for use case regeneration
    pub fn display_regenerated(use_case_id: &str, methodology: &str) {
        println!(
            "✅ Regenerated {} with {} methodology",
            use_case_id, methodology
        );
    }

    /// Display a success message for markdown regeneration
    pub fn display_markdown_regenerated(use_case_id: &str) {
        println!(
            "📝 Regenerated {}.md from {}.toml",
            use_case_id, use_case_id
        );
    }

    /// Display a success message for all use cases regenerated
    pub fn display_all_regenerated(count: usize) {
        for _ in 0..count {
            // Individual messages shown during iteration
        }
        println!("📝 Regenerated overview");
    }

    /// Display test generation info
    /// Display confirmation when test file is generated
    /// TODO: Call this when test generation is re-implemented
    pub fn display_test_generated(use_case_id: &str, test_file_path: &str) {
        println!(
            "✅ Generated test: {} -> {}",
            use_case_id.cyan(),
            test_file_path
        );
    }

    /// Display test file skipped message
    /// Display message when test generation is skipped
    /// TODO: Call this when auto_generate_tests is false
    pub fn display_test_skipped() {
        println!("⚠️  Test file exists and overwrite_test_documentation=false, skipping");
    }
}
