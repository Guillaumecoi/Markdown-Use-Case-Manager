use crate::cli::runner::CliRunner;
use crate::config::Config;
use anyhow::Result;
use inquire::{Confirm, Select, Text};

#[derive(Debug, Default)]
pub struct ExtendedMetadata {
    pub personas: Vec<String>,
    pub prerequisites: Vec<String>,
    pub author: Option<String>,
    pub reviewer: Option<String>,
    pub business_value: Option<String>,
    pub complexity: Option<String>,
    pub epic: Option<String>,
    pub acceptance_criteria: Vec<String>,
    pub assumptions: Vec<String>,
    pub constraints: Vec<String>,
}

/// Main menu options for interactive mode
#[derive(Debug, Clone)]
pub enum MainMenuOption {
    CreateUseCase,
    AddScenario,
    UpdateScenarioStatus,
    AddExtendedMetadata,
    ConfigureSettings,
    ListUseCases,
    ShowStatus,
    ShowLanguages,
    Exit,
}

impl std::fmt::Display for MainMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuOption::CreateUseCase => write!(f, "📝 Create a new use case"),
            MainMenuOption::AddScenario => write!(f, "➕ Add scenario to existing use case"),
            MainMenuOption::UpdateScenarioStatus => write!(f, "🔄 Update scenario status"),
            MainMenuOption::AddExtendedMetadata => write!(f, "📋 Add extended metadata to use case"),
            MainMenuOption::ConfigureSettings => write!(f, "⚙️  Configure settings"),
            MainMenuOption::ListUseCases => write!(f, "📋 List all use cases"),
            MainMenuOption::ShowStatus => write!(f, "📊 Show project status"),
            MainMenuOption::ShowLanguages => write!(f, "🗣️  Show available languages"),
            MainMenuOption::Exit => write!(f, "🚪 Exit"),
        }
    }
}

/// Show the main menu and return the selected option
pub fn show_main_menu() -> Result<MainMenuOption> {
    let options = vec![
        MainMenuOption::CreateUseCase,
        MainMenuOption::AddScenario,
        MainMenuOption::UpdateScenarioStatus,
        MainMenuOption::AddExtendedMetadata,
        MainMenuOption::ConfigureSettings,
        MainMenuOption::ListUseCases,
        MainMenuOption::ShowStatus,
        MainMenuOption::ShowLanguages,
        MainMenuOption::Exit,
    ];

    let selection = Select::new("What would you like to do?", options)
        .with_page_size(10)
        .prompt()?;

    Ok(selection)
}

/// Guided workflow for creating a use case
pub fn guided_create_use_case(runner: &mut CliRunner) -> Result<()> {
    println!("\n🔧 Creating a new use case...\n");

    // Get title
    let title = Text::new("Enter the use case title:")
        .with_help_message("e.g., 'User Login', 'File Upload', 'Data Export'")
        .prompt()?;

    // Get existing categories for suggestions
    let existing_categories = runner.get_categories().unwrap_or_default();

    let category = if existing_categories.is_empty() {
        Text::new("Enter the category:")
            .with_help_message("e.g., 'auth', 'api', 'security', 'profile'")
            .prompt()?
    } else {
        // Allow selection from existing or entering new
        let mut options = existing_categories.clone();
        options.push("✏️  Enter a new category".to_string());

        let selection = Select::new("Select a category or create a new one:", options).prompt()?;

        if selection == "✏️  Enter a new category" {
            Text::new("Enter the new category:")
                .with_help_message("e.g., 'auth', 'api', 'security', 'profile'")
                .prompt()?
        } else {
            selection
        }
    };

    // Get optional description
    let add_description = Confirm::new("Would you like to add a description?")
        .with_default(false)
        .prompt()?;

    let description = if add_description {
        Some(
            Text::new("Enter the description:")
                .with_help_message("Brief description of what this use case covers")
                .prompt()?,
        )
    } else {
        None
    };

    // Ask if they want to add extended metadata
    let add_extended_metadata = Confirm::new("Would you like to add extended metadata (personas, prerequisites, etc.)?")
        .with_default(false)
        .prompt()?;

    let mut extended_metadata = ExtendedMetadata::default();
    if add_extended_metadata {
        extended_metadata = collect_extended_metadata()?;
    }

    // Create the use case
    let result = runner.create_use_case_with_metadata(title, category, description, extended_metadata)?;
    println!("\n✅ {}", result);

    // Ask if they want to add scenarios immediately
    let add_scenarios = Confirm::new("Would you like to add scenarios to this use case now?")
        .with_default(true)
        .prompt()?;

    if add_scenarios {
        // Extract the use case ID from the result message
        if let Some(use_case_id) = extract_use_case_id(&result) {
            guided_add_scenarios_to_use_case(runner, &use_case_id)?;
        }
    }

    Ok(())
}

