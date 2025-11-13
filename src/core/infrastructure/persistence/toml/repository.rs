// TOML-based implementation of UseCaseRepository
use crate::config::Config;
use crate::core::infrastructure::persistence::traits::UseCaseRepository;
use crate::core::{to_snake_case, UseCase};
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Repository implementation that persists use cases to TOML files
///
/// Architecture:
/// - TOML files (.toml) are the source of truth - users edit these
/// - Markdown files (.md) are generated documentation - regenerated from TOML
pub struct TomlUseCaseRepository {
    config: Config,
}

impl TomlUseCaseRepository {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl UseCaseRepository for TomlUseCaseRepository {
    fn save(&self, use_case: &UseCase) -> Result<()> {
        self.save_toml_only(use_case)
    }

    fn save_markdown(&self, use_case_id: &str, markdown_content: &str) -> Result<()> {
        self.save_markdown_only(use_case_id, markdown_content)
    }

    fn find_by_category(&self, category: &str) -> Result<Vec<UseCase>> {
        let all_cases = self.load_all()?;
        Ok(all_cases.into_iter().filter(|uc| uc.category == category).collect())
    }

    fn find_by_priority(&self, priority: &str) -> Result<Vec<UseCase>> {
        let all_cases = self.load_all()?;
        Ok(all_cases.into_iter().filter(|uc| uc.priority.to_string().to_lowercase() == priority.to_lowercase()).collect())
    }

    fn search_by_title(&self, query: &str) -> Result<Vec<UseCase>> {
        let all_cases = self.load_all()?;
        let query_lower = query.to_lowercase();
        Ok(all_cases.into_iter().filter(|uc| uc.title.to_lowercase().contains(&query_lower)).collect())
    }

    fn save_batch(&self, use_cases: &[UseCase]) -> Result<()> {
        for use_case in use_cases {
            self.save(use_case)?;
        }
        Ok(())
    }

    fn delete_batch(&self, ids: &[&str]) -> Result<()> {
        for id in ids {
            self.delete(id)?;
        }
        Ok(())
    }

    fn backend_name(&self) -> &'static str {
        "toml"
    }

    fn health_check(&self) -> Result<()> {
        let toml_dir = Path::new(self.config.directories.get_toml_dir());
        if !toml_dir.exists() {
            fs::create_dir_all(toml_dir)?;
        }
        // Try to create a temporary file to check write permissions
        let test_file = toml_dir.join(".health_check.tmp");
        fs::write(&test_file, "test")?;
        fs::remove_file(&test_file)?;
        Ok(())
    }

    fn load_all(&self) -> Result<Vec<UseCase>> {
        let toml_dir = Path::new(self.config.directories.get_toml_dir());
        let mut use_cases = Vec::new();

        if !toml_dir.exists() {
            return Ok(use_cases); // No use cases yet
        }

        for entry in walkdir::WalkDir::new(toml_dir) {
            let entry = entry?;

            // Only process .toml files that start with "UC-" (use case ID pattern)
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "toml")
                && entry
                    .path()
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("UC-"))
            {
                let content = fs::read_to_string(entry.path())?;
                // Parse TOML to intermediate value, then convert to JSON value to ensure
                // extra fields are serde_json::Value instead of toml::Value
                let toml_value: toml::Value = toml::from_str(&content)?;
                let json_str = serde_json::to_string(&toml_value)?;
                let use_case: UseCase = serde_json::from_str(&json_str)?;
                use_cases.push(use_case);
            }
        }

        Ok(use_cases)
    }

    fn load_by_id(&self, id: &str) -> Result<Option<UseCase>> {
        let all_cases = self.load_all()?;
        Ok(all_cases.into_iter().find(|uc| uc.id == id))
    }

    fn delete(&self, id: &str) -> Result<()> {
        // Find the use case to get its category
        let use_case = self
            .load_by_id(id)?
            .ok_or_else(|| anyhow::anyhow!("Use case not found: {}", id))?;

        let category_snake = to_snake_case(&use_case.category);

        // Delete TOML file
        let toml_dir = Path::new(self.config.directories.get_toml_dir()).join(&category_snake);
        let toml_path = toml_dir.join(format!("{}.toml", id));
        if toml_path.exists() {
            fs::remove_file(&toml_path)?;
        }

        // Delete markdown file
        let md_dir = Path::new(&self.config.directories.use_case_dir).join(&category_snake);
        let md_path = md_dir.join(format!("{}.md", id));
        if md_path.exists() {
            fs::remove_file(&md_path)?;
        }

        Ok(())
    }

    fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.load_by_id(id)?.is_some())
    }
}

impl TomlUseCaseRepository {
    fn save_toml_only(&self, use_case: &UseCase) -> Result<()> {
        let category_snake = to_snake_case(&use_case.category);

        // Create TOML directory structure (source files)
        let toml_dir = Path::new(self.config.directories.get_toml_dir()).join(&category_snake);
        fs::create_dir_all(&toml_dir)?;

        // Filter out Null values from extra fields before serialization
        // TOML doesn't support null values like JSON does
        let mut use_case_for_toml = use_case.clone();
        use_case_for_toml.extra.retain(|_, v| !v.is_null());

        // Save TOML file (source of truth)
        let toml_path = toml_dir.join(format!("{}.toml", use_case.id));
        let toml_content = toml::to_string_pretty(&use_case_for_toml)?;
        fs::write(&toml_path, toml_content)?;

        Ok(())
    }

    fn save_markdown_only(&self, use_case_id: &str, markdown_content: &str) -> Result<()> {
        // Load the use case from TOML to get category
        let use_case = self
            .load_by_id(use_case_id)?
            .ok_or_else(|| anyhow::anyhow!("Use case {} not found in TOML", use_case_id))?;

        let category_snake = to_snake_case(&use_case.category);

        // Create markdown directory structure (generated docs)
        let md_dir = Path::new(&self.config.directories.use_case_dir).join(&category_snake);
        fs::create_dir_all(&md_dir)?;

        // Save markdown file (generated output)
        let md_path = md_dir.join(format!("{}.md", use_case.id));
        fs::write(&md_path, markdown_content)?;

        Ok(())
    }
}
