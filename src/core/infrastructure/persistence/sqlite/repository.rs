//! SQLite implementation of the UseCaseRepository trait.
//!
//! This module provides a complete SQLite-based repository for use cases,
//! implementing all methods from the UseCaseRepository trait with proper
//! transaction support and error handling.

use crate::core::domain::UseCase;
use crate::core::infrastructure::persistence::sqlite::Migrator;
use crate::core::infrastructure::persistence::traits::{
    TransactionalRepository, UseCaseRepository,
};
use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, Transaction};
use std::path::Path;
use std::sync::Mutex;

/// SQLite-based repository for use cases.
///
/// Provides full CRUD operations, querying, batch operations, and transaction
/// support for use case persistence.
///
/// # Thread Safety
/// Uses `Mutex<Connection>` for thread-safe database access.
///
/// # Error Handling
/// All methods return `anyhow::Result` with descriptive error messages.
///
/// # Performance
/// - Indexed queries for fast lookups
/// - Batch operations for efficiency
/// - Connection pooling via Mutex
pub struct SqliteUseCaseRepository {
    /// Thread-safe database connection
    conn: Mutex<Connection>,
    /// Path to the database file (used for relative markdown storage)
    db_path: std::path::PathBuf,
}

impl SqliteUseCaseRepository {
    /// Create a new SQLite repository with database at the given path.
    ///
    /// # Arguments
    /// * `db_path` - Path to the SQLite database file
    ///
    /// # Returns
    /// New repository instance, or error if database setup fails
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path_buf = db_path.as_ref().to_path_buf();
        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open database at {:?}", db_path.as_ref()))?;

        // Enable foreign keys for data integrity
        conn.execute("PRAGMA foreign_keys = ON", [])
            .context("Failed to enable foreign keys")?;

        // Run migrations to ensure schema is up to date
        Migrator::migrate(&conn).context("Failed to run database migrations")?;

