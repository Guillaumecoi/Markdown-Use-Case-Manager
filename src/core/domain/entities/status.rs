use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Planned,
    InProgress,
    Implemented,
    Tested,
    Deployed,
    Deprecated,
}

impl Status {
    pub fn emoji(&self) -> &'static str {
        match self {
            Status::Planned => "📋",
            Status::InProgress => "🔄",
            Status::Implemented => "⚡",
            Status::Tested => "✅",
            Status::Deployed => "🚀",
            Status::Deprecated => "⚠️",
        }
    }
    pub fn display_name(&self) -> &'static str {
        match self {
            Status::Planned => "PLANNED",
            Status::InProgress => "IN_PROGRESS",
            Status::Implemented => "IMPLEMENTED",
            Status::Tested => "TESTED",
            Status::Deployed => "DEPLOYED",
            Status::Deprecated => "DEPRECATED",
        }
    }

    /// Parse status from string
    pub fn from_str(status_str: &str) -> Result<Self, String> {
        match status_str.to_lowercase().as_str() {
            "planned" => Ok(Status::Planned),
            "in_progress" => Ok(Status::InProgress),
            "implemented" => Ok(Status::Implemented),
            "tested" => Ok(Status::Tested),
            "deployed" => Ok(Status::Deployed),
            "deprecated" => Ok(Status::Deprecated),
            _ => Err(format!(
                "Invalid status: {}. Valid options: planned, in_progress, implemented, tested, deployed, deprecated",
                status_str
            )),
        }
    }

    /// Check if this status represents a completed state
    pub fn is_complete(&self) -> bool {
        matches!(self, Status::Deployed | Status::Deprecated)
    }

    /// Check if this status represents active development
    pub fn is_in_progress(&self) -> bool {
        matches!(
            self,
            Status::InProgress | Status::Implemented | Status::Tested
        )
    }

    /// Check if transition to target status is valid
    pub fn can_transition_to(&self, target: &Status) -> bool {
        use Status::*;
        match (self, target) {
            // Can always stay in same status
            (a, b) if a == b => true,

            // Forward progression
            (Planned, InProgress) => true,
            (InProgress, Implemented) => true,
            (Implemented, Tested) => true,
            (Tested, Deployed) => true,

            // Can skip ahead
            (Planned, Implemented | Tested | Deployed) => true,
            (InProgress, Tested | Deployed) => true,
            (Implemented, Deployed) => true,

            // Can deprecate from any status
            (_, Deprecated) => true,

            // Can go back to planning
            (_, Planned) => true,

            // Everything else is invalid
            _ => false,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Status enum variants exist and have correct ordering
    #[test]
    fn test_status_enum_variants() {
        let status = Status::Planned;
        assert_eq!(format!("{:?}", status), "Planned");

        let status = Status::InProgress;
        assert_eq!(format!("{:?}", status), "InProgress");

        let status = Status::Implemented;
        assert_eq!(format!("{:?}", status), "Implemented");

        let status = Status::Tested;
        assert_eq!(format!("{:?}", status), "Tested");

        let status = Status::Deployed;
        assert_eq!(format!("{:?}", status), "Deployed");

        let status = Status::Deprecated;
        assert_eq!(format!("{:?}", status), "Deprecated");
    }

    /// Test Status emoji method returns correct emojis
    #[test]
    fn test_status_emoji() {
        assert_eq!(Status::Planned.emoji(), "📋");
        assert_eq!(Status::InProgress.emoji(), "🔄");
        assert_eq!(Status::Implemented.emoji(), "⚡");
        assert_eq!(Status::Tested.emoji(), "✅");
        assert_eq!(Status::Deployed.emoji(), "🚀");
        assert_eq!(Status::Deprecated.emoji(), "⚠️");
    }

    /// Test Status display_name method returns correct names
    #[test]
    fn test_status_display_name() {
        assert_eq!(Status::Planned.display_name(), "PLANNED");
        assert_eq!(Status::InProgress.display_name(), "IN_PROGRESS");
        assert_eq!(Status::Implemented.display_name(), "IMPLEMENTED");
        assert_eq!(Status::Tested.display_name(), "TESTED");
        assert_eq!(Status::Deployed.display_name(), "DEPLOYED");
        assert_eq!(Status::Deprecated.display_name(), "DEPRECATED");
    }

    /// Test Status Display trait implementation
    #[test]
    fn test_status_display() {
        assert_eq!(Status::Planned.to_string(), "📋 PLANNED");
        assert_eq!(Status::InProgress.to_string(), "🔄 IN_PROGRESS");
        assert_eq!(Status::Implemented.to_string(), "⚡ IMPLEMENTED");
        assert_eq!(Status::Tested.to_string(), "✅ TESTED");
        assert_eq!(Status::Deployed.to_string(), "🚀 DEPLOYED");
        assert_eq!(Status::Deprecated.to_string(), "⚠️ DEPRECATED");
    }

    /// Test Status comparison and ordering (based on enum variant order)
    #[test]
    fn test_status_ordering() {
        assert!(Status::Planned < Status::InProgress);
        assert!(Status::InProgress < Status::Implemented);
        assert!(Status::Implemented < Status::Tested);
        assert!(Status::Tested < Status::Deployed);
        assert!(Status::Deployed < Status::Deprecated);
    }

    /// Test Status equality
    #[test]
    fn test_status_equality() {
        assert_eq!(Status::Planned, Status::Planned);
        assert_ne!(Status::Planned, Status::InProgress);
        assert_eq!(Status::Tested, Status::Tested);
    }

    /// Test Status serialization works (for YAML storage)
    #[test]
    fn test_status_serialization() {
        let status = Status::InProgress;

        let serialized = serde_json::to_string(&status).expect("Failed to serialize");
        let deserialized: Status =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(status, deserialized);
    }

    /// Test Status from_str parsing functionality
    #[test]
    fn test_status_from_str() {
        // Test valid status strings
        assert_eq!(Status::from_str("planned").unwrap(), Status::Planned);
        assert_eq!(Status::from_str("in_progress").unwrap(), Status::InProgress);
        assert_eq!(
            Status::from_str("implemented").unwrap(),
            Status::Implemented
        );
        assert_eq!(Status::from_str("tested").unwrap(), Status::Tested);
        assert_eq!(Status::from_str("deployed").unwrap(), Status::Deployed);
        assert_eq!(Status::from_str("deprecated").unwrap(), Status::Deprecated);

        // Test case insensitive parsing
        assert_eq!(Status::from_str("PLANNED").unwrap(), Status::Planned);
        assert_eq!(Status::from_str("In_Progress").unwrap(), Status::InProgress);

        // Test invalid status strings
        assert!(Status::from_str("invalid").is_err());
        assert!(Status::from_str("urgent").is_err());
        assert!(Status::from_str("").is_err());
        assert!(Status::from_str("123").is_err());

        let error = Status::from_str("invalid").unwrap_err();
        assert!(error.contains("Invalid status: invalid"));
    }

    /// Test Status::is_complete() method
    #[test]
    fn test_is_complete() {
        assert!(!Status::Planned.is_complete());
        assert!(!Status::InProgress.is_complete());
        assert!(!Status::Implemented.is_complete());
        assert!(!Status::Tested.is_complete());
        assert!(Status::Deployed.is_complete());
        assert!(Status::Deprecated.is_complete());
    }

    /// Test Status::is_in_progress() method
    #[test]
    fn test_is_in_progress() {
        assert!(!Status::Planned.is_in_progress());
        assert!(Status::InProgress.is_in_progress());
        assert!(Status::Implemented.is_in_progress());
        assert!(Status::Tested.is_in_progress());
        assert!(!Status::Deployed.is_in_progress());
        assert!(!Status::Deprecated.is_in_progress());
    }

    /// Test Status::can_transition_to() method for valid transitions
    #[test]
    fn test_can_transition_to_forward() {
        assert!(Status::Planned.can_transition_to(&Status::InProgress));
        assert!(Status::InProgress.can_transition_to(&Status::Implemented));
        assert!(Status::Implemented.can_transition_to(&Status::Tested));
        assert!(Status::Tested.can_transition_to(&Status::Deployed));
    }

    /// Test Status::can_transition_to() method for skip-ahead transitions
    #[test]
    fn test_can_transition_to_skip() {
        assert!(Status::Planned.can_transition_to(&Status::Deployed));
        assert!(Status::InProgress.can_transition_to(&Status::Tested));
    }

    /// Test Status::can_transition_to() method for deprecation transitions
    #[test]
    fn test_can_transition_to_deprecate() {
        assert!(Status::Planned.can_transition_to(&Status::Deprecated));
        assert!(Status::Deployed.can_transition_to(&Status::Deprecated));
    }

    /// Test Status::can_transition_to() method for invalid transitions
    #[test]
    fn test_can_transition_to_invalid() {
        assert!(!Status::Deployed.can_transition_to(&Status::Implemented));
        assert!(!Status::Tested.can_transition_to(&Status::InProgress));
    }

    /// Test Status::can_transition_to() method for same status (should always be true)
    #[test]
    fn test_can_transition_to_same_status() {
        for &status in &[
            Status::Planned,
            Status::InProgress,
            Status::Implemented,
            Status::Tested,
            Status::Deployed,
            Status::Deprecated,
        ] {
            assert!(status.can_transition_to(&status));
        }
    }

    /// Test Status::can_transition_to() method for backward transitions to Planned
    #[test]
    fn test_can_transition_to_back_to_planned() {
        assert!(Status::InProgress.can_transition_to(&Status::Planned));
        assert!(Status::Implemented.can_transition_to(&Status::Planned));
        assert!(Status::Tested.can_transition_to(&Status::Planned));
        assert!(Status::Deployed.can_transition_to(&Status::Planned));
        assert!(Status::Deprecated.can_transition_to(&Status::Planned));
    }
}
