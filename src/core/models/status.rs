// src/core/models/status.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    Planned,
    InProgress,
    Implemented,
    Tested,
    Deployed,
    Deprecated,
}

impl Status {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn priority(&self) -> u8 {
        match self {
            Status::Deprecated => 0, // Always lowest
            Status::Planned => 1,
            Status::InProgress => 2,
            Status::Implemented => 3,
            Status::Tested => 4,
            Status::Deployed => 5,
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
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

    #[allow(clippy::trivially_copy_pass_by_ref)]
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

    /// Compute aggregated status for use case from scenario statuses
    /// Rule: Lowest status across all scenarios, except Planned only shows if everything is Planned
    pub fn aggregate(statuses: &[Status]) -> Status {
        if statuses.is_empty() {
            return Status::Planned;
        }

        // Deprecated always wins
        if statuses.contains(&Status::Deprecated) {
            return Status::Deprecated;
        }

        // Check if all scenarios are planned
        let all_planned = statuses.iter().all(|s| *s == Status::Planned);
        if all_planned {
            return Status::Planned;
        }

        // Otherwise return the lowest non-planned status
        *statuses
            .iter()
            .filter(|s| **s != Status::Planned)
            .min_by_key(|s| s.priority())
            .unwrap_or(&Status::Planned)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.display_name())
    }
}
