use super::Metadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Business persona - stakeholder experiencing scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    /// Unique identifier (e.g., "customer", "admin", "guest")
    pub id: String,

    /// Display name
    pub name: String,

    /// Persona description
    pub description: String,

    /// Primary goal of this persona
    pub goal: String,

    /// Context/background information
    #[serde(default)]
    pub context: Option<String>,

    /// Technical proficiency level (1-5)
    #[serde(default)]
    pub tech_level: Option<u8>,

    /// Frequency of system use
    #[serde(default)]
    pub usage_frequency: Option<String>, // "daily", "weekly", "occasional"

    pub metadata: Metadata,

    /// Flexible extra fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Persona {
    pub fn new(id: String, name: String, description: String, goal: String) -> Self {
        Self {
            id,
            name,
            description,
            goal,
            context: None,
            tech_level: None,
            usage_frequency: None,
            metadata: Metadata::new(),
            extra: HashMap::new(),
        }
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_tech_level(mut self, level: u8) -> Self {
        self.tech_level = Some(level.min(5));
        self
    }

    pub fn with_usage_frequency(mut self, frequency: String) -> Self {
        self.usage_frequency = Some(frequency);
        self
    }

    /// Get emoji representation based on persona characteristics
    pub fn emoji(&self) -> &str {
        // Based on persona ID or role
        match self.id.to_lowercase().as_str() {
            id if id.contains("admin") || id.contains("manager") => "👨‍💼",
            id if id.contains("customer") || id.contains("buyer") => "👤",
            id if id.contains("guest") || id.contains("visitor") => "🚶",
            id if id.contains("developer") || id.contains("engineer") => "👨‍💻",
            id if id.contains("support") || id.contains("agent") => "🎧",
            id if id.contains("analyst") || id.contains("data") => "📊",
            _ => "🙂",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_creation() {
        let persona = Persona::new(
            "customer".to_string(),
            "Regular Customer".to_string(),
            "A returning customer with an account".to_string(),
            "Purchase products quickly and easily".to_string(),
        );

        assert_eq!(persona.id, "customer");
        assert_eq!(persona.name, "Regular Customer");
        assert_eq!(persona.goal, "Purchase products quickly and easily");
        assert!(persona.context.is_none());
        assert!(persona.tech_level.is_none());
        assert!(persona.usage_frequency.is_none());
    }

    #[test]
    fn test_persona_with_optional_fields() {
        let persona = Persona::new(
            "power_user".to_string(),
            "Power User".to_string(),
            "Advanced user".to_string(),
            "Efficient workflows".to_string(),
        )
        .with_tech_level(5)
        .with_usage_frequency("daily".to_string())
        .with_context("Professional using the system extensively".to_string());

        assert_eq!(persona.tech_level, Some(5));
        assert_eq!(persona.usage_frequency, Some("daily".to_string()));
        assert!(persona.context.is_some());
        assert_eq!(
            persona.context.unwrap(),
            "Professional using the system extensively"
        );
    }

    #[test]
    fn test_tech_level_capped_at_5() {
        let persona = Persona::new(
            "expert".to_string(),
            "Expert".to_string(),
            "Expert user".to_string(),
            "Master the system".to_string(),
        )
        .with_tech_level(10); // Should be capped to 5

        assert_eq!(persona.tech_level, Some(5));
    }

    #[test]
    fn test_persona_emoji() {
        let admin = Persona::new(
            "admin".to_string(),
            "Admin".to_string(),
            "System administrator".to_string(),
            "Manage system".to_string(),
        );
        assert_eq!(admin.emoji(), "👨‍💼");

        let customer = Persona::new(
            "customer".to_string(),
            "Customer".to_string(),
            "Regular customer".to_string(),
            "Purchase items".to_string(),
        );
        assert_eq!(customer.emoji(), "👤");

        let guest = Persona::new(
            "guest".to_string(),
            "Guest".to_string(),
            "Guest user".to_string(),
            "Browse content".to_string(),
        );
        assert_eq!(guest.emoji(), "🚶");

        let developer = Persona::new(
            "developer".to_string(),
            "Developer".to_string(),
            "Software developer".to_string(),
            "Build integrations".to_string(),
        );
        assert_eq!(developer.emoji(), "👨‍💻");

        let generic = Persona::new(
            "random_user".to_string(),
            "Random".to_string(),
            "Random user".to_string(),
            "Use system".to_string(),
        );
        assert_eq!(generic.emoji(), "🙂");
    }

    #[test]
    fn test_persona_serialization() {
        let persona = Persona::new(
            "test".to_string(),
            "Test".to_string(),
            "Test persona".to_string(),
            "Test goal".to_string(),
        )
        .with_tech_level(3)
        .with_usage_frequency("weekly".to_string());

        let json = serde_json::to_string(&persona).unwrap();
        let deserialized: Persona = serde_json::from_str(&json).unwrap();

        assert_eq!(persona.id, deserialized.id);
        assert_eq!(persona.name, deserialized.name);
        assert_eq!(persona.tech_level, deserialized.tech_level);
        assert_eq!(persona.usage_frequency, deserialized.usage_frequency);
    }

    #[test]
    fn test_persona_with_extra_fields() {
        let mut persona = Persona::new(
            "complex".to_string(),
            "Complex".to_string(),
            "Complex persona".to_string(),
            "Complex goal".to_string(),
        );

        persona.extra.insert(
            "pain_points".to_string(),
            serde_json::json!(["Slow loading", "Complex UI"]),
        );
        persona.extra.insert(
            "motivations".to_string(),
            serde_json::json!(["Save time", "Increase efficiency"]),
        );

        assert_eq!(persona.extra.len(), 2);
        assert!(persona.extra.contains_key("pain_points"));
        assert!(persona.extra.contains_key("motivations"));
    }
}
