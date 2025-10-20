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

    // Extended metadata fields
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub personas: Vec<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub reviewer: Option<String>,
    #[serde(default)]
    pub business_value: Option<String>,
    #[serde(default)]
    pub complexity: Option<String>,
    #[serde(default)]
    pub epic: Option<String>,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
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
            scenarios: Vec::new(),
            metadata: Metadata::new(),
            prerequisites: Vec::new(),
            personas: Vec::new(),
            author: None,
            reviewer: None,
            business_value: None,
            complexity: None,
            epic: None,
            acceptance_criteria: Vec::new(),
            assumptions: Vec::new(),
            constraints: Vec::new(),
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
