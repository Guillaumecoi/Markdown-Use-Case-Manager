use super::Metadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Business persona - stakeholder experiencing scenarios
///
/// Based on persona modeling in software engineering (Sommerville et al.),
/// personas represent archetypal users with specific characteristics, goals,
/// and pain points that help drive requirements and design decisions.
///
/// Personas are created with minimal required fields (id and name).
/// Additional fields are determined by the persona configuration in the
/// project's config file and can be filled in by editing the TOML/SQL directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    /// Unique identifier (e.g., "customer", "admin", "guest")
    pub id: String,

    /// Display name (required)
    pub name: String,

    pub metadata: Metadata,

    /// All other persona fields are stored as flexible extra fields
    /// These are determined by the persona configuration in the project config
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Persona {
    /// Create a new persona with minimal required fields
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            metadata: Metadata::new(),
            extra: HashMap::new(),
        }
    }

    /// Create a persona from config fields
    /// This initializes extra fields based on the persona configuration
    pub fn from_config_fields(
        id: String,
        name: String,
        config_fields: &HashMap<String, crate::core::CustomFieldConfig>,
    ) -> Self {
        let mut persona = Self::new(id, name);

        // Initialize extra fields with empty/default values based on config
        for (field_name, field_config) in config_fields {
            let default_value = if let Some(default) = &field_config.default {
                // Use the default value from config if provided
                serde_json::Value::String(default.clone())
            } else {
                // Otherwise initialize with empty/zero values based on type
                match field_config.field_type.as_str() {
                    "string" => serde_json::Value::String(String::new()),
                    "array" => serde_json::Value::Array(vec![]),
                    "number" => serde_json::Value::Number(serde_json::Number::from(0)),
                    "boolean" => serde_json::Value::Bool(false),
                    _ => serde_json::Value::String(String::new()),
                }
            };
            persona.extra.insert(field_name.clone(), default_value);
        }

        persona
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
        let persona = Persona::new("customer".to_string(), "Regular Customer".to_string());

        assert_eq!(persona.id, "customer");
        assert_eq!(persona.name, "Regular Customer");
        assert!(persona.extra.is_empty());
    }

    #[test]
    fn test_persona_emoji() {
        assert_eq!(
            Persona::new("admin".to_string(), "Admin".to_string()).emoji(),
            "👨‍💼"
        );
        assert_eq!(
            Persona::new("customer".to_string(), "Customer".to_string()).emoji(),
            "👤"
        );
        assert_eq!(
            Persona::new("guest".to_string(), "Guest".to_string()).emoji(),
            "🚶"
        );
        assert_eq!(
            Persona::new("developer".to_string(), "Developer".to_string()).emoji(),
            "👨‍💻"
        );
    }

    #[test]
    fn test_persona_with_extra_fields() {
        let mut persona = Persona::new("test".to_string(), "Test User".to_string());

        persona
            .extra
            .insert("department".to_string(), serde_json::json!("Engineering"));
        persona
            .extra
            .insert("experience_level".to_string(), serde_json::json!("Expert"));

        assert_eq!(persona.extra.len(), 2);
        assert_eq!(
            persona.extra.get("department"),
            Some(&serde_json::json!("Engineering"))
        );
    }

    #[test]
    fn test_persona_serialization() {
        let mut persona = Persona::new("test".to_string(), "Test".to_string());
        persona
            .extra
            .insert("role".to_string(), serde_json::json!("Developer"));

        let json = serde_json::to_string(&persona).unwrap();
        let deserialized: Persona = serde_json::from_str(&json).unwrap();

        assert_eq!(persona.id, deserialized.id);
        assert_eq!(persona.name, deserialized.name);
        assert_eq!(persona.extra, deserialized.extra);
    }
}
