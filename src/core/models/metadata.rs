// src/core/models/metadata.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
}

impl Metadata {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }
    
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
        self.version += 1;
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}