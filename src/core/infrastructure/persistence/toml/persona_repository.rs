// TOML-based implementation of PersonaRepository
use crate::config::Config;
use crate::core::domain::{Persona, PersonaRepository};
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Repository implementation that persists personas to TOML files
///
/// Architecture:
/// - TOML files (.toml) are the source of truth in .mucm/personas/
/// - Markdown files (.md) are generated documentation in docs/personas/
pub struct TomlPersonaRepository {
    config: Config,
}

impl TomlPersonaRepository {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get the directory for persona TOML files
    /// Stores in .mucm/personas alongside use case TOMLs
    fn get_toml_dir(&self) -> String {
        let base = self.config.directories.get_toml_dir();
        format!("{}/../personas", base)
    }

    /// Get the directory for persona markdown files
    /// Stores in docs/personas alongside use case docs
    fn get_markdown_dir(&self) -> String {
        let base = &self.config.directories.use_case_dir;
        // Extract the base path and replace use-cases with personas
        // E.g., "/tmp/xyz/docs/use-cases" -> "/tmp/xyz/docs/personas"
        if let Some(idx) = base.rfind("/use-cases") {
            format!("{}/personas", &base[..idx])
        } else if let Some(idx) = base.rfind("\\use-cases") {
            format!("{}\\personas", &base[..idx])
        } else {
            // Fallback: just append personas
            format!("{}/personas", base)
        }
    }
}

impl PersonaRepository for TomlPersonaRepository {
    fn save(&self, persona: &Persona) -> Result<()> {
        // Create TOML directory structure
        let toml_dir_str = self.get_toml_dir();
        let toml_dir = Path::new(&toml_dir_str);
        fs::create_dir_all(toml_dir)?;

        // Filter out Null values from extra fields before serialization
        // TOML doesn't support null values like JSON does
        let mut persona_for_toml = persona.clone();
        persona_for_toml.extra.retain(|_, v| !v.is_null());

        // Save TOML file (source of truth)
        let toml_path = toml_dir.join(format!("{}.toml", persona.id));
        let toml_content = toml::to_string_pretty(&persona_for_toml)?;
        fs::write(&toml_path, toml_content)?;

        Ok(())
    }

    fn load_all(&self) -> Result<Vec<Persona>> {
        let toml_dir_str = self.get_toml_dir();
        let toml_dir = Path::new(&toml_dir_str);
        let mut personas = Vec::new();

        if !toml_dir.exists() {
            return Ok(personas); // No personas yet
        }

        for entry in fs::read_dir(toml_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Only process .toml files
            if path.is_file() && path.extension().is_some_and(|ext| ext == "toml") {
                let content = fs::read_to_string(&path)?;
                // Parse TOML to intermediate value, then convert to JSON value to ensure
                // extra fields are serde_json::Value instead of toml::Value
                let toml_value: toml::Value = toml::from_str(&content)?;
                let json_str = serde_json::to_string(&toml_value)?;
                let persona: Persona = serde_json::from_str(&json_str)?;
                personas.push(persona);
            }
        }

        Ok(personas)
    }

    fn load_by_id(&self, id: &str) -> Result<Option<Persona>> {
        let toml_path = Path::new(&self.get_toml_dir()).join(format!("{}.toml", id));

        if !toml_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&toml_path)?;
        let toml_value: toml::Value = toml::from_str(&content)?;
        let json_str = serde_json::to_string(&toml_value)?;
        let persona: Persona = serde_json::from_str(&json_str)?;

        Ok(Some(persona))
    }

    fn delete(&self, id: &str) -> Result<()> {
        // Delete TOML file
        let toml_path = Path::new(&self.get_toml_dir()).join(format!("{}.toml", id));
        if toml_path.exists() {
            fs::remove_file(&toml_path)?;
        }

        // Delete markdown file
        let md_path = Path::new(&self.get_markdown_dir()).join(format!("{}.md", id));
        if md_path.exists() {
            fs::remove_file(&md_path)?;
        }

        Ok(())
    }

    fn exists(&self, id: &str) -> Result<bool> {
        let toml_path = Path::new(&self.get_toml_dir()).join(format!("{}.toml", id));
        Ok(toml_path.exists())
    }

    fn save_markdown(&self, persona_id: &str, markdown_content: &str) -> Result<()> {
        // Create markdown directory structure
        let md_dir_str = self.get_markdown_dir();
        let md_dir = Path::new(&md_dir_str);
        fs::create_dir_all(md_dir)?;

        // Save markdown file (generated output)
        let md_path = md_dir.join(format!("{}.md", persona_id));
        fs::write(&md_path, markdown_content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_repo() -> (TomlPersonaRepository, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_str().unwrap();

        // Create a test config pointing to temp directory
        // Important: use_case_dir must be in format "{base}/docs/use-cases"
        // so that get_markdown_dir() can extract "{base}/docs/personas"
        let mut config = Config::default();
        config.directories.use_case_dir = format!("{}/docs/use-cases", temp_path);
        config.directories.toml_dir = Some(format!("{}/.mucm", temp_path));

        let repo = TomlPersonaRepository::new(config);
        (repo, temp_dir)
    }

    fn create_test_persona() -> Persona {
        Persona::new("test-persona".to_string(), "Test User".to_string())
    }

    #[test]
    fn test_save_and_load_persona() {
        let (repo, _temp_dir) = create_test_repo();

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
        let (repo, _temp_dir) = create_test_repo();

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
        let (repo, _temp_dir) = create_test_repo();

        let result = repo.load_by_id("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_persona() {
        let (repo, _temp_dir) = create_test_repo();

        let persona = create_test_persona();
        repo.save(&persona).unwrap();

        assert!(repo.exists("test-persona").unwrap());

        repo.delete("test-persona").unwrap();

        assert!(!repo.exists("test-persona").unwrap());
        assert!(repo.load_by_id("test-persona").unwrap().is_none());
    }

    #[test]
    fn test_exists() {
        let (repo, _temp_dir) = create_test_repo();

        assert!(!repo.exists("test-persona").unwrap());

        let persona = create_test_persona();
        repo.save(&persona).unwrap();

        assert!(repo.exists("test-persona").unwrap());
    }

    #[test]
    fn test_save_markdown() {
        let (repo, _temp_dir) = create_test_repo();

        let persona = create_test_persona();
        repo.save(&persona).unwrap();

        let markdown_content = "# Test User\n\nA test persona documentation.";
        repo.save_markdown("test-persona", markdown_content)
            .unwrap();

        let md_dir_str = repo.get_markdown_dir();
        let md_path = Path::new(&md_dir_str).join("test-persona.md");
        assert!(md_path.exists());

        let content = fs::read_to_string(&md_path).unwrap();
        assert_eq!(content, markdown_content);
    }

    #[test]
    fn test_update_persona() {
        let (repo, _temp_dir) = create_test_repo();

        let mut persona = create_test_persona();
        repo.save(&persona).unwrap();

        persona.name = "Updated Test User".to_string();
        repo.save(&persona).unwrap();

        let loaded = repo.load_by_id("test-persona").unwrap().unwrap();
        assert_eq!(loaded.name, "Updated Test User");
    }
}
