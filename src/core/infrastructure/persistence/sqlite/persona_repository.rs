//! SQLite implementation of PersonaRepository.
//!
//! Stores personas in a SQLite database with full CRUD operations.

use crate::core::domain::{Metadata, Persona, PersonaRepository};
use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// SQLite-backed persona repository.
pub struct SqlitePersonaRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqlitePersonaRepository {
    /// Create a new SQLite persona repository.
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Initialize the personas table in the database.
    /// Uses a flexible schema with JSON for extra fields.
    pub fn initialize(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS personas (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                extra_fields TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // Index for common queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_personas_name 
             ON personas(name COLLATE NOCASE)",
            [],
        )?;

        Ok(())
    }
}

impl PersonaRepository for SqlitePersonaRepository {
    fn save(&self, persona: &Persona) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();

        // Serialize extra fields to JSON
        let extra_json = serde_json::to_string(&persona.extra)?;

        conn.execute(
            "INSERT OR REPLACE INTO personas 
             (id, name, extra_fields, created_at, updated_at)
             VALUES (?1, ?2, ?3, 
                     COALESCE((SELECT created_at FROM personas WHERE id = ?1), ?4), 
                     ?4)",
            params![persona.id, persona.name, extra_json, now],
        )?;

        Ok(())
    }

    fn load_all(&self) -> Result<Vec<Persona>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, extra_fields, created_at, updated_at
             FROM personas
             ORDER BY name",
        )?;

        let personas = stmt
            .query_map([], |row| {
                let extra_json: String = row.get(2).unwrap_or_else(|_| "{}".to_string());
                let extra: HashMap<String, serde_json::Value> =
                    serde_json::from_str(&extra_json).unwrap_or_default();

                let created_str: String = row.get(3)?;
                let updated_str: String = row.get(4)?;

                let created_at = created_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());
                let updated_at = updated_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());

                Ok(Persona {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    metadata: Metadata {
                        created_at,
                        updated_at,
                    },
                    extra,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(personas)
    }

    fn load_by_id(&self, id: &str) -> Result<Option<Persona>> {
        let conn = self.conn.lock().unwrap();
        let persona = conn
            .query_row(
                "SELECT id, name, extra_fields, created_at, updated_at
                 FROM personas
                 WHERE id = ?1",
                params![id],
                |row| {
                    let extra_json: String = row.get(2).unwrap_or_else(|_| "{}".to_string());
                    let extra: HashMap<String, serde_json::Value> =
                        serde_json::from_str(&extra_json).unwrap_or_default();

                    let created_str: String = row.get(3)?;
                    let updated_str: String = row.get(4)?;

                    let created_at = created_str
                        .parse::<chrono::DateTime<chrono::Utc>>()
                        .unwrap_or_else(|_| chrono::Utc::now());
                    let updated_at = updated_str
                        .parse::<chrono::DateTime<chrono::Utc>>()
                        .unwrap_or_else(|_| chrono::Utc::now());

                    Ok(Persona {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        metadata: Metadata {
                            created_at,
                            updated_at,
                        },
                        extra,
                    })
                },
            )
            .optional()?;

        Ok(persona)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM personas WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn exists(&self, id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM personas WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    fn save_markdown(&self, _persona_id: &str, _markdown_content: &str) -> Result<()> {
        // SQLite backend doesn't store markdown files separately
        // Markdown generation is handled by the application layer
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_repo() -> SqlitePersonaRepository {
        let conn = Connection::open_in_memory().unwrap();
        SqlitePersonaRepository::initialize(&conn).unwrap();
        SqlitePersonaRepository::new(Arc::new(Mutex::new(conn)))
    }

    fn create_test_persona() -> Persona {
        Persona::new("test-persona".to_string(), "Test User".to_string())
    }

    #[test]
    fn test_initialize_creates_table() {
        let conn = Connection::open_in_memory().unwrap();
        SqlitePersonaRepository::initialize(&conn).unwrap();

        // Verify table exists
        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='personas'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )
            .unwrap();

        assert!(table_exists);
    }

    #[test]
    fn test_save_and_load_persona() {
        let repo = create_test_repo();
        let persona = create_test_persona();

        repo.save(&persona).unwrap();

        let loaded = repo.load_by_id("test-persona").unwrap();
        assert!(loaded.is_some());

        let loaded_persona = loaded.unwrap();
        assert_eq!(loaded_persona.id, "test-persona");
        assert_eq!(loaded_persona.name, "Test User");
    }

    #[test]
    fn test_load_all_personas() {
        let repo = create_test_repo();

        let persona1 = create_test_persona();
        repo.save(&persona1).unwrap();

        let persona2 = Persona::new("admin-persona".to_string(), "Admin User".to_string());
        repo.save(&persona2).unwrap();

        let personas = repo.load_all().unwrap();
        assert_eq!(personas.len(), 2);
        assert!(personas.iter().any(|p| p.id == "test-persona"));
        assert!(personas.iter().any(|p| p.id == "admin-persona"));
    }

    #[test]
    fn test_load_nonexistent_persona() {
        let repo = create_test_repo();
        let result = repo.load_by_id("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_persona() {
        let repo = create_test_repo();
        let persona = create_test_persona();

        repo.save(&persona).unwrap();
        assert!(repo.exists("test-persona").unwrap());

        repo.delete("test-persona").unwrap();
        assert!(!repo.exists("test-persona").unwrap());
        assert!(repo.load_by_id("test-persona").unwrap().is_none());
    }

    #[test]
    fn test_exists() {
        let repo = create_test_repo();
        assert!(!repo.exists("test-persona").unwrap());

        let persona = create_test_persona();
        repo.save(&persona).unwrap();

        assert!(repo.exists("test-persona").unwrap());
    }

    #[test]
    fn test_update_persona() {
        let repo = create_test_repo();

        let mut persona = create_test_persona();
        repo.save(&persona).unwrap();

        persona.name = "Updated Test User".to_string();
        repo.save(&persona).unwrap();

        let loaded = repo.load_by_id("test-persona").unwrap().unwrap();
        assert_eq!(loaded.name, "Updated Test User");
    }

    #[test]
    fn test_delete_nonexistent_persona() {
        let repo = create_test_repo();
        // Should not error when deleting nonexistent persona
        let result = repo.delete("nonexistent");
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_overwrite() {
        let repo = create_test_repo();

        let persona = create_test_persona();
        repo.save(&persona).unwrap();

        // Save again with same ID should overwrite
        let mut updated = persona.clone();
        updated.name = "Updated Name".to_string();
        repo.save(&updated).unwrap();

        let loaded = repo.load_by_id("test-persona").unwrap().unwrap();
        assert_eq!(loaded.name, "Updated Name");
    }

    #[test]
    fn test_multiple_personas() {
        let repo = create_test_repo();

        for i in 1..=5 {
            let persona = Persona::new(format!("persona-{}", i), format!("User {}", i));
            repo.save(&persona).unwrap();
        }

        let personas = repo.load_all().unwrap();
        assert_eq!(personas.len(), 5);
    }

    #[test]
    fn test_empty_repository() {
        let repo = create_test_repo();
        let personas = repo.load_all().unwrap();
        assert_eq!(personas.len(), 0);
    }

    #[test]
    fn test_save_markdown_noop() {
        let repo = create_test_repo();
        let persona = create_test_persona();
        repo.save(&persona).unwrap();

        // save_markdown should not error (it's a no-op for SQLite)
        let result = repo.save_markdown("test-persona", "# Test");
        assert!(result.is_ok());
    }
}
