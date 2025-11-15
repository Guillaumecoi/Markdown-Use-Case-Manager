use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mucm")]
#[command(version)]
pub struct Cli {
    /// Enable interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new use case manager project
    Init {
        /// Programming language for test templates (rust, python, javascript, etc.)
        #[arg(short, long)]
        language: Option<String>,
        /// Documentation methodology (feature, business, developer, tester)
        #[arg(short, long)]
        methodology: Option<String>,
        /// Storage backend (toml or sqlite)
        #[arg(long, short = 's', default_value = "toml")]
        storage: String,
        /// Finalize initialization by copying templates (run after reviewing config)
        #[arg(long)]
        finalize: bool,
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
    /// Manage use case preconditions
    Precondition {
        #[command(subcommand)]
        command: PreconditionCommands,
    },
    /// Manage use case postconditions
    Postcondition {
        #[command(subcommand)]
        command: PostconditionCommands,
    },
    /// Manage use case references
    Reference {
        #[command(subcommand)]
        command: ReferenceCommands,
    },
    /// Manage use case scenarios
    Scenario {
        #[command(subcommand)]
        command: ScenarioCommands,
    },
    /// Manage personas
    Persona {
        #[command(subcommand)]
        command: PersonaCommands,
    },
    /// Enter interactive mode
    Interactive,
}

#[derive(Debug, Subcommand)]
pub enum PreconditionCommands {
    /// Add a precondition to a use case
    Add {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Precondition text
        precondition: String,
    },
    /// List preconditions for a use case
    List {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
    },
    /// Remove a precondition from a use case
    Remove {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Precondition index (1-based)
        index: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum PostconditionCommands {
    /// Add a postcondition to a use case
    Add {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Postcondition text
        postcondition: String,
    },
    /// List postconditions for a use case
    List {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
    },
    /// Remove a postcondition from a use case
    Remove {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Postcondition index (1-based)
        index: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReferenceCommands {
    /// Add a reference to a use case
    Add {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Target use case ID
        target_id: String,
        /// Relationship type (dependency, extension, inclusion, alternative)
        relationship: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List references for a use case
    List {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
    },
    /// Remove a reference from a use case
    Remove {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Target use case ID to remove
        target_id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ScenarioCommands {
    /// Add a scenario to a use case
    Add {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Scenario title
        title: String,
        /// Scenario type (main, alternative, exception)
        #[arg(short, long)]
        scenario_type: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Add a step to a scenario
    AddStep {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Scenario title
        scenario_title: String,
        /// Step description
        step: String,
        /// Step order (1-based, optional - will be appended if not specified)
        #[arg(short, long)]
        order: Option<u32>,
    },
    /// Update scenario status
    UpdateStatus {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Scenario title
        scenario_title: String,
        /// New status (planned, in-progress, completed, deprecated)
        status: String,
    },
    /// List scenarios for a use case
    List {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
    },
    /// Remove a step from a scenario
    RemoveStep {
        /// Use case ID (e.g., UC-SEC-001)
        use_case_id: String,
        /// Scenario title
        scenario_title: String,
        /// Step order to remove (1-based)
        order: u32,
    },
    /// Manage scenario references
    #[command(subcommand)]
    Reference(ScenarioReferenceCommands),
}

#[derive(Debug, Subcommand)]
pub enum ScenarioReferenceCommands {
    /// Add a reference from one scenario to another scenario or use case
    Add {
        /// Use case ID containing the source scenario
        use_case_id: String,
        /// Source scenario title
        scenario_title: String,
        /// Target ID (scenario or use case)
        target_id: String,
        /// Reference type: "scenario" or "usecase"
        #[arg(short = 't', long)]
        ref_type: String,
        /// Relationship: "includes", "extends", "depends-on", "alternative-to"
        #[arg(short, long)]
        relationship: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove a reference from a scenario
    Remove {
        /// Use case ID containing the scenario
        use_case_id: String,
        /// Scenario title
        scenario_title: String,
        /// Target ID to remove
        target_id: String,
        /// Relationship type
        #[arg(short, long)]
        relationship: String,
    },
    /// List all references for a scenario
    List {
        /// Use case ID containing the scenario
        use_case_id: String,
        /// Scenario title
        scenario_title: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum PersonaCommands {
    /// Create a new persona
    Create {
        /// Persona ID (e.g., "admin", "customer")
        id: String,
        /// Persona name
        name: String,
        /// Persona description
        description: String,
        /// Primary goal
        goal: String,
        /// Context/background information
        #[arg(long)]
        context: Option<String>,
        /// Technical proficiency level (1-5)
        #[arg(long)]
        tech_level: Option<u8>,
        /// Frequency of system use (e.g., "daily", "weekly", "occasional")
        #[arg(long)]
        usage_frequency: Option<String>,
    },
    /// List all personas
    List,
    /// Show persona details
    Show {
        /// Persona ID
        id: String,
    },
    /// Delete a persona
    Delete {
        /// Persona ID
        id: String,
    },
}