        Ok(Self {
            conn: Mutex::new(conn),
            db_path: db_path_buf,
        })
    }

    /// Get a connection from the mutex (internal helper).
    fn get_conn(&self) -> Result<std::sync::MutexGuard<Connection>> {
        self.conn
            .lock()
            .map_err(|e| anyhow!("Failed to acquire database lock: {}", e))
    }

    /// Execute a query within a database transaction (internal helper).
    fn with_db_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Transaction) -> Result<T>,
    {
        let mut conn = self.get_conn()?;
        let tx = conn.transaction().context("Failed to start transaction")?;
        let result = f(&tx)?;
        tx.commit().context("Failed to commit transaction")?;
        Ok(result)
    }

    /// Load scenarios for a use case from relational tables (Connection version).
    fn load_scenarios_for_use_case(conn: &Connection, use_case_id: &str) -> Result<Vec<crate::core::domain::Scenario>> {
        use crate::core::domain::{Scenario, ScenarioStep, ScenarioReference, ReferenceType, ScenarioType, Status};

        let mut scenarios = Vec::new();

        // Load all scenarios for this use case
        let mut stmt = conn.prepare(
            "SELECT id, title, description, scenario_type, status, persona, created_at, updated_at, extra_json
             FROM scenarios WHERE use_case_id = ? ORDER BY id"
        )?;
        
        let scenario_rows = stmt.query_map([use_case_id], |row| {
            let scenario_id: String = row.get(0)?;
            let extra_json: String = row.get(8)?;
            let extra: std::collections::HashMap<String, serde_json::Value> =
                serde_json::from_str(&extra_json).unwrap_or_default();

            Ok((scenario_id, (
                row.get::<_, String>(1)?,  // title
                row.get::<_, String>(2)?,  // description
                row.get::<_, String>(3)?,  // scenario_type
                row.get::<_, String>(4)?,  // status
                row.get::<_, Option<String>>(5)?,  // persona
                row.get::<_, String>(6)?,  // created_at
                row.get::<_, String>(7)?,  // updated_at
                extra,
            )))
        })?;

        for scenario_result in scenario_rows {
            let (scenario_id, (title, description, scenario_type_str, status_str, persona, created_at_str, updated_at_str, extra)) = scenario_result?;

            // Parse scenario type and status
            let scenario_type = scenario_type_str.parse().unwrap_or_default();
            let status = crate::core::domain::Status::from_str(&status_str).unwrap_or(crate::core::domain::Status::Planned);

            // Load steps
            let mut steps_stmt = conn.prepare(
                "SELECT step_order, actor, action, description, notes FROM scenario_steps WHERE scenario_id = ? ORDER BY step_order"
            )?;
            let step_rows = steps_stmt.query_map([&scenario_id], |row| {
                Ok(ScenarioStep {
                    order: row.get(0)?,
                    actor: row.get(1)?,
                    action: row.get(2)?,
                    description: row.get(3)?,
                    notes: row.get(4)?,
                })
            })?;
            let steps: Vec<ScenarioStep> = step_rows.collect::<Result<Vec<_>, _>>()?;

            // Load preconditions
            let mut precond_stmt = conn.prepare(
                "SELECT condition_text FROM scenario_preconditions WHERE scenario_id = ? ORDER BY condition_order"
            )?;
            let precond_rows = precond_stmt.query_map([&scenario_id], |row| row.get::<_, String>(0))?;
            let preconditions: Vec<String> = precond_rows.collect::<Result<Vec<_>, _>>()?;

            // Load postconditions
            let mut postcond_stmt = conn.prepare(
                "SELECT condition_text FROM scenario_postconditions WHERE scenario_id = ? ORDER BY condition_order"
            )?;
            let postcond_rows = postcond_stmt.query_map([&scenario_id], |row| row.get::<_, String>(0))?;
            let postconditions: Vec<String> = postcond_rows.collect::<Result<Vec<_>, _>>()?;

            // Load references
            let mut ref_stmt = conn.prepare(
                "SELECT ref_type, target_id, relationship, description FROM scenario_references WHERE scenario_id = ? ORDER BY id"
            )?;
            let ref_rows = ref_stmt.query_map([&scenario_id], |row| {
                let ref_type_str: String = row.get(0)?;
                let ref_type = ref_type_str.parse().unwrap_or(crate::core::domain::ReferenceType::UseCase);
                Ok(ScenarioReference {
                    ref_type,
                    target_id: row.get(1)?,
                    relationship: row.get(2)?,
                    description: row.get(3)?,
                })
            })?;
            let references: Vec<ScenarioReference> = ref_rows.collect::<Result<Vec<_>, _>>()?;

            scenarios.push(Scenario {
                id: scenario_id,
                title,
                description,
                scenario_type,
                status,
                persona,
                steps,
                preconditions,
                postconditions,
                references,
                metadata: crate::core::domain::Metadata {
                    created_at: created_at_str.parse().context("Failed to parse created_at")?,
                    updated_at: updated_at_str.parse().context("Failed to parse updated_at")?,
                },
                extra,
            });
        }

        Ok(scenarios)
    }

    /// Load scenarios for a use case from relational tables (Transaction version).
    fn load_scenarios_for_use_case_tx(tx: &Transaction, use_case_id: &str) -> Result<Vec<crate::core::domain::Scenario>> {
        use crate::core::domain::{Scenario, ScenarioStep, ScenarioReference};

        let mut scenarios = Vec::new();

        // Load all scenarios for this use case
        let mut stmt = tx.prepare(
            "SELECT id, title, description, scenario_type, status, persona, created_at, updated_at, extra_json
             FROM scenarios WHERE use_case_id = ? ORDER BY id"
        )?;
        
        let scenario_rows = stmt.query_map([use_case_id], |row| {
            let scenario_id: String = row.get(0)?;
            let extra_json: String = row.get(8)?;
            let extra: std::collections::HashMap<String, serde_json::Value> =
                serde_json::from_str(&extra_json).unwrap_or_default();

            Ok((scenario_id, (
                row.get::<_, String>(1)?,  // title
                row.get::<_, String>(2)?,  // description
                row.get::<_, String>(3)?,  // scenario_type
                row.get::<_, String>(4)?,  // status
                row.get::<_, Option<String>>(5)?,  // persona
                row.get::<_, String>(6)?,  // created_at
                row.get::<_, String>(7)?,  // updated_at
                extra,
            )))
        })?;

        for scenario_result in scenario_rows {
            let (scenario_id, (title, description, scenario_type_str, status_str, persona, created_at_str, updated_at_str, extra)) = scenario_result?;

            // Parse scenario type and status
            let scenario_type = scenario_type_str.parse().unwrap_or_default();
            let status = crate::core::domain::Status::from_str(&status_str).unwrap_or(crate::core::domain::Status::Planned);

            // Load steps
            let mut steps_stmt = tx.prepare(
                "SELECT step_order, actor, action, description, notes FROM scenario_steps WHERE scenario_id = ? ORDER BY step_order"
            )?;
            let step_rows = steps_stmt.query_map([&scenario_id], |row| {
                Ok(ScenarioStep {
                    order: row.get(0)?,
                    actor: row.get(1)?,
                    action: row.get(2)?,
                    description: row.get(3)?,
                    notes: row.get(4)?,
                })
            })?;
            let steps: Vec<ScenarioStep> = step_rows.collect::<Result<Vec<_>, _>>()?;

            // Load preconditions
            let mut precond_stmt = tx.prepare(
                "SELECT condition_text FROM scenario_preconditions WHERE scenario_id = ? ORDER BY condition_order"
            )?;
            let precond_rows = precond_stmt.query_map([&scenario_id], |row| row.get::<_, String>(0))?;
            let preconditions: Vec<String> = precond_rows.collect::<Result<Vec<_>, _>>()?;

            // Load postconditions
            let mut postcond_stmt = tx.prepare(
                "SELECT condition_text FROM scenario_postconditions WHERE scenario_id = ? ORDER BY condition_order"
            )?;
            let postcond_rows = postcond_stmt.query_map([&scenario_id], |row| row.get::<_, String>(0))?;
            let postconditions: Vec<String> = postcond_rows.collect::<Result<Vec<_>, _>>()?;

            // Load references
            let mut ref_stmt = tx.prepare(
                "SELECT ref_type, target_id, relationship, description FROM scenario_references WHERE scenario_id = ? ORDER BY id"
            )?;
            let ref_rows = ref_stmt.query_map([&scenario_id], |row| {
                let ref_type_str: String = row.get(0)?;
                let ref_type = ref_type_str.parse().unwrap_or(crate::core::domain::ReferenceType::UseCase);
                Ok(ScenarioReference {
                    ref_type,
                    target_id: row.get(1)?,
                    relationship: row.get(2)?,
                    description: row.get(3)?,
                })
            })?;
            let references: Vec<ScenarioReference> = ref_rows.collect::<Result<Vec<_>, _>>()?;

            scenarios.push(Scenario {
                id: scenario_id,
                title,
                description,
                scenario_type,
                status,
                persona,
                steps,
                preconditions,
                postconditions,
                references,
                metadata: crate::core::domain::Metadata {
                    created_at: created_at_str.parse().context("Failed to parse created_at")?,
                    updated_at: updated_at_str.parse().context("Failed to parse updated_at")?,
                },
                extra,
            });
        }

        Ok(scenarios)
    }

    /// Save a use case to the database (internal implementation).
    fn save_internal(tx: &Transaction, use_case: &UseCase) -> Result<()> {
        // Serialize extra fields to JSON (scenarios are now in separate tables)
        let extra_json = serde_json::to_string(&use_case.extra)
            .context("Failed to serialize extra fields to JSON")?;

        // Insert or replace the main use case record
        tx.execute(
            r#"
            INSERT OR REPLACE INTO use_cases (
                id, title, category, description, priority,
                created_at, updated_at, extra_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            params![
                use_case.id,
                use_case.title,
                use_case.category,
                use_case.description,
                use_case.priority.to_string().to_uppercase(),
                use_case.metadata.created_at.to_rfc3339(),
                use_case.metadata.updated_at.to_rfc3339(),
                extra_json,
            ],
        )
        .context("Failed to save use case")?;

        // Clear existing preconditions, postconditions, and references
        tx.execute(
            "DELETE FROM use_case_preconditions WHERE use_case_id = ?",
            [&use_case.id],
        )
        .context("Failed to clear existing preconditions")?;
        tx.execute(
            "DELETE FROM use_case_postconditions WHERE use_case_id = ?",
            [&use_case.id],
        )
        .context("Failed to clear existing postconditions")?;
        tx.execute(
            "DELETE FROM use_case_references WHERE use_case_id = ?",
            [&use_case.id],
        )
        .context("Failed to clear existing references")?;

        // Clear existing scenarios (CASCADE will delete related data)
        tx.execute(
            "DELETE FROM scenarios WHERE use_case_id = ?",
            [&use_case.id],
        )
        .context("Failed to clear existing scenarios")?;

        // Insert preconditions
        for (index, precondition) in use_case.preconditions.iter().enumerate() {
            tx.execute(
                "INSERT INTO use_case_preconditions (use_case_id, condition_order, condition_text) VALUES (?, ?, ?)",
                params![use_case.id, index as i32, precondition],
            )
            .context("Failed to save precondition")?;
        }

        // Insert postconditions
        for (index, postcondition) in use_case.postconditions.iter().enumerate() {
            tx.execute(
                "INSERT INTO use_case_postconditions (use_case_id, condition_order, condition_text) VALUES (?, ?, ?)",
                params![use_case.id, index as i32, postcondition],
            )
            .context("Failed to save postcondition")?;
        }

        // Insert use case references
        for reference in &use_case.use_case_references {
            tx.execute(
                "INSERT INTO use_case_references (use_case_id, target_id, relationship, description) VALUES (?, ?, ?, ?)",
                params![use_case.id, reference.target_id, reference.relationship, reference.description],
            )
            .context("Failed to save use case reference")?;
        }

        // Insert scenarios
        for scenario in &use_case.scenarios {
            // Serialize scenario extra fields
            let scenario_extra_json = serde_json::to_string(&scenario.extra)
                .context("Failed to serialize scenario extra fields")?;

            tx.execute(
                "INSERT INTO scenarios (id, use_case_id, title, description, scenario_type, status, persona, created_at, updated_at, extra_json)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    scenario.id,
                    use_case.id,
                    scenario.title,
                    scenario.description,
                    scenario.scenario_type.to_string(),
                    scenario.status.to_string(),
                    scenario.persona,
                    scenario.metadata.created_at.to_rfc3339(),
                    scenario.metadata.updated_at.to_rfc3339(),
                    scenario_extra_json,
                ],
            )
            .context("Failed to save scenario")?;

            // Insert scenario steps
            for step in &scenario.steps {
                tx.execute(
                    "INSERT INTO scenario_steps (scenario_id, step_order, actor, action, description, notes)
                     VALUES (?, ?, ?, ?, ?, ?)",
                    params![scenario.id, step.order, step.actor, step.action, step.description, step.notes],
                )
                .context("Failed to save scenario step")?;
            }

            // Insert scenario preconditions
            for (index, precondition) in scenario.preconditions.iter().enumerate() {
                tx.execute(
                    "INSERT INTO scenario_preconditions (scenario_id, condition_order, condition_text)
                     VALUES (?, ?, ?)",
                    params![scenario.id, index as i32, precondition],
                )
                .context("Failed to save scenario precondition")?;
            }

            // Insert scenario postconditions
            for (index, postcondition) in scenario.postconditions.iter().enumerate() {
                tx.execute(
                    "INSERT INTO scenario_postconditions (scenario_id, condition_order, condition_text)
                     VALUES (?, ?, ?)",
                    params![scenario.id, index as i32, postcondition],
                )
                .context("Failed to save scenario postcondition")?;
            }

            // Insert scenario references
            for reference in &scenario.references {
                tx.execute(
                    "INSERT INTO scenario_references (scenario_id, ref_type, target_id, relationship, description)
                     VALUES (?, ?, ?, ?, ?)",
                    params![
                        scenario.id,
                        reference.ref_type.to_string(),
                        reference.target_id,
                        reference.relationship,
                        reference.description,
                    ],
                )
                .context("Failed to save scenario reference")?;
            }
        }

        Ok(())
    }

    /// Load a use case from the database (internal implementation for Connection).
    fn load_by_id_internal_conn(conn: &Connection, id: &str) -> Result<Option<UseCase>> {
        // Query the main use case record
        let mut stmt = conn
            .prepare(
                r#"
            SELECT id, title, category, description, priority,
                   created_at, updated_at, extra_json
            FROM use_cases WHERE id = ?
            "#,
            )
            .context("Failed to prepare use case query")?;

        let mut rows = stmt
            .query_map([id], |row| {
                let extra_json: String = row.get(7)?;
                let extra: std::collections::HashMap<String, serde_json::Value> =
                    serde_json::from_str(&extra_json).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            7,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

                Ok(UseCase {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    category: row.get(2)?,
                    description: row.get(3)?,
                    priority: row.get::<_, String>(4)?.parse().map_err(|e: String| {
                        rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                        )
                    })?,
                    metadata: crate::core::domain::Metadata {
                        created_at: row.get::<_, String>(5)?.parse().map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                5,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?,
                        updated_at: row.get::<_, String>(6)?.parse().map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                6,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?,
                    },
                    preconditions: Vec::new(),  // Will be populated below
                    postconditions: Vec::new(), // Will be populated below
                    use_case_references: Vec::new(), // Will be populated below
                    scenarios: Vec::new(), // Will be loaded from relational tables
                    extra,
                })
            })
            .context("Failed to execute use case query")?;

        let mut use_case = match rows.next() {
            Some(row) => row.context("Failed to read use case row")?,
            None => return Ok(None),
        };

        // Load preconditions
        let mut preconditions = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT condition_text FROM use_case_preconditions WHERE use_case_id = ? ORDER BY condition_order"
        )
        .context("Failed to prepare preconditions query")?;
        let precondition_rows = stmt
            .query_map([id], |row| row.get::<_, String>(0))
            .context("Failed to execute preconditions query")?;
        for text in precondition_rows {
            preconditions.push(text.context("Failed to read precondition")?);
        }
        use_case.preconditions = preconditions;

        // Load postconditions
        let mut postconditions = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT condition_text FROM use_case_postconditions WHERE use_case_id = ? ORDER BY condition_order"
        )
        .context("Failed to prepare postconditions query")?;
        let postcondition_rows = stmt
            .query_map([id], |row| row.get::<_, String>(0))
            .context("Failed to execute postconditions query")?;
        for text in postcondition_rows {
            postconditions.push(text.context("Failed to read postcondition")?);
        }
        use_case.postconditions = postconditions;

        // Load references
        let mut references = Vec::new();
        let mut stmt = conn
            .prepare(
                "SELECT target_id, relationship, description FROM use_case_references WHERE use_case_id = ? ORDER BY id",
            )
            .context("Failed to prepare references query")?;
        let reference_rows = stmt
            .query_map([id], |row| {
                Ok(crate::core::domain::UseCaseReference {
                    target_id: row.get(0)?,
                    relationship: row.get(1)?,
                    description: row.get(2)?,
                })
            })
            .context("Failed to execute references query")?;
        for reference in reference_rows {
            references.push(reference.context("Failed to read reference")?);
        }
        use_case.use_case_references = references;

        // Load scenarios from relational tables
        let scenarios = Self::load_scenarios_for_use_case(conn, id)?;
        use_case.scenarios = scenarios;

        Ok(Some(use_case))
    }

    /// Load a use case from the database (internal implementation for Transaction).
    fn load_by_id_internal(tx: &Transaction, id: &str) -> Result<Option<UseCase>> {
        // Query the main use case record
        let mut stmt = tx
            .prepare(
                r#"
            SELECT id, title, category, description, priority,
                   created_at, updated_at, extra_json
            FROM use_cases WHERE id = ?
            "#,
            )
            .context("Failed to prepare use case query")?;

        let mut rows = stmt
            .query_map([id], |row| {
                let extra_json: String = row.get(7)?;
                let extra: std::collections::HashMap<String, serde_json::Value> =
                    serde_json::from_str(&extra_json).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            7,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

                Ok(UseCase {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    category: row.get(2)?,
                    description: row.get(3)?,
                    priority: row.get::<_, String>(4)?.parse().map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                        )
                    })?,
                    metadata: crate::core::domain::Metadata {
                        created_at: row.get::<_, String>(5)?.parse().map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                5,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?,
                        updated_at: row.get::<_, String>(6)?.parse().map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                6,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?,
                    },
                    preconditions: Vec::new(),  // Will be populated below
                    postconditions: Vec::new(), // Will be populated below
                    use_case_references: Vec::new(), // Will be populated below
                    scenarios: Vec::new(), // Will be loaded from relational tables
                    extra,
                })
            })
            .context("Failed to execute use case query")?;

        let mut use_case = match rows.next() {
            Some(row) => row.context("Failed to read use case row")?,
            None => return Ok(None),
        };

        // Load preconditions
        let mut preconditions = Vec::new();
        let mut stmt = tx.prepare(
            "SELECT condition_text FROM use_case_preconditions WHERE use_case_id = ? ORDER BY condition_order"
        )
        .context("Failed to prepare preconditions query")?;
        let precondition_rows = stmt
            .query_map([id], |row| row.get::<_, String>(0))
            .context("Failed to execute preconditions query")?;
        for text in precondition_rows {
            preconditions.push(text.context("Failed to read precondition")?);
        }
        use_case.preconditions = preconditions;

        // Load postconditions
        let mut postconditions = Vec::new();
        let mut stmt = tx.prepare(
            "SELECT condition_text FROM use_case_postconditions WHERE use_case_id = ? ORDER BY condition_order"
        )
        .context("Failed to prepare postconditions query")?;
        let postcondition_rows = stmt
            .query_map([id], |row| row.get::<_, String>(0))
            .context("Failed to execute postconditions query")?;
        for text in postcondition_rows {
            postconditions.push(text.context("Failed to read postcondition")?);
        }
        use_case.postconditions = postconditions;

        // Load references
        let mut references = Vec::new();
        let mut stmt = tx
            .prepare(
                "SELECT target_id, relationship, description FROM use_case_references WHERE use_case_id = ? ORDER BY id",
            )
            .context("Failed to prepare references query")?;
        let reference_rows = stmt
            .query_map([id], |row| {
                Ok(crate::core::domain::UseCaseReference {
                    target_id: row.get(0)?,
                    relationship: row.get(1)?,
                    description: row.get(2)?,
                })
            })
            .context("Failed to execute references query")?;
        for reference in reference_rows {
            references.push(reference.context("Failed to read reference")?);
        }
        use_case.use_case_references = references;

        // Load scenarios from relational tables
        let scenarios = Self::load_scenarios_for_use_case_tx(tx, id)?;
        use_case.scenarios = scenarios;

        Ok(Some(use_case))
    }
}

