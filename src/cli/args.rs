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
pub enum PersonaAction {
    /// Create a new persona
    Create {
        /// Persona name
        name: String,
        /// Persona description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List all personas
    List,
    /// Delete a persona
    Delete {
        /// Persona name to delete
        name: String,
    },
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new use case manager project
    Init {
        /// Programming language for test templates (rust, python, javascript, etc.)
        #[arg(short, long)]
        language: Option<String>,
        /// Documentation methodology (feature, business, developer, tester)
        #[arg(short, long)]
        methodology: Option<String>,
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
        /// Documentation methodology (feature, business, developer, tester)
        #[arg(long)]
        methodology: Option<String>,
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
    /// Manage personas (create, list, delete)
    Persona {
        #[command(subcommand)]
        action: PersonaAction,
    },
    /// List all use cases
    List,
    /// List available programming languages for templates
    Languages,
    /// List available methodologies
    Methodologies,
    /// Show methodology information
    MethodologyInfo {
        /// Methodology name to get info for
        name: String,
    },
    /// Regenerate markdown documentation from TOML files
    /// 
    /// Without arguments, regenerates all use cases with their current methodology.
    /// With a use case ID, regenerates just that use case.
    /// With --methodology, changes the methodology during regeneration.
    Regenerate {
        /// Use case ID (e.g., UC-SEC-001). If omitted, regenerates all use cases.
        use_case_id: Option<String>,
        /// Documentation methodology (feature, business, developer, tester)
        /// If omitted, uses the use case's current methodology
        #[arg(long)]
        methodology: Option<String>,
        /// Regenerate all use cases (explicit flag, same as omitting use_case_id)
        #[arg(long, short)]
        all: bool,
    },
    /// Show project status
    Status,
    /// Enter interactive mode
    Interactive,
}
