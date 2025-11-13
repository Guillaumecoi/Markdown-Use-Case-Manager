use crate::cli::standard::CliRunner;
use crate::controller::DisplayResult;
use crate::presentation::DisplayResultFormatter;
use anyhow::Result;

/// Handles the 'precondition add' CLI command.
pub fn handle_precondition_add_command(
    runner: &mut CliRunner,
    use_case_id: String,
    precondition: String,
) -> Result<()> {
    let result = match runner.add_precondition(use_case_id, precondition) {
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

/// Handles the 'precondition list' CLI command.
pub fn handle_precondition_list_command(runner: &mut CliRunner, use_case_id: String) -> Result<()> {
    let result = match runner.list_preconditions(use_case_id) {
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

/// Handles the 'precondition remove' CLI command.
pub fn handle_precondition_remove_command(
    runner: &mut CliRunner,
    use_case_id: String,
    index: usize,
) -> Result<()> {
    let result = match runner.remove_precondition(use_case_id, index) {
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

/// Handles the 'postcondition add' CLI command.
pub fn handle_postcondition_add_command(
    runner: &mut CliRunner,
    use_case_id: String,
    postcondition: String,
) -> Result<()> {
    let result = match runner.add_postcondition(use_case_id, postcondition) {
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

/// Handles the 'postcondition list' CLI command.
pub fn handle_postcondition_list_command(
    runner: &mut CliRunner,
    use_case_id: String,
) -> Result<()> {
    let result = match runner.list_postconditions(use_case_id) {
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

/// Handles the 'postcondition remove' CLI command.
pub fn handle_postcondition_remove_command(
    runner: &mut CliRunner,
    use_case_id: String,
    index: usize,
) -> Result<()> {
    let result = match runner.remove_postcondition(use_case_id, index) {
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

/// Handles the 'reference add' CLI command.
pub fn handle_reference_add_command(
    runner: &mut CliRunner,
    use_case_id: String,
    target_id: String,
    relationship: String,
    description: Option<String>,
) -> Result<()> {
    let result = match runner.add_reference(use_case_id, target_id, relationship, description) {
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

/// Handles the 'reference list' CLI command.
pub fn handle_reference_list_command(runner: &mut CliRunner, use_case_id: String) -> Result<()> {
    let result = match runner.list_references(use_case_id) {
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

/// Handles the 'reference remove' CLI command.
pub fn handle_reference_remove_command(
    runner: &mut CliRunner,
    use_case_id: String,
    target_id: String,
) -> Result<()> {
    let result = match runner.remove_reference(use_case_id, target_id) {
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
