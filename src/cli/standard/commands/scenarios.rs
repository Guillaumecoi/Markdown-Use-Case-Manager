use crate::cli::standard::CliRunner;
use crate::controller::DisplayResult;
use crate::presentation::DisplayResultFormatter;
use anyhow::Result;

/// Handles the 'scenario add' CLI command.
///
/// Adds a new scenario to the specified use case with the given title, type, and optional description.
/// The scenario will be created with a default status of 'planned'.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case to add the scenario to.
/// * `title` - The title of the scenario.
/// * `scenario_type` - The type of scenario (main, alternative, exception).
/// * `description` - Optional detailed description of the scenario.
///
/// # Returns
/// Returns `Ok(())` on successful addition, or an error if addition fails.
pub fn handle_scenario_add_command(
    runner: &mut CliRunner,
    use_case_id: String,
    title: String,
    scenario_type: String,
    description: Option<String>,
) -> Result<()> {
    let result = match runner.add_scenario(use_case_id, title, scenario_type, description) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'scenario add-step' CLI command.
///
/// Adds a new step to the specified scenario with the given description.
/// If no order is provided, the step will be appended to the end of the scenario.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case containing the scenario.
/// * `scenario_title` - The title of the scenario to add the step to.
/// * `step` - The description of the step to add.
/// * `order` - Optional 1-based order for the step (will be appended if not specified).
///
/// # Returns
/// Returns `Ok(())` on successful addition, or an error if addition fails.
pub fn handle_scenario_add_step_command(
    runner: &mut CliRunner,
    use_case_id: String,
    scenario_title: String,
    step: String,
    order: Option<u32>,
) -> Result<()> {
    let result = match runner.add_scenario_step(use_case_id, scenario_title, step, order) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'scenario update-status' CLI command.
///
/// Updates the status of the specified scenario to the new status.
/// Valid status values are: planned, in-progress, completed, deprecated.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case containing the scenario.
/// * `scenario_title` - The title of the scenario to update.
/// * `status` - The new status for the scenario.
///
/// # Returns
/// Returns `Ok(())` on successful update, or an error if update fails.
pub fn handle_scenario_update_status_command(
    runner: &mut CliRunner,
    use_case_id: String,
    scenario_title: String,
    status: String,
) -> Result<()> {
    let result = match runner.update_scenario_status(use_case_id, scenario_title, status) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'scenario list' CLI command.
///
/// Lists all scenarios for the specified use case, including their titles,
/// types, statuses, and step counts.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case to list scenarios for.
///
/// # Returns
/// Returns `Ok(())` on successful display, or an error if retrieval fails.
pub fn handle_scenario_list_command(runner: &mut CliRunner, use_case_id: String) -> Result<()> {
    let result = match runner.list_scenarios(use_case_id) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'scenario remove-step' CLI command.
///
/// Removes the step at the specified order from the given scenario.
/// The order is 1-based, so the first step is order 1.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case containing the scenario.
/// * `scenario_title` - The title of the scenario to remove the step from.
/// * `order` - The 1-based order of the step to remove.
///
/// # Returns
/// Returns `Ok(())` on successful removal, or an error if removal fails.
pub fn handle_scenario_remove_step_command(
    runner: &mut CliRunner,
    use_case_id: String,
    scenario_title: String,
    order: u32,
) -> Result<()> {
    let result = match runner.remove_scenario_step(use_case_id, scenario_title, order) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

// ========== Scenario Reference Commands (PR #7) ==========

/// Handles the 'scenario reference add' CLI command.
///
/// Adds a reference from one scenario to another scenario or use case.
/// Valid reference types are 'scenario' and 'usecase'.
/// Valid relationship types include 'includes', 'extends', 'depends-on', 'alternative-to'.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case containing the source scenario.
/// * `scenario_title` - The title of the source scenario.
/// * `target_id` - The ID of the target scenario or use case.
/// * `ref_type` - The type of reference ('scenario' or 'usecase').
/// * `relationship` - The relationship type.
/// * `description` - Optional description of the reference.
///
/// # Returns
/// Returns `Ok(())` on successful addition, or an error if addition fails.
pub fn handle_scenario_reference_add_command(
    runner: &mut CliRunner,
    use_case_id: String,
    scenario_title: String,
    target_id: String,
    ref_type: String,
    relationship: String,
    description: Option<String>,
) -> Result<()> {
    let result = match runner.add_scenario_reference(
        use_case_id,
        scenario_title,
        target_id,
        ref_type,
        relationship,
        description,
    ) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'scenario reference remove' CLI command.
///
/// Removes a reference from a scenario to a specific target with a given relationship.
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case containing the scenario.
/// * `scenario_title` - The title of the scenario.
/// * `target_id` - The ID of the target to remove.
/// * `relationship` - The relationship type of the reference to remove.
///
/// # Returns
/// Returns `Ok(())` on successful removal, or an error if removal fails.
pub fn handle_scenario_reference_remove_command(
    runner: &mut CliRunner,
    use_case_id: String,
    scenario_title: String,
    target_id: String,
    relationship: String,
) -> Result<()> {
    let result = match runner.remove_scenario_reference(
        use_case_id,
        scenario_title,
        target_id,
        relationship,
    ) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Handles the 'scenario reference list' CLI command.
///
/// Lists all references for a specific scenario, showing the relationship type,
/// reference type, target ID, and description (if available).
///
/// # Arguments
/// * `runner` - A mutable reference to the CLI runner responsible for scenario operations.
/// * `use_case_id` - The ID of the use case containing the scenario.
/// * `scenario_title` - The title of the scenario to list references for.
///
/// # Returns
/// Returns `Ok(())` on successful display, or an error if retrieval fails.
pub fn handle_scenario_reference_list_command(
    runner: &mut CliRunner,
    use_case_id: String,
    scenario_title: String,
) -> Result<()> {
    let result = match runner.list_scenario_references(use_case_id, scenario_title) {
        Ok(display_result) => display_result,
        Err(e) => DisplayResult::error(e.to_string()),
    };

    DisplayResultFormatter::display(&result);

    if result.success {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
