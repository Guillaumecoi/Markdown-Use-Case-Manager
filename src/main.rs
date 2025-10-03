// src/main.rs
use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod core;

use config::Config;
use core::languages::LanguageRegistry;
use core::use_case_coordinator::UseCaseCoordinator;

#[derive(Parser)]
#[command(name = "mucm")]
#[command(about = "Markdown Use Case Manager - Manage use cases and scenarios in markdown format")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new use case manager project
    Init {
        /// Programming language for test templates (rust, python, javascript, etc.)
        #[arg(short, long)]
        language: Option<String>,
    },
    /// Create a new use case
    Create {
        /// Use case title
        title: String,
        /// Category (e.g., "Authentication", "API")
        #[arg(short, long)]
        category: String,
        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Add a scenario to a use case
    AddScenario {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Scenario title
        title: String,
        /// Scenario description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Update scenario status
    UpdateStatus {
        /// Scenario ID (e.g., UC-SEC-001-S01)
        scenario_id: String,
        /// New status (planned, in_progress, implemented, tested, deployed, deprecated)
        #[arg(short, long)]
        status: String,
    },
    /// List all use cases
    List,
    /// List available programming languages for templates
    Languages,
    /// Show project status
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { language } => {
            println!("Initializing use case manager project...");
            let config = Config::init_project_with_language(language)?;
            println!("Project initialized! Configuration saved to .config/.mucm/mucm.toml");
            println!("Feel free to edit the configuration file to customize your setup.");
            println!(
                "Unless changed, use cases will be stored in: {}",
                config.directories.use_case_dir
            );
        }
        Commands::Create {
            title,
            category,
            description,
        } => {
            let mut coordinator = UseCaseCoordinator::load()?;
            let use_case_id = coordinator.create_use_case(title, category, description)?;
            println!("Created use case: {}", use_case_id);
        }
        Commands::AddScenario {
            use_case_id,
            title,
            description,
        } => {
            let mut coordinator = UseCaseCoordinator::load()?;
            let scenario_id =
                coordinator.add_scenario_to_use_case(use_case_id, title, description)?;
            println!("Added scenario: {}", scenario_id);
        }
        Commands::UpdateStatus {
            scenario_id,
            status,
        } => {
            let mut coordinator = UseCaseCoordinator::load()?;
            coordinator.update_scenario_status(scenario_id, status)?;
        }
        Commands::List => {
            let coordinator = UseCaseCoordinator::load()?;
            coordinator.list_use_cases()?;
        }
        Commands::Languages => {
            println!("Available programming languages:");
            match Config::get_available_languages() {
                Ok(languages) => {
                    for lang in languages {
                        println!("  - {}", lang);
                    }
                    println!("\nTo initialize with a specific language: mucm init -l <language>");
                    println!("To add a new language manually, create a directory: .config/.mucm/templates/lang-<language>/");
                }
                Err(e) => {
                    eprintln!("Error getting available languages: {}", e);
                    let language_registry = LanguageRegistry::new();
                    let builtin_languages = language_registry.available_languages();
                    println!("Built-in languages: {}", builtin_languages.join(", "));
                }
            }
        }
        Commands::Status => {
            let coordinator = UseCaseCoordinator::load()?;
            coordinator.show_status()?;
        }
    }

    Ok(())
}