impl UseCaseRepository for SqliteUseCaseRepository {
    fn save(&self, use_case: &UseCase) -> Result<()> {
        self.with_db_transaction(|tx| Self::save_internal(tx, use_case))
    }

    fn load_all(&self) -> Result<Vec<UseCase>> {
        let conn = self.get_conn()?;
        let mut stmt = conn
            .prepare("SELECT id FROM use_cases ORDER BY id")
            .context("Failed to prepare load_all query")?;

        let ids = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .context("Failed to execute load_all query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = Self::load_by_id_internal_conn(&conn, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn load_by_id(&self, id: &str) -> Result<Option<UseCase>> {
        let conn = self.get_conn()?;
        Self::load_by_id_internal_conn(&conn, id)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let conn = self.get_conn()?;
        let rows_affected = conn
            .execute("DELETE FROM use_cases WHERE id = ?", [id])
            .context("Failed to delete use case")?;

        if rows_affected == 0 {
            // Not an error - use case didn't exist
        }

        Ok(())
    }

    fn exists(&self, id: &str) -> Result<bool> {
        let conn = self.get_conn()?;
        let mut stmt = conn
            .prepare("SELECT 1 FROM use_cases WHERE id = ? LIMIT 1")
            .context("Failed to prepare exists query")?;

        let exists = stmt
            .exists([id])
            .context("Failed to execute exists query")?;

        Ok(exists)
    }

    fn save_markdown(&self, use_case_id: &str, content: &str) -> Result<()> {
        // Save markdown files in a directory relative to the database location
        // This ensures test isolation and proper organization
        let db_dir = self.db_path.parent().unwrap_or(std::path::Path::new("."));
        let markdown_dir = db_dir.join("markdown");
        std::fs::create_dir_all(&markdown_dir)
            .with_context(|| format!("Failed to create markdown directory {:?}", markdown_dir))?;

        let filename = format!("{}.md", use_case_id);
        let filepath = markdown_dir.join(filename);
        std::fs::write(&filepath, content)
            .with_context(|| format!("Failed to write markdown file {:?}", filepath))?;

        Ok(())
    }

    fn find_by_category(&self, category: &str) -> Result<Vec<UseCase>> {
        let conn = self.get_conn()?;
        let mut stmt = conn
            .prepare("SELECT id FROM use_cases WHERE category = ? ORDER BY id")
            .context("Failed to prepare find_by_category query")?;

        let ids = stmt
            .query_map([category], |row| row.get::<_, String>(0))
            .context("Failed to execute find_by_category query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = Self::load_by_id_internal_conn(&conn, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn find_by_priority(&self, priority: &str) -> Result<Vec<UseCase>> {
        let conn = self.get_conn()?;
        let mut stmt = conn
            .prepare("SELECT id FROM use_cases WHERE UPPER(priority) = UPPER(?) ORDER BY id")
            .context("Failed to prepare find_by_priority query")?;

        let ids = stmt
            .query_map([priority], |row| row.get::<_, String>(0))
            .context("Failed to execute find_by_priority query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = Self::load_by_id_internal_conn(&conn, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn search_by_title(&self, query: &str) -> Result<Vec<UseCase>> {
        let conn = self.get_conn()?;
        let search_pattern = format!("%{}%", query.to_lowercase());
        let mut stmt = conn
            .prepare("SELECT id FROM use_cases WHERE LOWER(title) LIKE ? ORDER BY id")
            .context("Failed to prepare search_by_title query")?;

        let ids = stmt
            .query_map([search_pattern], |row| row.get::<_, String>(0))
            .context("Failed to execute search_by_title query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = Self::load_by_id_internal_conn(&conn, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn save_batch(&self, use_cases: &[UseCase]) -> Result<()> {
        self.with_db_transaction(|tx| {
            for use_case in use_cases {
                Self::save_internal(tx, use_case)
                    .with_context(|| format!("Failed to save use case {}", use_case.id))?;
            }
            Ok(())
        })
    }

    fn delete_batch(&self, ids: &[&str]) -> Result<()> {
        self.with_db_transaction(|tx| {
            for id in ids {
                tx.execute("DELETE FROM use_cases WHERE id = ?", [id])
                    .with_context(|| format!("Failed to delete use case {}", id))?;
            }
            Ok(())
        })
    }

    fn backend_name(&self) -> &'static str {
        "sqlite"
    }

    fn health_check(&self) -> Result<()> {
        let conn = self.get_conn()?;

        // Check if we can execute a simple query
        let result: i32 = conn
            .query_row("SELECT 1", [], |row| row.get(0))
            .context("Database health check failed - cannot execute queries")?;

        if result != 1 {
            return Err(anyhow!("Database health check failed - unexpected result"));
        }

        Ok(())
    }
}

impl TransactionalRepository for SqliteUseCaseRepository {
    fn with_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&dyn UseCaseRepository) -> Result<T>,
    {
        let mut conn = self.get_conn()?;
        let tx = conn.transaction().context("Failed to start transaction")?;

        // Create a temporary repository that uses the transaction
        let tx_repo = TransactionalSqliteRepository {
            tx: &tx,
            db_path: &self.db_path,
        };

        let result = f(&tx_repo as &dyn UseCaseRepository)?;

        tx.commit().context("Failed to commit transaction")?;
        Ok(result)
    }
}

/// Temporary repository that operates within a transaction.
///
/// This is used internally by the TransactionalRepository implementation
/// to provide transactional operations.
struct TransactionalSqliteRepository<'a> {
    tx: &'a Transaction<'a>,
    db_path: &'a std::path::Path,
}

impl<'a> UseCaseRepository for TransactionalSqliteRepository<'a> {
    fn save(&self, use_case: &UseCase) -> Result<()> {
        SqliteUseCaseRepository::save_internal(self.tx, use_case)
    }

