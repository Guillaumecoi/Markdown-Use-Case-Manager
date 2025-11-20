//! # Scenario Workflow
//!
//! Interactive scenario management within use cases.
//! Provides guided workflows for scenario operations.

use anyhow::Result;
use inquire::{Select, Text};

use crate::cli::interactive::{runner::InteractiveRunner, ui::UI};
use crate::controller::ScenarioController;

/// Scenario workflow handler
pub struct ScenarioWorkflow;

impl ScenarioWorkflow {
    /// Main scenario management entry point for a use case
    ///
    /// # Arguments
    /// * `use_case_id` - The ID of the use case to manage scenarios for
    pub fn manage_scenarios(use_case_id: &str) -> Result<()> {
        loop {
            UI::show_section_header(&format!("Manage Scenarios - {}", use_case_id), "🎬")?;

            // Show existing scenarios
            let mut controller = ScenarioController::new()?;
            let scenarios = controller.get_scenarios(use_case_id)?;

            if scenarios.is_empty() {
                println!("\n  No scenarios yet.\n");
            } else {
                println!("\n  Existing scenarios:");
                for scenario in &scenarios {
                    println!(
                        "    • {} - {} [{}] ({} steps)",
                        scenario.id,
                        scenario.title,
                        scenario.scenario_type,
                        scenario.steps.len()
                    );
                }
                println!();
            }

            // Show action menu
            let actions = vec![
                "Create new scenario",
                "Edit scenario",
                "Delete scenario",
                "Manage scenario steps",
                "Assign persona to scenario",
                "Back to use case menu",
            ];

            let choice = Select::new("What would you like to do?", actions).prompt()?;

            match choice {
                "Create new scenario" => {
                    Self::create_scenario(use_case_id)?;
                }
                "Edit scenario" => {
                    Self::edit_scenario(use_case_id)?;
                }
                "Delete scenario" => {
                    Self::delete_scenario(use_case_id)?;
                }
                "Manage scenario steps" => {
                    Self::manage_steps(use_case_id)?;
                }
                "Assign persona to scenario" => {
                    Self::assign_persona(use_case_id)?;
                }
                "Back to use case menu" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Create a new scenario interactively
    fn create_scenario(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Create Scenario", "➕")?;

        let title = Text::new("Scenario title:")
            .with_help_message("Brief title for the scenario")
            .prompt()?;

        let scenario_types = vec!["main", "alternative", "exception"];
        let scenario_type = Select::new("Scenario type:", scenario_types).prompt()?;

        let description = Text::new("Description (optional):")
            .with_help_message("Detailed description of the scenario")
            .prompt()
            .ok();

        // Ask if they want to assign a persona
        let assign_persona_choice =
            Select::new("Assign a persona to this scenario?", vec!["No", "Yes"]).prompt()?;

        let persona_id = if assign_persona_choice == "Yes" {
            let mut runner = InteractiveRunner::new();
            let persona_ids = runner.get_persona_ids()?;

            if persona_ids.is_empty() {
                println!("\n  No personas available. Skipping persona assignment.\n");
                None
            } else {
                let persona = Select::new("Select persona:", persona_ids).prompt()?;
                Some(persona)
            }
        } else {
            None
        };

        // Create the scenario
        let mut controller = ScenarioController::new()?;
        let result = controller.create_scenario(
            use_case_id.to_string(),
            title,
            scenario_type.to_string(),
            description,
            persona_id,
        )?;

        UI::show_success(&result.message)?;
        UI::pause_for_input()?;

        Ok(())
    }

    /// Edit an existing scenario
    fn edit_scenario(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Edit Scenario", "✏️")?;

        let mut controller = ScenarioController::new()?;
        let scenarios = controller.get_scenarios(use_case_id)?;

        if scenarios.is_empty() {
            println!("\n  No scenarios to edit.\n");
            UI::pause_for_input()?;
            return Ok(());
        }

        // Select scenario to edit
        let scenario_options: Vec<String> = scenarios
            .iter()
            .map(|s| format!("{} - {}", s.id, s.title))
            .collect();

        let selected = Select::new("Select scenario to edit:", scenario_options).prompt()?;
        let scenario_id = selected.split(" - ").next().unwrap();

        // Get current scenario
        let scenario = scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .unwrap()
            .clone();

        loop {
            println!("\n  Current values:");
            println!("    Title: {}", scenario.title);
            println!("    Type: {}", scenario.scenario_type);
            println!("    Description: {}", scenario.description);
            println!("    Status: {}", scenario.status);
            println!();

            let fields = vec![
                "Edit title",
                "Edit description",
                "Edit type",
                "Edit status",
                "Done editing",
            ];

            let choice = Select::new("What would you like to edit?", fields).prompt()?;

            match choice {
                "Edit title" => {
                    let new_title = Text::new("New title:")
                        .with_default(&scenario.title)
                        .prompt()?;

                    controller.edit_scenario(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        Some(new_title),
                        None,
                        None,
                        None,
                    )?;

                    UI::show_success("✓ Title updated")?;
                }
                "Edit description" => {
                    let new_desc = Text::new("New description:")
                        .with_default(&scenario.description)
                        .prompt()?;

                    controller.edit_scenario(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        None,
                        Some(new_desc),
                        None,
                        None,
                    )?;

                    UI::show_success("✓ Description updated")?;
                }
                "Edit type" => {
                    let types = vec!["main", "alternative", "exception"];
                    let new_type = Select::new("New type:", types).prompt()?;

                    controller.edit_scenario(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        None,
                        None,
                        Some(new_type.to_string()),
                        None,
                    )?;

                    UI::show_success("✓ Type updated")?;
                }
                "Edit status" => {
                    let statuses =
                        vec!["Planned", "InProgress", "Implemented", "Tested", "Deployed"];
                    let new_status = Select::new("New status:", statuses).prompt()?;

                    controller.edit_scenario(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        None,
                        None,
                        None,
                        Some(new_status.to_string()),
                    )?;

                    UI::show_success("✓ Status updated")?;
                }
                "Done editing" => break,
                _ => {}
            }
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Delete a scenario
    fn delete_scenario(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Delete Scenario", "🗑️")?;

        let mut controller = ScenarioController::new()?;
        let scenarios = controller.get_scenarios(use_case_id)?;

        if scenarios.is_empty() {
            println!("\n  No scenarios to delete.\n");
            UI::pause_for_input()?;
            return Ok(());
        }

        // Select scenario to delete
        let scenario_options: Vec<String> = scenarios
            .iter()
            .map(|s| format!("{} - {}", s.id, s.title))
            .collect();

        let selected = Select::new("Select scenario to delete:", scenario_options).prompt()?;
        let scenario_id = selected.split(" - ").next().unwrap();

        // Confirm deletion
        let confirm = Select::new(
            &format!("Are you sure you want to delete '{}'?", scenario_id),
            vec!["No", "Yes"],
        )
        .prompt()?;

        if confirm == "Yes" {
            let result =
                controller.delete_scenario(use_case_id.to_string(), scenario_id.to_string())?;
            UI::show_success(&result.message)?;
        } else {
            println!("\n✓ Deletion cancelled.");
        }

        UI::pause_for_input()?;
        Ok(())
    }

    /// Manage steps within a scenario
    fn manage_steps(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Manage Scenario Steps", "📝")?;

        let mut controller = ScenarioController::new()?;
        let scenarios = controller.get_scenarios(use_case_id)?;

        if scenarios.is_empty() {
            println!("\n  No scenarios available.\n");
            UI::pause_for_input()?;
            return Ok(());
        }

        // Select scenario
        let scenario_options: Vec<String> = scenarios
            .iter()
            .map(|s| format!("{} - {}", s.id, s.title))
            .collect();

        let selected = Select::new("Select scenario:", scenario_options).prompt()?;
        let scenario_id = selected.split(" - ").next().unwrap();

        loop {
            // Show current steps
            let scenario = controller.get_scenario(use_case_id, scenario_id)?;

            println!("\n  Current steps:");
            if scenario.steps.is_empty() {
                println!("    (no steps)");
            } else {
                for step in &scenario.steps {
                    println!("    {}. {} - {}", step.order, step.actor, step.action);
                }
            }
            println!();

            let actions = vec!["Add step", "Edit step", "Remove step", "Back"];
            let choice = Select::new("What would you like to do?", actions).prompt()?;

            match choice {
                "Add step" => {
                    let description = Text::new("Step description:")
                        .with_help_message("What happens in this step")
                        .prompt()?;

                    let result = controller.add_step(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        description,
                        None, // Will append
                    )?;

                    UI::show_success(&result.message)?;
                }
                "Edit step" => {
                    if scenario.steps.is_empty() {
                        println!("\n  No steps to edit.\n");
                        continue;
                    }

                    let step_choices: Vec<String> = scenario
                        .steps
                        .iter()
                        .map(|s| format!("{}. {}", s.order, s.action))
                        .collect();

                    let selected_step =
                        Select::new("Select step to edit:", step_choices).prompt()?;
                    let step_order: u32 =
                        selected_step.split('.').next().unwrap().trim().parse()?;

                    let new_description = Text::new("New description:").prompt()?;

                    let result = controller.edit_step(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        step_order,
                        new_description,
                    )?;

                    UI::show_success(&result.message)?;
                }
                "Remove step" => {
                    if scenario.steps.is_empty() {
                        println!("\n  No steps to remove.\n");
                        continue;
                    }

                    let step_choices: Vec<String> = scenario
                        .steps
                        .iter()
                        .map(|s| format!("{}. {}", s.order, s.action))
                        .collect();

                    let selected_step =
                        Select::new("Select step to remove:", step_choices).prompt()?;
                    let step_order: u32 =
                        selected_step.split('.').next().unwrap().trim().parse()?;

                    let result = controller.remove_step(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        step_order,
                    )?;

                    UI::show_success(&result.message)?;
                }
                "Back" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Assign persona to a scenario
    fn assign_persona(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Assign Persona", "👤")?;

        let mut controller = ScenarioController::new()?;
        let scenarios = controller.get_scenarios(use_case_id)?;

        if scenarios.is_empty() {
            println!("\n  No scenarios available.\n");
            UI::pause_for_input()?;
            return Ok(());
        }

        // Select scenario
        let scenario_options: Vec<String> = scenarios
            .iter()
            .map(|s| {
                let persona_info = match &s.persona {
                    Some(p) => format!(" [{}]", p),
                    None => " [no persona]".to_string(),
                };
                format!("{} - {}{}", s.id, s.title, persona_info)
            })
            .collect();

        let selected = Select::new("Select scenario:", scenario_options).prompt()?;
        let scenario_id = selected.split(" - ").next().unwrap();

        // Get available personas
        let mut runner = InteractiveRunner::new();
        let persona_ids = runner.get_persona_ids()?;

        if persona_ids.is_empty() {
            println!("\n  No personas available. Create personas first.\n");
            UI::pause_for_input()?;
            return Ok(());
        }

        let mut options = persona_ids.clone();
        options.insert(0, "Unassign persona".to_string());

        let choice = Select::new("Select persona:", options).prompt()?;

        if choice == "Unassign persona" {
            let result =
                controller.unassign_persona(use_case_id.to_string(), scenario_id.to_string())?;
            UI::show_success(&result.message)?;
        } else {
            let result = controller.assign_persona(
                use_case_id.to_string(),
                scenario_id.to_string(),
                choice,
            )?;
            UI::show_success(&result.message)?;
        }

        UI::pause_for_input()?;
        Ok(())
    }
}
