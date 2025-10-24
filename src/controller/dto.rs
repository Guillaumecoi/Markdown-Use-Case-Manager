/// Data Transfer Objects for passing data between Controller and View layers

/// Result of an operation to display to the user
#[derive(Debug)]
pub struct DisplayResult {
    pub success: bool,
    pub message: String,
}

impl DisplayResult {
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}

/// Configuration summary for display
#[derive(Debug, Clone)]
pub struct ConfigSummary {
    pub project_name: String,
    pub project_description: String,
    pub use_case_dir: String,
    pub test_dir: String,
    pub test_language: String,
    pub default_methodology: String,
    pub auto_generate_tests: bool,
}

/// Available options for user selection
#[derive(Debug, Clone)]
pub struct SelectionOptions {
    pub items: Vec<String>,
}

impl SelectionOptions {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Methodology information for display
#[derive(Debug, Clone)]
pub struct MethodologyInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
}

impl MethodologyInfo {
    pub fn to_display_string(&self) -> String {
        format!("{} - {}", self.display_name, self.description)
    }
}