    fn load_all(&self) -> Result<Vec<UseCase>> {
        let mut stmt = self
            .tx
            .prepare("SELECT id FROM use_cases ORDER BY id")
            .context("Failed to prepare load_all query")?;

        let ids = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .context("Failed to execute load_all query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = SqliteUseCaseRepository::load_by_id_internal(self.tx, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn load_by_id(&self, id: &str) -> Result<Option<UseCase>> {
        SqliteUseCaseRepository::load_by_id_internal(self.tx, id)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let rows_affected = self
            .tx
            .execute("DELETE FROM use_cases WHERE id = ?", [id])
            .context("Failed to delete use case")?;

        if rows_affected == 0 {
            // Not an error - use case didn't exist
        }

        Ok(())
    }

    fn exists(&self, id: &str) -> Result<bool> {
        let mut stmt = self
            .tx
            .prepare("SELECT 1 FROM use_cases WHERE id = ? LIMIT 1")
            .context("Failed to prepare exists query")?;

        let exists = stmt
            .exists([id])
            .context("Failed to execute exists query")?;

        Ok(exists)
    }

    fn save_markdown(&self, use_case_id: &str, content: &str) -> Result<()> {
        // Save markdown files in a directory relative to the database location
        // This ensures test isolation and proper organization
        let db_dir = self.db_path.parent().unwrap_or(std::path::Path::new("."));
        let markdown_dir = db_dir.join("markdown");
        std::fs::create_dir_all(&markdown_dir)
            .with_context(|| format!("Failed to create markdown directory {:?}", markdown_dir))?;

        let filename = format!("{}.md", use_case_id);
        let filepath = markdown_dir.join(filename);
        std::fs::write(&filepath, content)
            .with_context(|| format!("Failed to write markdown file {:?}", filepath))?;

        Ok(())
    }

    fn find_by_category(&self, category: &str) -> Result<Vec<UseCase>> {
        let mut stmt = self
            .tx
            .prepare("SELECT id FROM use_cases WHERE category = ? ORDER BY id")
            .context("Failed to prepare find_by_category query")?;

        let ids = stmt
            .query_map([category], |row| row.get::<_, String>(0))
            .context("Failed to execute find_by_category query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = SqliteUseCaseRepository::load_by_id_internal(self.tx, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn find_by_priority(&self, priority: &str) -> Result<Vec<UseCase>> {
        let mut stmt = self
            .tx
            .prepare("SELECT id FROM use_cases WHERE UPPER(priority) = UPPER(?) ORDER BY id")
            .context("Failed to prepare find_by_priority query")?;

        let ids = stmt
            .query_map([priority], |row| row.get::<_, String>(0))
            .context("Failed to execute find_by_priority query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = SqliteUseCaseRepository::load_by_id_internal(self.tx, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn search_by_title(&self, query: &str) -> Result<Vec<UseCase>> {
        let search_pattern = format!("%{}%", query.to_lowercase());
        let mut stmt = self
            .tx
            .prepare("SELECT id FROM use_cases WHERE LOWER(title) LIKE ? ORDER BY id")
            .context("Failed to prepare search_by_title query")?;

        let ids = stmt
            .query_map([search_pattern], |row| row.get::<_, String>(0))
            .context("Failed to execute search_by_title query")?
            .collect::<Result<Vec<String>, _>>()
            .context("Failed to collect use case IDs")?;

        let mut use_cases = Vec::new();
        for id in ids {
            if let Some(use_case) = SqliteUseCaseRepository::load_by_id_internal(self.tx, &id)
                .with_context(|| format!("Failed to load use case {}", id))?
            {
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn save_batch(&self, use_cases: &[UseCase]) -> Result<()> {
        for use_case in use_cases {
            SqliteUseCaseRepository::save_internal(self.tx, use_case)
                .with_context(|| format!("Failed to save use case {}", use_case.id))?;
        }
        Ok(())
    }

    fn delete_batch(&self, ids: &[&str]) -> Result<()> {
        for id in ids {
            self.tx
                .execute("DELETE FROM use_cases WHERE id = ?", [id])
                .with_context(|| format!("Failed to delete use case {}", id))?;
        }
        Ok(())
    }

    fn backend_name(&self) -> &'static str {
        "sqlite-transaction"
    }

    fn health_check(&self) -> Result<()> {
        // Check if we can execute a simple query
        self.tx
            .execute("SELECT 1", [])
            .context("Transaction health check failed - cannot execute queries")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::{Metadata, Priority};
    use crate::core::infrastructure::persistence::traits::{
        TransactionalRepository, UseCaseRepository,
    };
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    fn create_test_use_case(id: &str, title: &str, category: &str, priority: Priority) -> UseCase {
        UseCase {
            id: id.to_string(),
            title: title.to_string(),
            category: category.to_string(),
            description: "A test use case".to_string(),
            priority,
            metadata: Metadata::new(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            use_case_references: Vec::new(),
            scenarios: Vec::new(),
            extra: HashMap::new(),
        }
    }

    fn create_temp_db() -> Result<NamedTempFile> {
        NamedTempFile::new().context("Failed to create temporary database file")
    }

    /// Test repository creation and basic health check
    #[test]
    fn test_repository_creation() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        // Health check should pass
        if let Err(e) = repo.health_check() {
            panic!("Health check failed: {}", e);
        }

        // Backend name should be correct
        assert_eq!(repo.backend_name(), "sqlite");
    }

    /// Test saving and loading a use case
    #[test]
    fn test_save_and_load_use_case() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_case = create_test_use_case("UC-TEST-001", "Test Use Case", "Test", Priority::High);

        // Save the use case
        repo.save(&use_case).unwrap();

        // Load it back
        let loaded = repo.load_by_id("UC-TEST-001").unwrap().unwrap();

        // Verify all fields match
        assert_eq!(loaded.id, use_case.id);
        assert_eq!(loaded.title, use_case.title);
        assert_eq!(loaded.category, use_case.category);
        assert_eq!(loaded.description, use_case.description);
        assert_eq!(loaded.priority, use_case.priority);
    }

    /// Test loading non-existent use case
    #[test]
    fn test_load_non_existent_use_case() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let result = repo.load_by_id("UC-NONEXISTENT").unwrap();
        assert!(result.is_none());
    }

    /// Test exists method
    #[test]
    fn test_exists() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_case = create_test_use_case("UC-TEST-001", "Test Use Case", "Test", Priority::High);

        // Should not exist initially
        assert!(!repo.exists("UC-TEST-001").unwrap());

        // Save it
        repo.save(&use_case).unwrap();

        // Should exist now
        assert!(repo.exists("UC-TEST-001").unwrap());

        // Non-existent should not exist
        assert!(!repo.exists("UC-NONEXISTENT").unwrap());
    }

    /// Test deleting a use case
    #[test]
    fn test_delete_use_case() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_case = create_test_use_case("UC-TEST-001", "Test Use Case", "Test", Priority::High);

        // Save it
        repo.save(&use_case).unwrap();
        assert!(repo.exists("UC-TEST-001").unwrap());

        // Delete it
        repo.delete("UC-TEST-001").unwrap();
        assert!(!repo.exists("UC-TEST-001").unwrap());

        // Deleting non-existent should not error
        repo.delete("UC-NONEXISTENT").unwrap();
    }

    /// Test loading all use cases
    #[test]
    fn test_load_all_use_cases() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_cases = vec![
            create_test_use_case("UC-TEST-001", "First Use Case", "Test", Priority::High),
            create_test_use_case("UC-TEST-002", "Second Use Case", "Test", Priority::Medium),
            create_test_use_case("UC-TEST-003", "Third Use Case", "Other", Priority::Low),
        ];

        // Save all use cases
        for use_case in &use_cases {
            repo.save(use_case).unwrap();
        }

        // Load all
        let loaded = repo.load_all().unwrap();

        // Should have all 3 use cases
        assert_eq!(loaded.len(), 3);

        // Should be sorted by ID
        assert_eq!(loaded[0].id, "UC-TEST-001");
        assert_eq!(loaded[1].id, "UC-TEST-002");
        assert_eq!(loaded[2].id, "UC-TEST-003");
    }

    /// Test finding by category
    #[test]
    fn test_find_by_category() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_cases = vec![
            create_test_use_case("UC-TEST-001", "First Use Case", "Test", Priority::High),
            create_test_use_case("UC-TEST-002", "Second Use Case", "Test", Priority::Medium),
            create_test_use_case("UC-TEST-003", "Third Use Case", "Other", Priority::Low),
        ];

        // Save all use cases
        for use_case in &use_cases {
            repo.save(use_case).unwrap();
        }

        // Find by "Test" category
        let test_cases = repo.find_by_category("Test").unwrap();
        assert_eq!(test_cases.len(), 2);
        assert_eq!(test_cases[0].id, "UC-TEST-001");
        assert_eq!(test_cases[1].id, "UC-TEST-002");

        // Find by "Other" category
        let other_cases = repo.find_by_category("Other").unwrap();
        assert_eq!(other_cases.len(), 1);
        assert_eq!(other_cases[0].id, "UC-TEST-003");

        // Find by non-existent category
        let empty_cases = repo.find_by_category("NonExistent").unwrap();
        assert_eq!(empty_cases.len(), 0);
    }

