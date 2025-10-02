// src/main.rs
use anyhow::Result;
use clap::{Parser, Subcommand};
use markdown_use_case_manager::{config::Config, UseCaseManager};

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
    Init,
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
    /// Show project status
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing use case manager project...");
            let config = Config::init_project()?;
            println!("Project initialized! Configuration saved to .mucm/mucm.toml");
            println!(
                "Use cases will be stored in: {}",
                config.directories.use_case_dir
            );
            println!(
                "Tests will be generated in: {}",
                config.directories.test_dir
            );
        }
        Commands::Create {
            title,
            category,
            description,
        } => {
            let mut manager = UseCaseManager::load()?;
            let use_case_id = manager.create_use_case(title, category, description)?;
            println!("Created use case: {}", use_case_id);
        }
        Commands::AddScenario {
            use_case_id,
            title,
            description,
        } => {
            let mut manager = UseCaseManager::load()?;
            let scenario_id = manager.add_scenario_to_use_case(use_case_id, title, description)?;
            println!("Added scenario: {}", scenario_id);
        }
        Commands::UpdateStatus {
            scenario_id,
            status,
        } => {
            let mut manager = UseCaseManager::load()?;
            manager.update_scenario_status(scenario_id, status)?;
        }
        Commands::List => {
            let manager = UseCaseManager::load()?;
            manager.list_use_cases()?;
        }
        Commands::Status => {
            let manager = UseCaseManager::load()?;
            manager.show_status()?;
        }
    }

    Ok(())
}
