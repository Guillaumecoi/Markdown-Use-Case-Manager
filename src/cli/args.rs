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
    /// Enter interactive mode
    Interactive,
}
