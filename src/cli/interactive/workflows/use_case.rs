//! # Use Case Workflow
//!
//! Interactive use case management for creating and managing use cases.
//! Provides guided workflows for use case operations.

use anyhow::{Context, Result};
use inquire::{Confirm, Select, Text};
use std::collections::HashMap;

use crate::cli::interactive::{runner::InteractiveRunner, ui::UI};

/// Use case workflow handler
pub struct UseCaseWorkflow;

impl UseCaseWorkflow {
    /// Interactive use case creation workflow
    pub fn create_use_case() -> Result<()> {
        UI::show_section_header("Create Use Case", "🔄")?;

        let mut runner = InteractiveRunner::new();
        let methodologies = runner.get_installed_methodologies()?;

        if methodologies.is_empty() {
            UI::show_error(
                "No methodologies available. Please configure methodologies in your project.",
            )?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Step 1: Prompt for title and category first
        UI::show_info("\n📋 Required Fields")?;

        let title = Text::new("Title:")
            .with_help_message("A clear, descriptive title for the use case")
            .prompt()?;

        let category = Text::new("Category:")
            .with_help_message("Group this use case (e.g., 'authentication', 'data-processing')")
            .prompt()?;

        // Step 2: Collect views
        UI::show_section_header("Select Views", "👁️")?;
        UI::show_info("Add methodology views. Each view will generate a separate markdown file.")?;
        let mut views: Vec<(String, String)> = Vec::new();

        loop {
            // Display methodologies with their descriptions
            let methodology_options: Vec<String> = methodologies
                .iter()
                .map(|m| format!("{} - {}", m.display_name, m.description))
                .collect();

            let selected_idx = Select::new(
                &format!("Select methodology (view #{}):", views.len() + 1),
                methodology_options.clone(),
            )
            .with_help_message("Choose how you want to structure this view")
            .prompt()?;

            // Find the selected methodology
            let selected_methodology = &methodologies[methodologies
                .iter()
                .position(|m| format!("{} - {}", m.display_name, m.description) == selected_idx)
                .context("Selected methodology not found")?];

            let methodology_name = selected_methodology.name.clone();

            // Get available levels for this methodology
            let available_levels = runner.get_methodology_levels(&methodology_name)?;

            if available_levels.is_empty() {
                UI::show_error(&format!(
                    "No levels available for methodology '{}'",
                    methodology_name
                ))?;
                continue;
            }

            // Display levels with their descriptions
            let level_options: Vec<String> = available_levels
                .iter()
                .map(|level| {
                    let display_name = level
                        .name
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == 0 {
                                c.to_uppercase().next().unwrap()
                            } else {
                                c
                            }
                        })
                        .collect::<String>();
                    format!("{} - {}", display_name, level.description)
                })
                .collect();

            let selected_level_display = Select::new("Select level:", level_options)
                .with_help_message("Choose the detail level for this view")
                .prompt()?;

            // Extract just the level name and convert to lowercase
            let level = selected_level_display
                .split(" - ")
                .next()
                .context("Failed to parse level name")?
                .to_lowercase();

            views.push((methodology_name.clone(), level.clone()));

            UI::show_success(&format!("✓ Added view: {}:{}", methodology_name, level))?;

            // Ask if user wants to add another view
            let add_another = Confirm::new("Add another view?")
                .with_default(false)
                .with_help_message("Each view will generate a separate markdown file")
                .prompt()?;

            if !add_another {
                break;
            }
        }

        if views.is_empty() {
            UI::show_error("No views selected. Use case creation cancelled.")?;
            UI::pause_for_input()?;
            return Ok(());
        }

