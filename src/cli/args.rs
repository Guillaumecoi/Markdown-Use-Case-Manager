use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mucm")]
#[command(about = "Markdown Use Case Manager - Manage use cases and scenarios in markdown format")]
#[command(version)]
pub struct Cli {
    /// Enable interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
    /// Enter interactive mode
    Interactive,
}
