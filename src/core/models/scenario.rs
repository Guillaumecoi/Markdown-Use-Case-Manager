// src/core/models/scenario.rs
use super::{Metadata, Status};
use super::scenario_types::{ScenarioType, ScenarioStep};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub title: String,
    pub description: String,
    pub scenario_type: ScenarioType,
    pub tags: Vec<String>,
    pub steps: Vec<ScenarioStep>,
    pub status: Status,
    pub metadata: Metadata,
}

impl Scenario {
    /// Create a new scenario with basic information
    pub fn new(id: String, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            scenario_type: ScenarioType::default(), // Primary by default
            tags: Vec::new(),
            steps: Vec::new(),
            status: Status::Planned,
            metadata: Metadata::new(),
        }
    }
    
    /// Create a new scenario with explicit type and tags
    #[allow(dead_code)] // Used by methodology processors
    pub fn new_with_type(
        id: String, 
        title: String, 
        description: String,
        scenario_type: ScenarioType,
        tags: Vec<String>
    ) -> Self {
        Self {
            id,
            title,
            description,
            scenario_type,
            tags,
            steps: Vec::new(),
            status: Status::Planned,
            metadata: Metadata::new(),
        }
    }
}
