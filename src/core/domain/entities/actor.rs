use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Technical actor that performs actions in scenario steps
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Actor {
    /// End user interacting with the system
    User,
    /// The system being designed
    System,
    /// Backend server/API
    Server,
    /// External third-party API
    ExternalAPI,
    /// Database system
    Database,
    /// Custom actor type
    Custom(String),
}

impl Actor {
    /// Create a custom actor
    pub fn custom(name: impl Into<String>) -> Self {
        Actor::Custom(name.into())
    }

    /// Get the actor name as a string
    pub fn name(&self) -> &str {
        match self {
            Actor::User => "User",
            Actor::System => "System",
            Actor::Server => "Server",
            Actor::ExternalAPI => "ExternalAPI",
            Actor::Database => "Database",
            Actor::Custom(name) => name,
        }
    }

    /// Check if this is a human actor
    pub fn is_human(&self) -> bool {
        matches!(self, Actor::User | Actor::Custom(_))
    }

    /// Check if this is a system actor
    pub fn is_system(&self) -> bool {
        matches!(
            self,
            Actor::System | Actor::Server | Actor::Database | Actor::ExternalAPI
        )
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Actor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Actor::User),
            "system" => Ok(Actor::System),
            "server" => Ok(Actor::Server),
            "externalapi" | "external_api" | "api" => Ok(Actor::ExternalAPI),
            "database" | "db" => Ok(Actor::Database),
            _ => Ok(Actor::Custom(s.to_string())),
        }
    }
}

// Migration helper: Allow converting from String for backward compatibility
impl From<String> for Actor {
    fn from(s: String) -> Self {
        Actor::from_str(&s).unwrap_or_else(|_| Actor::Custom(s))
    }
}

impl From<&str> for Actor {
    fn from(s: &str) -> Self {
        Actor::from_str(s).unwrap_or_else(|_| Actor::Custom(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_variants() {
        assert_eq!(Actor::User.name(), "User");
        assert_eq!(Actor::System.name(), "System");
        assert_eq!(Actor::Server.name(), "Server");
        assert_eq!(Actor::ExternalAPI.name(), "ExternalAPI");
        assert_eq!(Actor::Database.name(), "Database");
        assert_eq!(Actor::custom("Admin").name(), "Admin");
    }

    #[test]
    fn test_is_human() {
        assert!(Actor::User.is_human());
        assert!(Actor::custom("Admin").is_human());
        assert!(!Actor::System.is_human());
        assert!(!Actor::Server.is_human());
        assert!(!Actor::Database.is_human());
    }

    #[test]
    fn test_is_system() {
        assert!(Actor::System.is_system());
        assert!(Actor::Server.is_system());
        assert!(Actor::Database.is_system());
        assert!(Actor::ExternalAPI.is_system());
        assert!(!Actor::User.is_system());
        assert!(!Actor::custom("Admin").is_system());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Actor::from_str("user").unwrap(), Actor::User);
        assert_eq!(Actor::from_str("User").unwrap(), Actor::User);
        assert_eq!(Actor::from_str("SYSTEM").unwrap(), Actor::System);
        assert_eq!(Actor::from_str("server").unwrap(), Actor::Server);
        assert_eq!(Actor::from_str("api").unwrap(), Actor::ExternalAPI);
        assert_eq!(Actor::from_str("database").unwrap(), Actor::Database);
        assert_eq!(Actor::from_str("db").unwrap(), Actor::Database);
        assert_eq!(
            Actor::from_str("CustomActor").unwrap(),
            Actor::Custom("CustomActor".to_string())
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Actor::from("user".to_string()), Actor::User);
        assert_eq!(Actor::from("System".to_string()), Actor::System);
        assert_eq!(
            Actor::from("PaymentGateway".to_string()),
            Actor::Custom("PaymentGateway".to_string())
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Actor::User), "User");
        assert_eq!(format!("{}", Actor::System), "System");
        assert_eq!(format!("{}", Actor::custom("Admin")), "Admin");
    }

    #[test]
    fn test_serialization() {
        let actor = Actor::User;
        let json = serde_json::to_string(&actor).unwrap();
        let deserialized: Actor = serde_json::from_str(&json).unwrap();
        assert_eq!(actor, deserialized);

        let custom = Actor::Custom("Admin".to_string());
        let json = serde_json::to_string(&custom).unwrap();
        let deserialized: Actor = serde_json::from_str(&json).unwrap();
        assert_eq!(custom, deserialized);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        set.insert(Actor::User);
        set.insert(Actor::System);
        set.insert(Actor::User); // Duplicate
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(&Actor::User));
        assert!(set.contains(&Actor::System));
    }
}
