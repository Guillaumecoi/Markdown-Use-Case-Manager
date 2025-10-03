// src/core/services/file_service.rs
use crate::config::Config;
use crate::core::models::{Scenario, UseCase};
use crate::core::templates::to_snake_case;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Service responsible for all file I/O operations
/// Handles saving, loading, and parsing of use case files
pub struct FileService {
    config: Config,
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Save a use case to file
    pub fn save_use_case(&self, use_case: &UseCase, markdown_content: &str) -> Result<()> {
        // Create category-based directory structure
        let category_dir = Path::new(&self.config.directories.use_case_dir)
            .join(to_snake_case(&use_case.category));
        fs::create_dir_all(&category_dir)?;

        // Save markdown file
        let md_path = category_dir.join(format!("{}.md", use_case.id));
        fs::write(&md_path, markdown_content)?;

        Ok(())
    }

    /// Load all use cases from the file system
    pub fn load_use_cases(&self) -> Result<Vec<UseCase>> {
        let use_case_dir = Path::new(&self.config.directories.use_case_dir);
        let mut use_cases = Vec::new();

        if !use_case_dir.exists() {
            return Ok(use_cases); // No use cases yet
        }

        for entry in walkdir::WalkDir::new(use_case_dir) {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "md")
            {
                let content = fs::read_to_string(entry.path())?;
                if let Some(use_case) = self.parse_use_case_from_markdown(&content)? {
                    use_cases.push(use_case);
                }
            }
        }

        Ok(use_cases)
    }

    /// Save a test file
    pub fn save_test_file(
        &self,
        use_case: &UseCase,
        test_content: &str,
        file_extension: &str,
    ) -> Result<()> {
        if use_case.scenarios.is_empty() {
            return Ok(()); // No scenarios = no tests
        }

        let test_dir =
            Path::new(&self.config.directories.test_dir).join(to_snake_case(&use_case.category));
        fs::create_dir_all(&test_dir)?;

        let test_file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = test_dir.join(test_file_name);

        fs::write(&test_path, test_content)?;

        Ok(())
    }

    /// Save overview file
    pub fn save_overview(&self, content: &str) -> Result<()> {
        let overview_path = Path::new(&self.config.directories.use_case_dir).join("README.md");
        fs::write(&overview_path, content)?;
        println!("Generated overview at: {}", overview_path.display());
        Ok(())
    }

    /// Check if a test file exists
    pub fn test_file_exists(&self, use_case: &UseCase, file_extension: &str) -> bool {
        let test_dir =
            Path::new(&self.config.directories.test_dir).join(to_snake_case(&use_case.category));
        let test_file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = test_dir.join(test_file_name);
        test_path.exists()
    }

    // Private parsing methods
    fn parse_use_case_from_markdown(&self, content: &str) -> Result<Option<UseCase>> {
        // Check if file has YAML frontmatter
        if content.starts_with("---\n") {
            return self.parse_use_case_with_frontmatter(content);
        }

        // No frontmatter found - not a valid use case file
        Ok(None)
    }

    fn parse_use_case_with_frontmatter(&self, content: &str) -> Result<Option<UseCase>> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() || lines[0] != "---" {
            return Ok(None);
        }

        // Find the end of frontmatter
        let mut frontmatter_end = 0;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line == &"---" {
                frontmatter_end = i;
                break;
            }
        }

        if frontmatter_end == 0 {
            return Ok(None);
        }

        // Parse YAML frontmatter
        let frontmatter = lines[1..frontmatter_end].join("\n");
        let yaml: serde_yaml::Value = serde_yaml::from_str(&frontmatter)?;

        // Extract basic fields
        let id = yaml["id"].as_str().unwrap_or_default().to_string();
        let title = yaml["title"].as_str().unwrap_or_default().to_string();
        let category = yaml["category"].as_str().unwrap_or_default().to_string();

        if id.is_empty() || category.is_empty() {
            return Ok(None);
        }

        // Parse markdown content after frontmatter
        let markdown_content = lines[frontmatter_end + 1..].join("\n");
        let description = self.extract_description_from_markdown(&markdown_content)?;
        let scenarios = self.parse_scenarios_from_markdown_content(&markdown_content)?;

        let mut use_case = UseCase::new(id, title, category, description);
        for scenario in scenarios {
            use_case.add_scenario(scenario);
        }

        Ok(Some(use_case))
    }



    fn extract_description_from_markdown(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("## Description") {
                // Look for the next non-empty line
                for desc_line in lines.iter().skip(i + 1) {
                    let desc_line = desc_line.trim();
                    if !desc_line.is_empty() && !desc_line.starts_with("##") {
                        return Ok(desc_line.to_string());
                    }
                    if desc_line.starts_with("##") {
                        break;
                    }
                }
            }
        }

        Ok(String::new())
    }

    fn parse_scenarios_from_markdown_content(&self, content: &str) -> Result<Vec<Scenario>> {
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("## Scenarios") {
                return self.parse_scenarios_from_markdown(&lines[i + 1..]);
            }
        }

        Ok(Vec::new())
    }

    fn parse_scenarios_from_markdown(&self, lines: &[&str]) -> Result<Vec<Scenario>> {
        let mut scenarios = Vec::new();
        let mut current_scenario: Option<Scenario> = None;

        for line in lines {
            if line.starts_with("### ") && line.contains("(") && line.contains(")") {
                // Save previous scenario if exists
                if let Some(scenario) = current_scenario.take() {
                    scenarios.push(scenario);
                }

                // Parse scenario title and ID
                let parts: Vec<&str> = line[4..].split('(').collect();
                if parts.len() >= 2 {
                    let title = parts[0].trim().to_string();
                    let id_part = parts[1].replace(')', "");
                    current_scenario = Some(Scenario::new(id_part, title, String::new()));
                }
            } else if line.starts_with("**Status:**") && current_scenario.is_some() {
                // Parse status - we could enhance this to set the actual status
            } else if !line.trim().is_empty()
                && !line.starts_with("**")
                && !line.starts_with("---")
                && !line.starts_with("## ")
                && current_scenario.is_some()
            {
                // This is likely the description
                if let Some(ref mut scenario) = current_scenario {
                    if scenario.description.is_empty() {
                        scenario.description = line.trim().to_string();
                    }
                }
            } else if line.starts_with("## ") && line != &"## Scenarios" {
                // End of scenarios section
                break;
            }
        }

        // Add the last scenario if exists
        if let Some(scenario) = current_scenario {
            scenarios.push(scenario);
        }

        Ok(scenarios)
    }
}
