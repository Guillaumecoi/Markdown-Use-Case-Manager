// src/core/models/use_case.rs
use super::{Metadata, Status};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "LOW"),
            Priority::Medium => write!(f, "MEDIUM"),
            Priority::High => write!(f, "HIGH"),
            Priority::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCase {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub priority: Priority,
    pub metadata: Metadata,

    // Catch-all for any additional fields from TOML (including business_value,
    // acceptance_criteria, prerequisites, etc.) - fully flexible!
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UseCase {
    pub fn new(
        id: String,
        title: String,
        category: String,
        description: String,
        priority: Priority,
    ) -> Self {
        Self {
            id,
            title,
            category,
            description,
            priority,
            metadata: Metadata::new(),
            extra: std::collections::HashMap::new(),
        }
    }

    pub fn status(&self) -> Status {
        Status::Planned
    }
}
