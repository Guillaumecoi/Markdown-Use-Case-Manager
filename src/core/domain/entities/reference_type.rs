use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Type of reference relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    /// Reference to another use case
    UseCase,
    /// Reference to a specific scenario
    Scenario,
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReferenceType::UseCase => write!(f, "use_case"),
            ReferenceType::Scenario => write!(f, "scenario"),
        }
    }
}

impl FromStr for ReferenceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "use_case" | "usecase" | "uc" => Ok(ReferenceType::UseCase),
            "scenario" | "s" => Ok(ReferenceType::Scenario),
            _ => Err(format!("Invalid reference type: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_type_display() {
        assert_eq!(ReferenceType::UseCase.to_string(), "use_case");
        assert_eq!(ReferenceType::Scenario.to_string(), "scenario");
    }

    #[test]
    fn test_reference_type_from_str() {
        assert_eq!(
            ReferenceType::from_str("use_case").unwrap(),
            ReferenceType::UseCase
        );
        assert_eq!(
            ReferenceType::from_str("UseCase").unwrap(),
            ReferenceType::UseCase
        );
        assert_eq!(
            ReferenceType::from_str("usecase").unwrap(),
            ReferenceType::UseCase
        );
        assert_eq!(
            ReferenceType::from_str("uc").unwrap(),
            ReferenceType::UseCase
        );

        assert_eq!(
            ReferenceType::from_str("scenario").unwrap(),
            ReferenceType::Scenario
        );
        assert_eq!(
            ReferenceType::from_str("Scenario").unwrap(),
            ReferenceType::Scenario
        );
        assert_eq!(
            ReferenceType::from_str("s").unwrap(),
            ReferenceType::Scenario
        );

        assert!(ReferenceType::from_str("invalid").is_err());
    }

    #[test]
    fn test_reference_type_serialization() {
        let uc_type = ReferenceType::UseCase;
        let json = serde_json::to_string(&uc_type).unwrap();
        assert_eq!(json, r#""use_case""#);

        let deserialized: ReferenceType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ReferenceType::UseCase);
    }
}
