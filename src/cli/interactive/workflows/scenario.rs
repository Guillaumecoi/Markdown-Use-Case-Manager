//! # Scenario Workflow
//!
//! Interactive scenario management within use cases.
//! Provides guided workflows for scenario operations.

use anyhow::Result;
use inquire::{Confirm, Select, Text};

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
                "Back to use case menu" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Create a scenario for a specific use case (called after use case creation)
    pub fn create_scenario_for_use_case(use_case_id: &str) -> Result<()> {
        Self::create_scenario(use_case_id)
    }

    /// Create a new scenario interactively
    fn create_scenario(use_case_id: &str) -> Result<()> {
        UI::show_section_header("Create Scenario", "➕")?;

        let title = Text::new("Scenario title:")
            .with_help_message("Brief, descriptive title (e.g., 'User successfully logs in', 'Invalid password error')")
            .prompt()?;

        let scenario_types = vec!["main", "alternative", "exception"];
        let scenario_type = Select::new("Scenario type:", scenario_types).prompt()?;

        let description = Text::new("Description (optional):")
            .with_help_message("Describe what this scenario covers. Press Enter to skip.")
            .prompt()
            .ok();

        // Collect preconditions
        let preconditions = Self::collect_conditions("preconditions", use_case_id)?;

        // Collect postconditions
        let postconditions = Self::collect_conditions("postconditions", use_case_id)?;

        // Ask if they want to assign actors
        let assign_actors_choice =
            Select::new("Assign actors to this scenario?", vec!["No", "Yes"]).prompt()?;

        let actors = if assign_actors_choice == "Yes" {
            Self::select_multiple_actors()?
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
            None, // persona_id removed from interactive workflow
            preconditions,
            postconditions,
            actors,
        )?;

        UI::show_success(&result.message)?;
        UI::pause_for_input()?;

        Ok(())
    }

    /// Helper to collect preconditions or postconditions interactively
    fn collect_conditions(
        condition_type: &str,
        current_use_case_id: &str,
    ) -> Result<Option<Vec<String>>> {
        let add_conditions = Confirm::new(&format!("Add {}?", condition_type))
            .with_default(false)
            .prompt()?;

        if !add_conditions {
            return Ok(None);
        }

        let mut conditions = Vec::new();
        loop {
            // Ask how to add the condition
            let input_options = vec!["Type text manually", "Reference a use case"];
            let input_method = Select::new(
                &format!(
                    "How do you want to add this {}?",
                    condition_type.trim_end_matches('s')
                ),
                input_options,
            )
            .with_help_message("Choose to type text or select from existing use cases")
            .prompt()?;

            let condition = match input_method {
                "Reference a use case" => {
                    // Get list of use cases
                    use crate::controller::UseCaseController;
                    let uc_controller = UseCaseController::new()?;
                    let all_use_cases = uc_controller.get_all_use_cases()?;

                    let use_case_ids = all_use_cases
                        .iter()
                        .filter(|uc| uc.id != current_use_case_id) // Exclude current use case
                        .map(|uc| format!("{} - {}", uc.id, uc.title))
                        .collect::<Vec<_>>();

                    if use_case_ids.is_empty() {
                        UI::show_warning("No other use cases found. Please type manually.")?;
                        Text::new(&format!("  {} (or press Enter to finish):", condition_type))
                            .with_help_message("Enter condition text")
                            .prompt()?
                    } else {
                        let selected = Select::new("Select use case:", use_case_ids)
                            .with_help_message("Choose which use case this condition references")
                            .prompt()?;

                        let target_id = selected
                            .split(" - ")
                            .next()
                            .unwrap_or(&selected)
                            .to_string();

                        let condition_text = Text::new("Condition text:")
                            .with_help_message(&format!(
                                "Describe the {} related to {}",
                                condition_type.trim_end_matches('s'),
                                target_id
                            ))
                            .prompt()?;

                        let relationship_options =
                            vec!["requires", "depends_on", "must_complete", "extends"];
                        let relationship = Select::new("Relationship type:", relationship_options)
                            .with_help_message(
                                "How does this condition relate to the referenced use case?",
                            )
                            .prompt()?;

                        format!("{}||UC:{}:{}", condition_text, target_id, relationship)
                    }
                }
                _ => Text::new(&format!("  {} (or press Enter to finish):", condition_type))
                    .with_help_message(&format!(
                        "Enter a {}. You can also reference use cases.",
                        condition_type.trim_end_matches('s')
                    ))
                    .prompt()?,
            };

            if condition.trim().is_empty() {
                break;
            }

            conditions.push(condition);

            let add_more = Confirm::new(&format!(
                "Add another {}?",
                condition_type.trim_end_matches('s')
            ))
            .with_default(true)
            .prompt()?;

            if !add_more {
                break;
            }
        }

        if conditions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(conditions))
        }
    }

    /// Helper to select multiple actors interactively
    fn select_multiple_actors() -> Result<Option<Vec<String>>> {
        let runner = InteractiveRunner::new();
        let available_actors = runner.get_available_actors()?;

        if available_actors.is_empty() {
            println!("\n  No actors available. Create personas or system actors first.\n");
            return Ok(None);
        }

        let mut selected_actors = Vec::new();

        loop {
            let mut options = available_actors
                .iter()
                .filter(|a| {
                    // Extract ID from format "emoji name (id)"
                    let id = a.split('(').nth(1).and_then(|s| s.strip_suffix(')'));
                    !selected_actors
                        .iter()
                        .any(|selected: &String| Some(selected.as_str()) == id)
                })
                .cloned()
                .collect::<Vec<_>>();

            if options.is_empty() {
                break;
            }

            options.push("Done selecting".to_string());

            let choice = Select::new("Select actor:", options).prompt()?;

            if choice == "Done selecting" {
                break;
            }

            // Extract actor ID from the display string
            if let Some(id) = choice.split('(').nth(1).and_then(|s| s.strip_suffix(')')) {
                selected_actors.push(id.to_string());
                println!("  ✓ Added: {}", choice);
            }

            if selected_actors.is_empty() {
                let add_more = Confirm::new("Add another actor?")
                    .with_default(true)
                    .prompt()?;

                if !add_more {
                    break;
                }
            }
        }

        if selected_actors.is_empty() {
            Ok(None)
        } else {
            Ok(Some(selected_actors))
        }
    }

    /// Helper to select a single actor for a step
    fn select_actor_for_step() -> Result<Option<String>> {
        let runner = InteractiveRunner::new();
        let mut available_actors = runner.get_available_actors()?;

        if available_actors.is_empty() {
            println!("\n  No actors available. Using default 'Actor'.\n");
            return Ok(None);
        }

        // Add built-in actors
        available_actors.insert(0, "User".to_string());
        available_actors.insert(1, "System".to_string());
        available_actors.insert(2, "Default (Actor)".to_string());

        let choice = Select::new("Select actor for this step:", available_actors).prompt()?;

        if choice == "Default (Actor)" {
            Ok(None)
        } else if choice == "User" || choice == "System" {
            Ok(Some(choice))
        } else {
            // Extract actor ID from display string
            if let Some(id) = choice.split('(').nth(1).and_then(|s| s.strip_suffix(')')) {
                Ok(Some(format!("ref:{}", id)))
            } else {
                Ok(Some(choice))
            }
        }
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
        let _scenario = scenarios
            .iter()
            .find(|s| s.id == scenario_id)
            .unwrap()
            .clone();

        loop {
            // Refresh scenario data
            let scenario = controller.get_scenario(use_case_id, scenario_id)?;

            println!("\n  Current values:");
            println!("    Title: {}", scenario.title);
            println!("    Type: {}", scenario.scenario_type);
            println!("    Description: {}", scenario.description);
            println!("    Status: {}", scenario.status);
            println!("    Steps: {}", scenario.steps.len());
            println!("    Preconditions: {}", scenario.preconditions.len());
            println!("    Postconditions: {}", scenario.postconditions.len());
            if let Some(ref p) = scenario.persona {
                println!("    Persona: {}", p);
            }
            println!();

            let fields = vec![
                "Edit title",
                "Edit description",
                "Edit type",
                "Edit status",
                "Manage steps",
                "Manage preconditions",
                "Manage postconditions",
                "Assign persona",
                "Manage actors",
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
                "Manage steps" => {
                    Self::manage_steps_inline(use_case_id, scenario_id, &mut controller)?;
                }
                "Manage preconditions" => {
                    Self::manage_conditions_inline(
                        use_case_id,
                        scenario_id,
                        "preconditions",
                        &mut controller,
                    )?;
                }
                "Manage postconditions" => {
                    Self::manage_conditions_inline(
                        use_case_id,
                        scenario_id,
                        "postconditions",
                        &mut controller,
                    )?;
                }
                "Assign persona" => {
                    Self::assign_persona_inline(use_case_id, scenario_id, &mut controller)?;
                }
                "Manage actors" => {
                    Self::manage_actors_inline(use_case_id, scenario_id, &mut controller)?;
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

    /// Inline helper to manage steps within the edit scenario context
    fn manage_steps_inline(
        use_case_id: &str,
        scenario_id: &str,
        controller: &mut ScenarioController,
    ) -> Result<()> {
        loop {
            let scenario = controller.get_scenario(use_case_id, scenario_id)?;

            println!("\n  Current steps:");
            if scenario.steps.is_empty() {
                println!("    (no steps)");
            } else {
                for step in &scenario.steps {
                    let receiver_str = step
                        .receiver()
                        .map(|r| format!(" → {}", r.name()))
                        .unwrap_or_default();
                    println!(
                        "    {}. {}{} - {}",
                        step.order,
                        step.sender().name(),
                        receiver_str,
                        step.action
                    );
                }
            }
            println!();

            let actions = vec!["Add step", "Edit step", "Remove step", "Back"];
            let choice = Select::new("What would you like to do?", actions).prompt()?;

            match choice {
                "Add step" => {
                    let actor = Self::select_actor_for_step()?;

                    let add_receiver = Confirm::new("Add a receiving actor?")
                        .with_default(false)
                        .with_help_message("Does this action have a target/receiver?")
                        .prompt()?;

                    let receiver = if add_receiver {
                        Self::select_actor_for_step()?
                    } else {
                        None
                    };

                    let description = Text::new("Step description:")
                        .with_help_message(
                            "Describe the action (e.g., 'enters credentials', 'validates input')",
                        )
                        .prompt()?;

                    let result = controller.add_step(
                        use_case_id.to_string(),
                        scenario_id.to_string(),
                        description,
                        None,
                        actor,
                        receiver,
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

    /// Inline helper to manage conditions (pre/post) within the edit scenario context
    fn manage_conditions_inline(
        use_case_id: &str,
        scenario_id: &str,
        condition_type: &str,
        controller: &mut ScenarioController,
    ) -> Result<()> {
        loop {
            let scenario = controller.get_scenario(use_case_id, scenario_id)?;
            let conditions = match condition_type {
                "preconditions" => &scenario.preconditions,
                "postconditions" => &scenario.postconditions,
                _ => return Err(anyhow::anyhow!("Invalid condition type")),
            };

            println!("\n  Current {}:", condition_type);
            if conditions.is_empty() {
                println!("    (none)");
            } else {
                for (i, cond) in conditions.iter().enumerate() {
                    println!("    {}. {}", i + 1, cond);
                }
            }
            println!();

            let actions = vec!["Add condition", "Remove condition", "Back"];
            let choice = Select::new("What would you like to do?", actions).prompt()?;

            match choice {
                "Add condition" => {
                    let condition =
                        Text::new(&format!("Enter {}:", condition_type.trim_end_matches('s')))
                            .with_help_message("You can reference other use cases with 'UC-XXX'")
                            .prompt()?;

                    let result = if condition_type == "preconditions" {
                        controller.add_precondition(
                            use_case_id.to_string(),
                            scenario_id.to_string(),
                            condition,
                        )?
                    } else {
                        controller.add_postcondition(
                            use_case_id.to_string(),
                            scenario_id.to_string(),
                            condition,
                        )?
                    };

                    UI::show_success(&result.message)?;
                }
                "Remove condition" => {
                    if conditions.is_empty() {
                        println!("\n  No conditions to remove.\n");
                        continue;
                    }

                    let selected =
                        Select::new("Select condition to remove:", conditions.clone()).prompt()?;

                    let result = if condition_type == "preconditions" {
                        controller.remove_precondition(
                            use_case_id.to_string(),
                            scenario_id.to_string(),
                            selected.text.clone(),
                        )?
                    } else {
                        controller.remove_postcondition(
                            use_case_id.to_string(),
                            scenario_id.to_string(),
                            selected.text.clone(),
                        )?
                    };

                    UI::show_success(&result.message)?;
                }
                "Back" => break,
                _ => {}
            }
        }

        Ok(())
    }

    /// Inline helper to assign a persona within the edit scenario context
    fn assign_persona_inline(
        use_case_id: &str,
        scenario_id: &str,
        controller: &mut ScenarioController,
    ) -> Result<()> {
        let mut runner = InteractiveRunner::new();
        let persona_ids = runner.get_persona_ids()?;

        if persona_ids.is_empty() {
            println!("\n  No personas available.\n");
            UI::pause_for_input()?;
            return Ok(());
        }

        let mut options = vec!["(Clear persona assignment)"];
        options.extend(persona_ids.iter().map(|s| s.as_str()));

        let choice = Select::new("Select persona:", options).prompt()?;

        let result = if choice == "(Clear persona assignment)" {
            controller.unassign_persona(use_case_id.to_string(), scenario_id.to_string())?
        } else {
            controller.assign_persona(
                use_case_id.to_string(),
                scenario_id.to_string(),
                choice.to_string(),
            )?
        };

        UI::show_success(&result.message)?;
        Ok(())
    }

    /// Inline helper to manage actors within the edit scenario context
    fn manage_actors_inline(
        _use_case_id: &str,
        _scenario_id: &str,
        _controller: &mut ScenarioController,
    ) -> Result<()> {
        // TODO: Implement actors management when Scenario.actors field is available
        println!("\n  ⚠️  Actor management not yet implemented.");
        println!("     Actors are currently managed during scenario creation.\n");
        UI::pause_for_input()?;
        Ok(())
    }
}