    /// Test finding by priority
    #[test]
    fn test_find_by_priority() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_cases = vec![
            create_test_use_case("UC-TEST-001", "First Use Case", "Test", Priority::High),
            create_test_use_case("UC-TEST-002", "Second Use Case", "Test", Priority::High),
            create_test_use_case("UC-TEST-003", "Third Use Case", "Other", Priority::Low),
        ];

        // Save all use cases
        for use_case in &use_cases {
            repo.save(use_case).unwrap();
        }

        // Find by "high" priority (case insensitive)
        let high_cases = repo.find_by_priority("high").unwrap();
        assert_eq!(high_cases.len(), 2);
        assert_eq!(high_cases[0].id, "UC-TEST-001");
        assert_eq!(high_cases[1].id, "UC-TEST-002");

        // Find by "HIGH" priority (case insensitive)
        let high_cases_upper = repo.find_by_priority("HIGH").unwrap();
        assert_eq!(high_cases_upper.len(), 2);

        // Find by "low" priority
        let low_cases = repo.find_by_priority("low").unwrap();
        assert_eq!(low_cases.len(), 1);
        assert_eq!(low_cases[0].id, "UC-TEST-003");
    }

    /// Test searching by title
    #[test]
    fn test_search_by_title() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_cases = vec![
            create_test_use_case("UC-TEST-001", "User Authentication", "Auth", Priority::High),
            create_test_use_case("UC-TEST-002", "Password Reset", "Auth", Priority::Medium),
            create_test_use_case("UC-TEST-003", "Data Processing", "Data", Priority::Low),
        ];

