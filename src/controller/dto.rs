/// Data Transfer Objects for passing data between Controller and View layers

/// Result of an operation to display to the user
#[derive(Debug)]
pub struct DisplayResult {
    /// Whether the operation succeeded
    /// TODO: Use this field to conditionally format output (green for success, red for error)
    #[allow(dead_code)]
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

/// Available options for user selection
#[derive(Debug, Clone)]
pub struct SelectionOptions {
    pub items: Vec<String>,
}

impl SelectionOptions {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    /// Check if there are no items available
    /// TODO: Use this to show helpful messages when no use cases/categories exist yet
    #[allow(dead_code)]
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
    /// Format methodology info as a single line for list display
    /// TODO: Use this in `mucm methodologies` command for cleaner output
    #[allow(dead_code)]
    pub fn to_display_string(&self) -> String {
        format!("{} - {}", self.display_name, self.description)
    }
}
