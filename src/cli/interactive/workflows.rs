use anyhow::Result;
use inquire::{Confirm, Select, Text};

use crate::cli::runner::CliRunner;
use crate::config::Config;

/// Guided workflow for creating a use case
pub fn guided_create_use_case(runner: &mut CliRunner) -> Result<()> {
    println!("\n🔧 Creating a new use case...\n");

    // Get title
    let title = Text::new("Enter the use case title:")
        .with_help_message("e.g., 'User Login', 'File Upload', 'Data Export'")
        .prompt()?;

    // Get existing categories for suggestions
    let existing_categories = runner.get_categories().unwrap_or_default();

    let category = if existing_categories.is_empty() {
        Text::new("Enter the category:")
            .with_help_message("e.g., 'auth', 'api', 'security', 'profile'")
            .prompt()?
    } else {
        // Allow selection from existing or entering new
        let mut options = existing_categories.clone();
        options.push("✏️  Enter a new category".to_string());

        let selection = Select::new("Select a category or create a new one:", options).prompt()?;

        if selection == "✏️  Enter a new category" {
            Text::new("Enter the new category:")
                .with_help_message("e.g., 'auth', 'api', 'security', 'profile'")
                .prompt()?
        } else {
            selection
        }
    };

    // Get optional description
    let add_description = Confirm::new("Would you like to add a description?")
        .with_default(false)
        .prompt()?;

    let description = if add_description {
        Some(
            Text::new("Enter the description:")
                .with_help_message("Brief description of what this use case covers")
                .prompt()?,
        )
    } else {
        None
    };

    // Ask for methodology
    let config = Config::load()?;
    let default_methodology = config.templates.default_methodology.clone();

    let use_default = Confirm::new(&format!(
        "Use default methodology '{}'?",
        default_methodology
    ))
    .with_default(true)
    .prompt()?;

    let methodology = if use_default {
        default_methodology
    } else {
        let available = Config::list_available_methodologies()?;
        if available.is_empty() {
            println!(
                "⚠️  No methodologies available, using default: {}",
                default_methodology
            );
            default_methodology
        } else {
            Select::new("Select methodology:", available)
                .with_help_message("Choose how this use case will be documented")
                .prompt()?
        }
    };

    // Create the use case with methodology
    let result =
        runner.create_use_case_with_methodology(title, category, description, methodology)?;
    println!("\n✅ {}", result);

    Ok(())
}