        // Save all use cases
        for use_case in &use_cases {
            repo.save(use_case).unwrap();
        }

        // Search for "auth" (should match first)
        let auth_cases = repo.search_by_title("auth").unwrap();
        assert_eq!(auth_cases.len(), 1);
        assert_eq!(auth_cases[0].id, "UC-TEST-001");

        // Search for "password" (should match second)
        let password_cases = repo.search_by_title("password").unwrap();
        assert_eq!(password_cases.len(), 1);
        assert_eq!(password_cases[0].id, "UC-TEST-002");

        // Search for "processing" (should match third)
        let processing_cases = repo.search_by_title("processing").unwrap();
        assert_eq!(processing_cases.len(), 1);
        assert_eq!(processing_cases[0].id, "UC-TEST-003");

        // Search for non-existent term
        let empty_cases = repo.search_by_title("nonexistent").unwrap();
        assert_eq!(empty_cases.len(), 0);
    }

    /// Test batch save operation
    #[test]
    fn test_save_batch() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_cases = vec![
            create_test_use_case("UC-TEST-001", "First Use Case", "Test", Priority::High),
            create_test_use_case("UC-TEST-002", "Second Use Case", "Test", Priority::Medium),
            create_test_use_case("UC-TEST-003", "Third Use Case", "Other", Priority::Low),
        ];

        // Save batch
        repo.save_batch(&use_cases).unwrap();

        // Verify all were saved
        for use_case in &use_cases {
            assert!(repo.exists(&use_case.id).unwrap());
        }

        // Load all and verify
        let loaded = repo.load_all().unwrap();
        assert_eq!(loaded.len(), 3);
    }

    /// Test batch delete operation
    #[test]
    fn test_delete_batch() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_cases = vec![
            create_test_use_case("UC-TEST-001", "First Use Case", "Test", Priority::High),
            create_test_use_case("UC-TEST-002", "Second Use Case", "Test", Priority::Medium),
            create_test_use_case("UC-TEST-003", "Third Use Case", "Other", Priority::Low),
        ];

        // Save all first
        for use_case in &use_cases {
            repo.save(use_case).unwrap();
        }

        // Delete batch of first two
        repo.delete_batch(&["UC-TEST-001", "UC-TEST-002"]).unwrap();

        // Verify deletions
        assert!(!repo.exists("UC-TEST-001").unwrap());
        assert!(!repo.exists("UC-TEST-002").unwrap());
        assert!(repo.exists("UC-TEST-003").unwrap());
    }

    /// Test transactional operations
    #[test]
    fn test_transactional_operations() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let use_case1 =
            create_test_use_case("UC-TEST-001", "First Use Case", "Test", Priority::High);
        let use_case2 =
            create_test_use_case("UC-TEST-002", "Second Use Case", "Test", Priority::Medium);

        // Test successful transaction
        let result: Result<()> = repo.with_transaction(|tx_repo| {
            tx_repo.save(&use_case1)?;
            tx_repo.save(&use_case2)?;
            Ok(())
        });
        result.unwrap();

        // Both should exist
        assert!(repo.exists("UC-TEST-001").unwrap());
        assert!(repo.exists("UC-TEST-002").unwrap());

        // Test failed transaction (simulate error)
        let result: Result<()> = repo.with_transaction(|tx_repo| {
            tx_repo.save(&create_test_use_case(
                "UC-TEST-003",
                "Third Use Case",
                "Test",
                Priority::Low,
            ))?;
            // Simulate an error
            Err(anyhow!("Simulated transaction failure"))
        });

        // Transaction should have failed
        assert!(result.is_err());

        // Third use case should not exist (transaction rolled back)
        assert!(!repo.exists("UC-TEST-003").unwrap());
    }

    /// Test use case with extra fields
    #[test]
    fn test_use_case_with_extra_fields() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let mut use_case =
            create_test_use_case("UC-TEST-001", "Test Use Case", "Test", Priority::High);

        // Add extra fields
        use_case.extra.insert(
            "business_value".to_string(),
            serde_json::json!("High impact"),
        );
        use_case.extra.insert(
            "preconditions".to_string(),
            serde_json::json!(["User logged in", "Valid session"]),
        );
        use_case.extra.insert(
            "postconditions".to_string(),
            serde_json::json!(["Data processed", "Email sent"]),
        );
        use_case.extra.insert(
            "references".to_string(),
            serde_json::json!(["RFC-123", "API-456"]),
        );

        // Save the use case
        repo.save(&use_case).unwrap();

        // Load it back
        let loaded = repo.load_by_id("UC-TEST-001").unwrap().unwrap();

        // Verify extra fields are preserved
        assert_eq!(
            loaded.extra["business_value"],
            serde_json::json!("High impact")
        );
        assert_eq!(
            loaded.extra["preconditions"],
            serde_json::json!(["User logged in", "Valid session"])
        );
        assert_eq!(
            loaded.extra["postconditions"],
            serde_json::json!(["Data processed", "Email sent"])
        );
        assert_eq!(
            loaded.extra["references"],
            serde_json::json!(["RFC-123", "API-456"])
        );
    }

    /// Test save_markdown method
    #[test]
    fn test_save_markdown() {
        let temp_db = create_temp_db().unwrap();
        let repo = SqliteUseCaseRepository::new(temp_db.path()).unwrap();

        let content = "# Test Use Case\n\nThis is a test markdown file.";
        repo.save_markdown("UC-TEST-001", content).unwrap();

        // Verify file was created in directory relative to database
        let db_dir = temp_db.path().parent().unwrap_or(std::path::Path::new("."));
        let markdown_path = db_dir.join("markdown/UC-TEST-001.md");
        assert!(markdown_path.exists());

        // Verify content
        let saved_content = std::fs::read_to_string(markdown_path).unwrap();
        assert_eq!(saved_content, content);

        // Clean up
        let markdown_dir = db_dir.join("markdown");
        let markdown_path = markdown_dir.join("UC-TEST-001.md");
        let _ = std::fs::remove_file(&markdown_path);
        if markdown_dir.exists() {
            let _ = std::fs::remove_dir(markdown_dir);
        }
    }
}