        // Always use interactive form for additional fields
        Self::fill_use_case_form(&mut runner, title, category, None, views)?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Prompt user for methodology-specific field values
    fn prompt_for_methodology_fields(
        runner: &InteractiveRunner,
        views: &[(String, String)],
    ) -> Result<HashMap<String, String>> {
        // Collect field definitions
        let field_collection = match runner.collect_methodology_fields(views) {
            Ok(collection) => collection,
            Err(e) => {
                // If we can't collect fields (e.g., methodology not found in workspace),
                // just warn and continue without methodology fields
                UI::show_warning(&format!(
                    "Could not collect methodology fields: {}. Continuing without methodology-specific fields.",
                    e
                ))?;
                return Ok(HashMap::new());
            }
        };

        // Show any warnings
        for warning in &field_collection.warnings {
            UI::show_warning(warning)?;
        }

        if field_collection.fields.is_empty() {
            return Ok(HashMap::new());
        }

        UI::show_section_header("Methodology Fields", "🎯")?;
        UI::show_info("These fields are defined by the methodologies you selected. Press Enter to skip optional fields.")?;

        let mut field_values = HashMap::new();

        // Group fields by methodology for better UX
        let mut fields_by_methodology: HashMap<String, Vec<&crate::core::CollectedField>> =
            HashMap::new();
        for field in field_collection.fields.values() {
            for methodology in &field.methodologies {
                fields_by_methodology
                    .entry(methodology.clone())
                    .or_insert_with(Vec::new)
                    .push(field);
            }
        }

        // Sort methodologies for consistent ordering
        let mut methodology_names: Vec<_> = fields_by_methodology.keys().collect();
        methodology_names.sort();

        // Prompt for each methodology's fields
        for methodology_name in methodology_names {
            let fields = fields_by_methodology.get(methodology_name).unwrap();
            if !fields.is_empty() {
                UI::show_info(&format!("\n📋 {} Fields:", methodology_name))?;

                for field in fields {
                    let default_help = format!("{} ({})", field.label, field.field_type);
                    let help_msg = field.description.as_deref().unwrap_or(&default_help);

                    let prompt_text = if field.required {
                        format!("{} (required):", field.label)
                    } else {
                        format!("{} (optional):", field.label)
                    };

                    // Handle different field types
                    let value = match field.field_type.as_str() {
                        "boolean" => {
                            // For boolean fields, use Confirm prompt
                            let default = field
                                .default
                                .as_ref()
                                .and_then(|d| d.parse::<bool>().ok())
                                .unwrap_or(false);

                            let result = Confirm::new(&prompt_text)
                                .with_default(default)
                                .with_help_message(help_msg)
                                .prompt()?;

                            Some(result.to_string())
                        }
                        "array" => {
                            // For array fields, collect items one by one
                            UI::show_info("  💡 Enter items one at a time. Press Enter on empty line when done.")?;

                            let mut items = Vec::new();
                            let mut item_num = 1;

                            loop {
                                let item_prompt = format!("  Item {}: ", item_num);
                                let result = Text::new(&item_prompt)
                                    .with_help_message(help_msg)
                                    .prompt_skippable()?;

                                match result {
                                    Some(item) if !item.trim().is_empty() => {
                                        items.push(item.trim().to_string());
                                        item_num += 1;
                                    }
                                    _ => break,
                                }
                            }

                            if items.is_empty() && field.required {
                                None // Will be handled by required field logic below
                            } else if items.is_empty() {
                                None
                            } else {
                                // Join items with newlines for array storage
                                Some(items.join("\n"))
                            }
                        }
                        "number" => {
                            // For number fields, validate input
                            loop {
                                let result = Text::new(&prompt_text)
                                    .with_help_message(help_msg)
                                    .with_default(field.default.as_deref().unwrap_or(""))
                                    .prompt_skippable()?;

                                match result {
                                    Some(ref s) if !s.trim().is_empty() => {
                                        // Try to parse as number
                                        if s.parse::<f64>().is_ok() {
                                            break Some(s.clone());
                                        } else {
                                            UI::show_error("Please enter a valid number")?;
                                            continue;
                                        }
                                    }
                                    _ => break None,
                                }
                            }
                        }
                        _ => {
                            // Default to string
                            let result = Text::new(&prompt_text)
                                .with_help_message(help_msg)
                                .with_default(field.default.as_deref().unwrap_or(""))
                                .prompt_skippable()?;

                            result.filter(|s| !s.trim().is_empty())
                        }
                    };

                    if let Some(v) = value {
                        field_values.insert(field.name.clone(), v);
                    } else if field.required && field.default.is_none() {
                        // Required field with no value and no default - use empty string
                        field_values.insert(field.name.clone(), String::new());
                    }
                }
            }
        }

        Ok(field_values)
    }

