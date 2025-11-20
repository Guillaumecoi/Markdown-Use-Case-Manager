//! CLI commands for managing actors (personas and system actors)

use crate::cli::args::{ActorCommands, PersonaCommands};
use crate::controller::ActorController;
use crate::core::{ActorType, Persona};
use crate::presentation::DisplayResultFormatter;
use anyhow::Result;
use std::str::FromStr;

/// Handle actor commands
pub fn handle_actor_command(command: ActorCommands) -> Result<()> {
    let controller = ActorController::new()?;

    match command {
        ActorCommands::CreatePersona { id, name } => {
            let result = controller.create_persona(id, name)?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
        ActorCommands::CreateSystem {
            id,
            name,
            actor_type,
            emoji,
        } => {
            let result = controller.create_system_actor(id, name, actor_type, emoji)?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
        ActorCommands::InitStandard => {
            // Show SQLite disclaimer if using SQLite backend
            if controller.is_using_sqlite() {
                println!("⚠️  WARNING: SQLite backend is currently WORK IN PROGRESS and may be buggy. Use at your own risk!");
                println!();
            }

            let result = controller.init_standard_actors()?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
        ActorCommands::UpdateEmoji { id, emoji } => {
            let result = controller.update_emoji(id, emoji)?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
        ActorCommands::List { actor_type } => {
            let type_filter = if let Some(type_str) = actor_type {
                Some(ActorType::from_str(&type_str).map_err(|e| anyhow::anyhow!(e))?)
            } else {
                None
            };
            list_actors_with_controller(&controller, type_filter)
        }
        ActorCommands::Show { id } => show_actor_with_controller(&controller, &id),
        ActorCommands::Delete { id } => {
            let result = controller.delete_actor(id)?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
    }
}

/// List actors using the controller
fn list_actors_with_controller(
    controller: &ActorController,
    type_filter: Option<ActorType>,
) -> Result<()> {
    let actors = controller.list_actors(type_filter)?;

    if actors.is_empty() {
        println!("No actors found.");
        println!("\n💡 Tip: Create standard system actors with: mucm actor init-standard");
        return Ok(());
    }

    // Group by type
    let personas: Vec<_> = actors
        .iter()
        .filter(|a| a.actor_type == ActorType::Persona)
        .collect();
    let systems: Vec<_> = actors
        .iter()
        .filter(|a| a.actor_type != ActorType::Persona)
        .collect();

    if !personas.is_empty() {
        println!("👥 Personas ({}):", personas.len());
        for actor in personas {
            println!("  {} {} - {}", actor.emoji, actor.name, actor.id);
        }
        println!();
    }

    if !systems.is_empty() {
        println!("⚙️  System Actors ({}):", systems.len());
        for actor in systems {
            println!(
                "  {} {} - {} [{}]",
                actor.emoji, actor.name, actor.id, actor.actor_type
            );
        }
        println!();
    }

    Ok(())
}

/// Show actor details using the controller
fn show_actor_with_controller(controller: &ActorController, id: &str) -> Result<()> {
    let actor = controller.get_actor(id)?;

    println!("{} {}", actor.emoji, actor.name);
    println!("ID: {}", actor.id);
    println!("Type: {}", actor.actor_type);
    println!("Created: {}", actor.metadata.created_at);
    println!("Last Updated: {}", actor.metadata.updated_at);

    if !actor.extra.is_empty() {
        println!("\nFields:");
        for (key, value) in &actor.extra {
            match value {
                serde_json::Value::String(s) if !s.is_empty() => {
                    println!("  {}: {}", key, s);
                }
                serde_json::Value::Array(arr) if !arr.is_empty() => {
                    println!("  {}:", key);
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            println!("    - {}", s);
                        }
                    }
                }
                serde_json::Value::Number(n) => {
                    println!("  {}: {}", key, n);
                }
                serde_json::Value::Bool(b) => {
                    println!("  {}: {}", key, b);
                }
                _ => {} // Skip empty or null values
            }
        }
    }

    Ok(())
}

/// Handle persona commands (legacy support)
pub fn handle_persona_command(command: PersonaCommands) -> Result<()> {
    let controller = ActorController::new()?;

    match command {
        PersonaCommands::Create { id, name } => {
            let result = controller.create_persona(id, name)?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
        PersonaCommands::List => {
            let personas = controller.list_personas()?;
            list_personas_legacy(&personas);
            Ok(())
        }
        PersonaCommands::Show { id } => show_persona(&id, controller),
        PersonaCommands::UseCases { id } => list_use_cases_for_persona(&id),
        PersonaCommands::Delete { id } => {
            let result = controller.delete_persona(id)?;
            DisplayResultFormatter::display(&result);
            Ok(())
        }
    }
}

/// List personas (legacy format)
fn list_personas_legacy(personas: &[Persona]) {
    if personas.is_empty() {
        println!("No personas found.");
        return;
    }

    println!("Personas ({}):\n", personas.len());
    for persona in personas {
        println!("  {} {} - {}", persona.emoji(), persona.name, persona.id);

        // Show a few key extra fields if they exist
        if let Some(role) = persona.extra.get("role") {
            if let Some(role_str) = role.as_str() {
                println!("     Role: {}", role_str);
            }
        }
        if let Some(dept) = persona.extra.get("department") {
            if let Some(dept_str) = dept.as_str() {
                println!("     Department: {}", dept_str);
            }
        }
        println!();
    }
}

/// Show persona details (legacy support)
fn show_persona(id: &str, controller: ActorController) -> Result<()> {
    let persona = controller.get_persona(id)?;

    println!("{} {}", persona.emoji(), persona.name);
    println!("ID: {}", persona.id);
    println!("Created: {}", persona.metadata.created_at);
    println!("Last Updated: {}", persona.metadata.updated_at);

    if !persona.extra.is_empty() {
        println!("\nFields:");
        for (key, value) in &persona.extra {
            match value {
                serde_json::Value::String(s) if !s.is_empty() => {
                    println!("  {}: {}", key, s);
                }
                serde_json::Value::Array(arr) if !arr.is_empty() => {
                    println!("  {}:", key);
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            println!("    - {}", s);
                        }
                    }
                }
                serde_json::Value::Number(n) => {
                    println!("  {}: {}", key, n);
                }
                serde_json::Value::Bool(b) => {
                    println!("  {}: {}", key, b);
                }
                _ => {} // Skip empty or null values
            }
        }
    } else {
        println!("\n(No additional fields defined)");
    }

    Ok(())
}

/// List use cases that use a specific persona
fn list_use_cases_for_persona(persona_id: &str) -> Result<()> {
    use crate::cli::standard::CliRunner;
    use crate::presentation::DisplayResultFormatter;

    let mut runner = CliRunner::new();

    let result = runner.list_use_cases_for_persona(persona_id.to_string())?;
    DisplayResultFormatter::display(&result);

    Ok(())
}
// Note: Tests for actor commands are in integration tests
// See tests/persona_management_integration_test.rs and related