/// Guided workflow for adding scenarios to a use case
pub fn guided_add_scenario(runner: &mut CliRunner) -> Result<()> {
    println!("\n➕ Adding a scenario to an existing use case...\n");

    // Get all use case IDs
    let use_case_ids = runner.get_use_case_ids()?;

    if use_case_ids.is_empty() {
        println!("❌ No use cases found. Create a use case first!");
        return Ok(());
    }

    // Select use case
    let use_case_id = Select::new("Select a use case:", use_case_ids)
        .with_page_size(10)
        .prompt()?;

    guided_add_scenarios_to_use_case(runner, &use_case_id)?;

    Ok(())
}

/// Helper function to add multiple scenarios to a specific use case
fn guided_add_scenarios_to_use_case(runner: &mut CliRunner, use_case_id: &str) -> Result<()> {
    loop {
        // Get scenario title
        let title = Text::new(&format!("Enter scenario title for {}:", use_case_id))
            .with_help_message("e.g., 'Happy path login', 'Invalid credentials', 'Account locked'")
            .prompt()?;

        // Get optional description
        let add_description =
            Confirm::new("Would you like to add a description for this scenario?")
                .with_default(false)
                .prompt()?;

        let description = if add_description {
            Some(Text::new("Enter the scenario description:").prompt()?)
        } else {
            None
        };

        // Add the scenario
        let result = runner.add_scenario(use_case_id.to_string(), title, description)?;
        println!("✅ {}", result);

        // Ask if they want to add more scenarios
        let add_more = Confirm::new("Would you like to add another scenario to this use case?")
            .with_default(false)
            .prompt()?;

        if !add_more {
            break;
        }
    }

    Ok(())
}

/// Guided workflow for updating scenario status
pub fn guided_update_scenario_status(runner: &mut CliRunner) -> Result<()> {
    println!("\n🔄 Updating scenario status...\n");

    // Get all use case IDs
    let use_case_ids = runner.get_use_case_ids()?;

    if use_case_ids.is_empty() {
        println!("❌ No use cases found. Create a use case first!");
        return Ok(());
    }

    // Select use case
    let use_case_id = Select::new("Select a use case:", use_case_ids)
        .with_page_size(10)
        .prompt()?;

    // Get scenarios for this use case
    let scenario_ids = runner.get_scenario_ids(&use_case_id)?;

    if scenario_ids.is_empty() {
        println!("❌ No scenarios found for this use case. Add scenarios first!");
        return Ok(());
    }

    // Select scenario
    let scenario_id = Select::new("Select a scenario:", scenario_ids)
        .with_page_size(10)
        .prompt()?;

    // Select new status
    let statuses = vec![
        "planned",
        "in_progress",
        "implemented",
        "tested",
        "deployed",
        "deprecated",
    ];

    let status = Select::new("Select new status:", statuses).prompt()?;

    // Update the status
    let result = runner.update_scenario_status(scenario_id, status.to_string())?;
    println!("✅ {}", result);

    Ok(())
}

/// Extract use case ID from result message
fn extract_use_case_id(result: &str) -> Option<String> {
    // Look for pattern "Created use case: UC-XXX-001"
    if let Some(pos) = result.find("UC-") {
        let id_part = &result[pos..];
        if let Some(end) = id_part.find(char::is_whitespace) {
            Some(id_part[..end].to_string())
        } else {
            Some(id_part.to_string())
        }
    } else {
        None
    }
}

