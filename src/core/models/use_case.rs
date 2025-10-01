// src/core/models/use_case.rs
use super::{Metadata, Scenario, Status};
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
    pub scenarios: Vec<Scenario>,
    pub test_file: Option<String>,
    pub prerequisites: Vec<String>, // Changed from related_use_cases
    pub tags: Vec<String>,
    pub metadata: Metadata,

    /// Explicit status override (if None, computed from scenarios)
    pub explicit_status: Option<Status>,
}

impl UseCase {
    pub fn new(id: String, title: String, category: String, description: String) -> Self {
        Self {
            id,
            title,
            category,
            description,
            priority: Priority::Medium,
            scenarios: Vec::new(),
            test_file: None,
            prerequisites: Vec::new(),
            tags: Vec::new(),
            metadata: Metadata::new(),
            explicit_status: None,
        }
    }

    pub fn status(&self) -> Status {
        self.explicit_status.unwrap_or_else(|| {
            let scenario_statuses: Vec<Status> = self.scenarios.iter().map(|s| s.status).collect();
            Status::aggregate(&scenario_statuses)
        })
    }

    pub fn set_explicit_status(&mut self, status: Option<Status>) {
        self.explicit_status = status;
        self.metadata.touch();
    }

    pub fn add_scenario(&mut self, scenario: Scenario) {
        self.scenarios.push(scenario);
        self.metadata.touch();
    }

    pub fn update_scenario_status(&mut self, scenario_id: &str, status: Status) -> bool {
        if let Some(scenario) = self.scenarios.iter_mut().find(|s| s.id == scenario_id) {
            scenario.status = status;
            scenario.metadata.touch();
            self.metadata.touch();
            true
        } else {
            false
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.metadata.touch();
        }
    }
}
