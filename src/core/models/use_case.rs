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
    pub metadata: Metadata,

    // Optional fields (less commonly used)
    #[serde(default)]
    pub prerequisites: Vec<String>,
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
            metadata: Metadata::new(),
            prerequisites: Vec::new(),
        }
    }

    pub fn status(&self) -> Status {
        let scenario_statuses: Vec<Status> = self.scenarios.iter().map(|s| s.status).collect();
        Status::aggregate(&scenario_statuses)
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
}
