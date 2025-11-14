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
    pub fn initialize(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS personas (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                goal TEXT NOT NULL,
                context TEXT,
                tech_level INTEGER,
                usage_frequency TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // Indexes for common queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_personas_name 
             ON personas(name COLLATE NOCASE)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_personas_tech_level 
             ON personas(tech_level)",
            [],
        )?;

        Ok(())
    }
}

impl PersonaRepository for SqlitePersonaRepository {
    fn save(&self, persona: &Persona) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO personas 
             (id, name, description, goal, context, tech_level, usage_frequency, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 
                     COALESCE((SELECT created_at FROM personas WHERE id = ?1), ?8), 
                     ?8)",
            params![
                persona.id,
                persona.name,
                persona.description,
                persona.goal,
                persona.context,
                persona.tech_level,
                persona.usage_frequency,
                now,
            ],
        )?;

        Ok(())
    }

    fn load_all(&self) -> Result<Vec<Persona>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, goal, context, tech_level, usage_frequency
             FROM personas
             ORDER BY name",
        )?;

        let personas = stmt
            .query_map([], |row| {
                Ok(Persona {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    goal: row.get(3)?,
                    context: row.get(4)?,
                    tech_level: row.get(5)?,
                    usage_frequency: row.get(6)?,
                    metadata: Metadata::new(),
                    extra: HashMap::new(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(personas)
    }

    fn load_by_id(&self, id: &str) -> Result<Option<Persona>> {
        let conn = self.conn.lock().unwrap();
        let persona = conn
            .query_row(
                "SELECT id, name, description, goal, context, tech_level, usage_frequency
                 FROM personas
                 WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Persona {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        goal: row.get(3)?,
                        context: row.get(4)?,
                        tech_level: row.get(5)?,
                        usage_frequency: row.get(6)?,
                        metadata: Metadata::new(),
                        extra: HashMap::new(),
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
        Persona::new(
            "test-persona".to_string(),
            "Test User".to_string(),
            "A test persona for unit testing".to_string(),
            "Complete testing tasks efficiently".to_string(),
        )
        .with_tech_level(4)
        .with_usage_frequency("daily".to_string())
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
        assert_eq!(loaded_persona.description, "A test persona for unit testing");
        assert_eq!(loaded_persona.goal, "Complete testing tasks efficiently");
        assert_eq!(loaded_persona.tech_level, Some(4));
        assert_eq!(loaded_persona.usage_frequency, Some("daily".to_string()));
    }

    #[test]
    fn test_load_all_personas() {
        let repo = create_test_repo();

        let persona1 = create_test_persona();
        repo.save(&persona1).unwrap();

        let persona2 = Persona::new(
            "admin-persona".to_string(),
            "Admin User".to_string(),
            "System administrator".to_string(),
            "Manage system".to_string(),
        );
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
    fn test_persona_with_optional_fields() {
        let repo = create_test_repo();

        let persona = Persona::new(
            "minimal-persona".to_string(),
            "Minimal User".to_string(),
            "Minimal persona".to_string(),
            "Do stuff".to_string(),
        );
        repo.save(&persona).unwrap();

        let loaded = repo.load_by_id("minimal-persona").unwrap().unwrap();
        assert_eq!(loaded.tech_level, None);
        assert_eq!(loaded.usage_frequency, None);
        assert_eq!(loaded.context, None);
    }

    #[test]
    fn test_persona_with_context() {
        let repo = create_test_repo();

        let persona = create_test_persona()
            .with_context("Works remotely from home office".to_string());
        repo.save(&persona).unwrap();

        let loaded = repo.load_by_id("test-persona").unwrap().unwrap();
        assert_eq!(
            loaded.context,
            Some("Works remotely from home office".to_string())
        );
    }

    #[test]
    fn test_update_persona() {
        let repo = create_test_repo();

        let mut persona = create_test_persona();
        repo.save(&persona).unwrap();

        persona.name = "Updated Test User".to_string();
        persona.tech_level = Some(5);
        repo.save(&persona).unwrap();

        let loaded = repo.load_by_id("test-persona").unwrap().unwrap();
        assert_eq!(loaded.name, "Updated Test User");
        assert_eq!(loaded.tech_level, Some(5));
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
            let persona = Persona::new(
                format!("persona-{}", i),
                format!("User {}", i),
                format!("Description {}", i),
                format!("Goal {}", i),
            );
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
