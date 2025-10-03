use crate::cli::args::PersonaAction;
use crate::cli::runner::CliRunner;
use crate::config::Config;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn handle_persona_command(runner: &mut CliRunner, action: PersonaAction) -> Result<()> {
    match action {
        PersonaAction::Create { name, description } => {
            create_persona(runner, &name, description.as_deref())?;
        }
        PersonaAction::List => {
            list_personas(runner)?;
        }
        PersonaAction::Delete { name } => {
            delete_persona(runner, &name)?;
        }
    }
    Ok(())
}

fn create_persona(_runner: &CliRunner, name: &str, description: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let personas_dir = Path::new(&config.directories.persona_dir);
    
    // Create personas directory if it doesn't exist
    if !personas_dir.exists() {
        fs::create_dir_all(personas_dir)?;
    }
    
    let filename = format!("{}.md", sanitize_filename(name));
    let persona_path = personas_dir.join(&filename);
    
    if persona_path.exists() {
        anyhow::bail!("Persona '{}' already exists", name);
    }
    
    let content = format!(
        "# Persona: {}\n\n{}\n\n## Characteristics\n\n- \n\n## Goals\n\n- \n\n## Pain Points\n\n- \n\n## Technical Skills\n\n- \n\n## Context of Use\n\n- \n",
        name,
        description.unwrap_or(&format!("Description for persona '{}'", name))
    );
    
    fs::write(&persona_path, content)?;
    
    println!("✅ Created persona '{}' at {}", name, persona_path.display());
    Ok(())
}

fn list_personas(_runner: &CliRunner) -> Result<()> {
    let config = Config::load()?;
    let personas_dir = Path::new(&config.directories.persona_dir);
    
    if !personas_dir.exists() {
        println!("📋 No personas directory found. Create personas with 'mucm persona create <name>'");
        return Ok(());
    }
    
    let mut personas = Vec::new();
    
    for entry in fs::read_dir(personas_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                // Read first few lines to get description
                let content = fs::read_to_string(&path)?;
                let description = extract_description(&content);
                
                personas.push((stem.to_string(), description));
            }
        }
    }
    
    if personas.is_empty() {
        println!("📋 No personas found. Create personas with 'mucm persona create <name>'");
    } else {
        println!("👥 Available Personas:\n");
        for (name, description) in personas {
            println!("• {}", name);
            if let Some(desc) = description {
                println!("  {}", desc);
            }
            println!();
        }
    }
    
    Ok(())
}

fn delete_persona(_runner: &CliRunner, name: &str) -> Result<()> {
    let config = Config::load()?;
    let personas_dir = Path::new(&config.directories.persona_dir);
    let filename = format!("{}.md", sanitize_filename(name));
    let persona_path = personas_dir.join(&filename);
    
    if !persona_path.exists() {
        anyhow::bail!("Persona '{}' not found", name);
    }
    
    fs::remove_file(&persona_path)?;
    println!("🗑️  Deleted persona '{}'", name);
    
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            ' ' => '-',
            c if c.is_alphanumeric() || c == '-' || c == '_' => c,
            _ => '_',
        })
        .collect::<String>()
        .to_lowercase()
}

fn extract_description(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    
    // Look for description after the title (line starting with #)
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with('#') && i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if !next_line.is_empty() && !next_line.starts_with('#') {
                return Some(next_line.to_string());
            }
            // Look at the line after that if the next line is empty
            if i + 2 < lines.len() {
                let line_after = lines[i + 2].trim();
                if !line_after.is_empty() && !line_after.starts_with('#') {
                    return Some(line_after.to_string());
                }
            }
        }
    }
    None
}