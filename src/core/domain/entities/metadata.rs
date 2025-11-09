// src/core/models/metadata.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Metadata {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the updated_at timestamp to current time
    /// TODO: Call this when use case content is modified (scenarios added, status changed, etc.)
    #[allow(dead_code)]
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}
