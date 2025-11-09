// src/core/models/status.rs
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

    /// Parse status string to Status enum
    /// Parse status string from user input or TOML files
    #[allow(dead_code)]
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
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.display_name())
    }
}
