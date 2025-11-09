use anyhow::Result;
use inquire::Select;

/// Main menu options for interactive mode
#[derive(Debug, Clone)]
pub enum MainMenuOption {
    CreateUseCase,
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