/// Collect extended metadata through interactive prompts
fn collect_extended_metadata() -> Result<ExtendedMetadata> {
    println!("\n📋 Extended Metadata Collection\n");
    
    let mut metadata = ExtendedMetadata::default();
    
    // Load config to check which fields are enabled
    let config = Config::load()?;
    let metadata_config = &config.metadata;

    // Only show fields that are enabled in config
    if metadata_config.include_personas {
        let add_personas = Confirm::new("Add personas (target users)?")
            .with_default(false)
            .prompt()?;
        
        if add_personas {
            metadata.personas = collect_list_items("persona", "e.g., 'Admin User', 'Customer', 'Support Agent'")?;
        }
    }

    if metadata_config.include_prerequisites {
        let add_prerequisites = Confirm::new("Add prerequisites?")
            .with_default(false)
            .prompt()?;
        
        if add_prerequisites {
            metadata.prerequisites = collect_list_items("prerequisite", "e.g., 'User must be logged in (UC-AUTH-001)', 'Valid payment method required'")?;
        }
    }

    // Single-value fields with config check
    let single_fields = vec![
        ("author", "Author name", "Who is the author of this use case?", metadata_config.include_author),
        ("reviewer", "Reviewer name", "Who should review this use case?", metadata_config.include_reviewer),
        ("business_value", "Business value", "What business value does this provide?", metadata_config.include_business_value),
        ("complexity", "Complexity level", "e.g., 'Low', 'Medium', 'High'", metadata_config.include_complexity),
        ("epic", "Epic name/ID", "Which epic does this belong to?", metadata_config.include_epic),
    ];

    for (field, label, help, enabled) in single_fields {
        if enabled {
            let add_field = Confirm::new(&format!("Add {}?", label.to_lowercase()))
                .with_default(false)
                .prompt()?;
            
            if add_field {
                let value = Text::new(&format!("Enter {}:", label.to_lowercase()))
                    .with_help_message(help)
                    .prompt()?;
                
                match field {
                    "author" => metadata.author = Some(value),
                    "reviewer" => metadata.reviewer = Some(value),
                    "business_value" => metadata.business_value = Some(value),
                    "complexity" => metadata.complexity = Some(value),
                    "epic" => metadata.epic = Some(value),
                    _ => {}
                }
            }
        }
    }

    // List fields with config check
    let list_fields = vec![
        ("acceptance_criteria", "acceptance criteria", "e.g., 'System validates input', 'User receives confirmation'", metadata_config.include_acceptance_criteria),
        ("assumptions", "assumptions", "e.g., 'User has internet connection', 'Database is available'", metadata_config.include_assumptions),
        ("constraints", "constraints", "e.g., 'Must complete within 30 seconds', 'Mobile-friendly interface'", metadata_config.include_constraints),
    ];

    for (field, label, help, enabled) in list_fields {
        if enabled {
            let add_field = Confirm::new(&format!("Add {}?", label))
                .with_default(false)
                .prompt()?;
            
            if add_field {
                let items = collect_list_items(label, help)?;
                match field {
                    "acceptance_criteria" => metadata.acceptance_criteria = items,
                    "assumptions" => metadata.assumptions = items,
                    "constraints" => metadata.constraints = items,
                    _ => {}
                }
            }
        }
    }

    Ok(metadata)
}

/// Helper function to collect a list of items
fn collect_list_items(item_type: &str, help: &str) -> Result<Vec<String>> {
    let mut items = Vec::new();
    
    // Enhanced help text for prerequisites to suggest use case references
    let enhanced_help = if item_type == "prerequisite" {
        format!("{}\nTip: Reference other use cases like 'User must be logged in (UC-AUTH-001)'", help)
    } else {
        help.to_string()
    };
    
    loop {
        let item = Text::new(&format!("Enter {} (or press Enter to finish):", item_type))
            .with_help_message(&enhanced_help)
            .prompt()?;
        
        if item.trim().is_empty() {
            break;
        }
        
        items.push(item);
        
        if !Confirm::new(&format!("Add another {}?", item_type))
            .with_default(true)
            .prompt()? {
            break;
        }
    }
    
    Ok(items)
}

/// Guided workflow for adding extended metadata to an existing use case
pub fn guided_add_extended_metadata(runner: &mut CliRunner) -> Result<()> {
    println!("\n📋 Adding extended metadata to existing use case...\n");

    // Get list of existing use cases
    let use_case_ids = runner.get_use_case_ids()?;
    if use_case_ids.is_empty() {
        println!("❌ No use cases found. Create a use case first.");
        return Ok(());
    }

    // Select use case
    let selected_id = Select::new("Select a use case to add metadata to:", use_case_ids).prompt()?;

    // Collect extended metadata
    let extended_metadata = collect_extended_metadata()?;

    // Update the use case
    let result = runner.update_use_case_metadata(selected_id, extended_metadata)?;
    println!("\n✅ {}", result);

    Ok(())
}
