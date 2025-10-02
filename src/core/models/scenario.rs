// src/core/models/scenario.rs
use super::{Metadata, Status};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub metadata: Metadata,
}

impl Scenario {
    pub fn new(id: String, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: Status::Planned,
            metadata: Metadata::new(),
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.metadata.touch();
    }
}