    /// Interactive form for filling use case fields
    fn fill_use_case_form(
        runner: &mut InteractiveRunner,
        title: String,
        category: String,
        description: Option<String>,
        views: Vec<(String, String)>,
    ) -> Result<()> {
        // Ask if user wants to fill additional fields
        let fill_additional = Confirm::new("Fill in additional fields now?")
            .with_default(false)
            .with_help_message("You can add description, author, reviewer, and other custom fields")
            .prompt()?;

        if !fill_additional {
            // Create use case with just the basic fields and default priority
            let result = runner.create_use_case_with_views_and_fields(
                title,
                category,
                description,
                "Medium".to_string(), // Default priority
                views.clone(),
                HashMap::new(),
            )?;

            UI::show_success(&result)?;

            // Show summary of created views
            UI::show_info("\n📄 Generated files:")?;
            for (methodology, level) in &views {
                println!("   • {}-{}.md", methodology, level);
            }

            UI::show_info("\n💡 You can edit the TOML files directly to add additional fields like author, reviewer, and custom methodology fields.")?;
            return Ok(());
        }

        UI::show_section_header("Additional Fields", "📝")?;

        // Priority (with default)
        let priority_options = vec!["Low", "Medium", "High", "Critical"];
        let priority = Select::new("Priority:", priority_options)
            .with_starting_cursor(1) // Default to "Medium"
            .with_help_message("Priority level for this use case")
            .prompt()?;

        // Description (if not already provided)
        let final_description = if description.is_some() {
            description
        } else {
            Text::new("Description:")
                .with_help_message("Brief description of what this use case accomplishes")
                .prompt_skippable()?
        };

        // Author (optional)
        let author = Text::new("Author (optional):")
            .with_help_message("Person who created this use case")
            .prompt_skippable()?;

        // Reviewer (optional)
        let reviewer = Text::new("Reviewer (optional):")
            .with_help_message("Person responsible for reviewing this use case")
            .prompt_skippable()?;

        // Collect methodology-specific field values
        let methodology_field_values = Self::prompt_for_methodology_fields(runner, &views)?;

        // Create the use case with additional fields (only truly extra fields)
        let mut extra_fields = HashMap::new();

        if let Some(auth) = author {
            if !auth.is_empty() {
                extra_fields.insert("author".to_string(), auth);
            }
        }

        if let Some(rev) = reviewer {
            if !rev.is_empty() {
                extra_fields.insert("reviewer".to_string(), rev);
            }
        }

        // Merge methodology field values into extra_fields
        extra_fields.extend(methodology_field_values);

        let result = runner.create_use_case_with_views_and_fields(
            title,
            category,
            final_description,
            priority.to_string(),
            views.clone(),
            extra_fields,
        )?;

        UI::show_success(&result)?;

        // Show summary of created views
        UI::show_info("\n📄 Generated files:")?;
        for (methodology, level) in &views {
            println!("   • {}-{}.md", methodology, level);
        }

        Ok(())
    }

    /// List all use cases
    pub fn list_use_cases() -> Result<()> {
        UI::show_section_header("Use Cases", "📋")?;

        let mut runner = InteractiveRunner::new();
        runner.list_use_cases()?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Show project status
    pub fn show_status() -> Result<()> {
        UI::show_section_header("Project Status", "📊")?;

        let mut runner = InteractiveRunner::new();
        runner.show_status()?;

        UI::pause_for_input()?;
        Ok(())
    }

    /// Interactive use case management menu
    pub fn manage_use_cases() -> Result<()> {
        UI::clear_screen()?;
        UI::show_section_header("Use Case Management", "📝")?;

        loop {
            let options = vec![
                "Create New Use Case",
                "List All Use Cases",
                "Show Project Status",
                "Back to Main Menu",
            ];

            let choice = Select::new("What would you like to do?", options).prompt()?;

            match choice {
                "Create New Use Case" => Self::create_use_case()?,
                "List All Use Cases" => Self::list_use_cases()?,
                "Show Project Status" => Self::show_status()?,
                "Back to Main Menu" => break,
                _ => {}
            }
        }

        Ok(())
    }
}
