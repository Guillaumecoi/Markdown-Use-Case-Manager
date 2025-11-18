use serde::{Deserialize, Serialize};

/// Represents a specific methodology/level combination enabled for a use case.
/// Each view generates a separate markdown file.
///
/// Example:
/// - methodology: "feature", level: "simple", enabled: true
///   → generates UC-001-feat-s.md
/// - methodology: "business", level: "normal", enabled: false
///   → view exists but output generation is skipped
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MethodologyView {
    /// The methodology name (e.g., "feature", "business", "tester")
    pub methodology: String,

    /// The documentation level (e.g., "simple", "normal", "detailed")
    pub level: String,

    /// Whether this view is currently active for output generation
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl MethodologyView {
    /// Create a new methodology view
    pub fn new(methodology: impl Into<String>, level: impl Into<String>) -> Self {
        Self {
            methodology: methodology.into(),
            level: level.into(),
            enabled: true,
        }
    }

    /// Create a new disabled methodology view
    pub fn new_disabled(methodology: impl Into<String>, level: impl Into<String>) -> Self {
        Self {
            methodology: methodology.into(),
            level: level.into(),
            enabled: false,
        }
    }

    /// Get a unique key for this view (methodology-level)
    pub fn key(&self) -> String {
        format!("{}-{}", self.methodology, self.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methodology_view_new() {
        let view = MethodologyView::new("feature", "simple");
        assert_eq!(view.methodology, "feature");
        assert_eq!(view.level, "simple");
        assert!(view.enabled);
    }

    #[test]
    fn test_methodology_view_new_disabled() {
        let view = MethodologyView::new_disabled("business", "detailed");
        assert_eq!(view.methodology, "business");
        assert_eq!(view.level, "detailed");
        assert!(!view.enabled);
    }

    #[test]
    fn test_methodology_view_key() {
        let view = MethodologyView::new("tester", "normal");
        assert_eq!(view.key(), "tester-normal");
    }

    #[test]
    fn test_methodology_view_serialization() {
        let view = MethodologyView::new("feature", "simple");
        let toml = toml::to_string(&view).unwrap();
        assert!(toml.contains("methodology = \"feature\""));
        assert!(toml.contains("level = \"simple\""));
        assert!(toml.contains("enabled = true"));
    }

    #[test]
    fn test_methodology_view_deserialization() {
        let toml = r#"
            methodology = "business"
            level = "detailed"
            enabled = false
        "#;
        let view: MethodologyView = toml::from_str(toml).unwrap();
        assert_eq!(view.methodology, "business");
        assert_eq!(view.level, "detailed");
        assert!(!view.enabled);
    }

    #[test]
    fn test_methodology_view_default_enabled() {
        let toml = r#"
            methodology = "feature"
            level = "normal"
        "#;
        let view: MethodologyView = toml::from_str(toml).unwrap();
        assert!(view.enabled); // Should default to true
    }
}
