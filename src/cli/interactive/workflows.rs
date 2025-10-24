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
            println!("⚠️  No methodologies available, using default: {}", default_methodology);
            default_methodology
        } else {
            Select::new("Select methodology:", available)
                .with_help_message("Choose how this use case will be documented")
                .prompt()?
        }
    };

    // Create the use case with methodology
    let result = runner.create_use_case_with_methodology(
        title,
        category,
        description,
        methodology,
    )?;
    println!("\n✅ {}", result);

    // Ask if they want to add scenarios immediately
    let add_scenarios = Confirm::new("Would you like to add scenarios to this use case now?")
        .with_default(true)
        .prompt()?;

    if add_scenarios {
        // Extract the use case ID from the result message
        if let Some(use_case_id) = extract_use_case_id(&result) {
            guided_add_scenarios_to_use_case(runner, &use_case_id)?;
        }
    }

    Ok(())
}

/// Guided workflow for adding scenarios to a use case
pub fn guided_add_scenario(runner: &mut CliRunner) -> Result<()> {
    println!("\n➕ Adding a scenario to an existing use case...\n");

    // Get all use case IDs
    let use_case_ids = runner.get_use_case_ids()?;

    if use_case_ids.is_empty() {
        println!("❌ No use cases found. Create a use case first!");
        return Ok(());
    }

    // Select use case
    let use_case_id = Select::new("Select a use case:", use_case_ids)
        .with_page_size(10)
        .prompt()?;

    guided_add_scenarios_to_use_case(runner, &use_case_id)?;

    Ok(())
}

/// Helper function to add multiple scenarios to a specific use case
pub fn guided_add_scenarios_to_use_case(runner: &mut CliRunner, use_case_id: &str) -> Result<()> {
    loop {
        // Get scenario title
        let title = Text::new(&format!("Enter scenario title for {}:", use_case_id))
            .with_help_message("e.g., 'Happy path login', 'Invalid credentials', 'Account locked'")
            .prompt()?;

        // Get optional description
        let add_description =
            Confirm::new("Would you like to add a description for this scenario?")
                .with_default(false)
                .prompt()?;

        let description = if add_description {
            Some(Text::new("Enter the scenario description:").prompt()?)
        } else {
            None
        };

        // Add the scenario
        let result = runner.add_scenario(use_case_id.to_string(), title, description)?;
        println!("✅ {}", result);

        // Ask if they want to add more scenarios
        let add_more = Confirm::new("Would you like to add another scenario to this use case?")
            .with_default(false)
            .prompt()?;

        if !add_more {
            break;
        }
    }

    Ok(())
}

/// Guided workflow for updating scenario status
pub fn guided_update_scenario_status(runner: &mut CliRunner) -> Result<()> {
    println!("\n🔄 Updating scenario status...\n");

    // Get all use case IDs
    let use_case_ids = runner.get_use_case_ids()?;

    if use_case_ids.is_empty() {
        println!("❌ No use cases found. Create a use case first!");
        return Ok(());
    }

    // Select use case
    let use_case_id = Select::new("Select a use case:", use_case_ids)
        .with_page_size(10)
        .prompt()?;

    // Get scenarios for this use case
    let scenario_ids = runner.get_scenario_ids(&use_case_id)?;

    if scenario_ids.is_empty() {
        println!("❌ No scenarios found for this use case. Add scenarios first!");
        return Ok(());
    }

    // Select scenario
    let scenario_id = Select::new("Select a scenario:", scenario_ids)
        .with_page_size(10)
        .prompt()?;

    // Select new status
    let statuses = vec![
        "planned",
        "in_progress",
        "implemented",
        "tested",
        "deployed",
        "deprecated",
    ];

    let status = Select::new("Select new status:", statuses).prompt()?;

    // Update the status
    let result = runner.update_scenario_status(scenario_id, status.to_string())?;
    println!("✅ {}", result);

    Ok(())
}

/// Extract use case ID from result message
fn extract_use_case_id(result: &str) -> Option<String> {
    // Look for pattern "Created use case: UC-XXX-001"
    if let Some(pos) = result.find("UC-") {
        let id_part = &result[pos..];
        if let Some(end) = id_part.find(char::is_whitespace) {
            Some(id_part[..end].to_string())
        } else {
            Some(id_part.to_string())
        }
    } else {
        None
    }
}
