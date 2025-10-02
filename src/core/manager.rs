// src/core/manager.rs
use super::languages::LanguageRegistry;
use super::models::{Scenario, Status, UseCase};
use super::templates::{to_snake_case, TemplateEngine};
use crate::config::Config;
use anyhow::Result;
use chrono::Utc;
use colored::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct UseCaseManager {
    config: Config,
    use_cases: Vec<UseCase>,
    template_engine: TemplateEngine,
}

impl UseCaseManager {
    pub fn load() -> Result<Self> {
        let config = Config::load()?;
        let template_engine = TemplateEngine::with_config(Some(&config));
        let mut manager = Self {
            config,
            use_cases: Vec::new(),
            template_engine,
        };

        manager.load_use_cases()?;
        Ok(manager)
    }

    pub fn create_use_case(
        &mut self,
        title: String,
        category: String,
        description: Option<String>,
    ) -> Result<String> {
        let use_case_id = self.generate_use_case_id(&category)?;
        let description = description.unwrap_or_default();

        let use_case = UseCase::new(use_case_id.clone(), title.clone(), category, description);

        // Save use case to file
        self.save_use_case(&use_case)?;

        // Note: Tests are only generated when scenarios are added (not for empty use cases)

        self.use_cases.push(use_case);

        // Automatically regenerate overview
        self.generate_overview()?;

        Ok(use_case_id)
    }

    pub fn add_scenario_to_use_case(
        &mut self,
        use_case_id: String,
        title: String,
        description: Option<String>,
    ) -> Result<String> {
        // Find the use case and get scenario count
        let use_case_index = self
            .use_cases
            .iter()
            .position(|uc| uc.id == use_case_id)
            .ok_or_else(|| anyhow::anyhow!("Use case {} not found", use_case_id))?;

        let scenario_count = self.use_cases[use_case_index].scenarios.len();
        let scenario_id = format!("{}-S{:02}", use_case_id, scenario_count + 1);
        let description = description.unwrap_or_default();

        // Create scenario
        let scenario = Scenario::new(scenario_id.clone(), title.clone(), description);

        // Add scenario to use case
        self.use_cases[use_case_index].add_scenario(scenario);

        // Clone the use case for saving (to avoid borrowing issues)
        let use_case_copy = self.use_cases[use_case_index].clone();

        // Save updated use case
        self.save_use_case(&use_case_copy)?;

        // Always generate/update test file when scenarios are added
        self.generate_test_file(&use_case_copy)?;

        // Automatically regenerate overview
        self.generate_overview()?;

        Ok(scenario_id)
    }

    pub fn update_scenario_status(
        &mut self,
        scenario_id: String,
        status_str: String,
    ) -> Result<()> {
        // Parse status
        let status = match status_str.to_lowercase().as_str() {
            "planned" => Status::Planned,
            "in_progress" => Status::InProgress,
            "implemented" => Status::Implemented,
            "tested" => Status::Tested,
            "deployed" => Status::Deployed,
            "deprecated" => Status::Deprecated,
            _ => return Err(anyhow::anyhow!("Invalid status: {}. Valid options: planned, in_progress, implemented, tested, deployed, deprecated", status_str)),
        };

        // Find use case and scenario
        for use_case in &mut self.use_cases {
            if use_case.update_scenario_status(&scenario_id, status) {
                // Clone the use case for saving (to avoid borrowing issues)
                let use_case_copy = use_case.clone();

                // Save updated use case
                self.save_use_case(&use_case_copy)?;

                // Note: We don't regenerate tests for status changes
                // Tests are only generated when scenarios are added

                // Automatically regenerate overview
                self.generate_overview()?;

                println!("Updated scenario {} status to: {}", scenario_id, status);
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("Scenario {} not found", scenario_id))
    }

    pub fn generate_overview(&self) -> Result<()> {
        let overview_path = Path::new(&self.config.directories.use_case_dir).join("README.md");

        // Prepare data for template
        let mut template_data = std::collections::HashMap::new();

        // Basic project info
        template_data.insert(
            "project_name".to_string(),
            serde_json::Value::String(self.config.project.name.clone()),
        );
        template_data.insert(
            "generated_date".to_string(),
            serde_json::Value::String(
                chrono::Utc::now()
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string(),
            ),
        );

        // Statistics
        let total_scenarios: usize = self.use_cases.iter().map(|uc| uc.scenarios.len()).sum();
        template_data.insert(
            "total_use_cases".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.use_cases.len())),
        );
        template_data.insert(
            "total_scenarios".to_string(),
            serde_json::Value::Number(serde_json::Number::from(total_scenarios)),
        );

        // Status distribution
        let mut status_counts = std::collections::HashMap::new();
        for use_case in &self.use_cases {
            let status_str = use_case.status().to_string();
            *status_counts.entry(status_str).or_insert(0) += 1;
        }
        template_data.insert(
            "status_counts".to_string(),
            serde_json::to_value(status_counts)?,
        );

        // Group by category
        let mut categories: std::collections::HashMap<String, Vec<&UseCase>> =
            std::collections::HashMap::new();
        for use_case in &self.use_cases {
            categories
                .entry(use_case.category.clone())
                .or_default()
                .push(use_case);
        }

        let mut category_data = Vec::new();
        for (category, use_cases) in categories {
            let mut use_case_data = Vec::new();
            for use_case in use_cases {
                let mut uc_data = std::collections::HashMap::new();
                uc_data.insert(
                    "id".to_string(),
                    serde_json::Value::String(use_case.id.clone()),
                );
                uc_data.insert(
                    "title".to_string(),
                    serde_json::Value::String(use_case.title.clone()),
                );
                uc_data.insert(
                    "description".to_string(),
                    serde_json::Value::String(use_case.description.clone()),
                );
                uc_data.insert(
                    "priority".to_string(),
                    serde_json::Value::String(use_case.priority.to_string()),
                );
                uc_data.insert(
                    "aggregated_status".to_string(),
                    serde_json::Value::String(use_case.status().to_string()),
                );
                uc_data.insert(
                    "category_path".to_string(),
                    serde_json::Value::String(to_snake_case(&use_case.category)),
                );
                uc_data.insert(
                    "scenario_count".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(use_case.scenarios.len())),
                );

                let scenario_data: Vec<serde_json::Value> = use_case
                    .scenarios
                    .iter()
                    .map(|s| {
                        let mut scenario_map = std::collections::HashMap::new();
                        scenario_map
                            .insert("id".to_string(), serde_json::Value::String(s.id.clone()));
                        scenario_map.insert(
                            "title".to_string(),
                            serde_json::Value::String(s.title.clone()),
                        );
                        scenario_map.insert(
                            "status".to_string(),
                            serde_json::Value::String(s.status.to_string()),
                        );
                        serde_json::Value::Object(scenario_map.into_iter().collect())
                    })
                    .collect();

                uc_data.insert(
                    "scenarios".to_string(),
                    serde_json::Value::Array(scenario_data),
                );
                use_case_data.push(serde_json::Value::Object(uc_data.into_iter().collect()));
            }

            let mut cat_data = std::collections::HashMap::new();
            cat_data.insert(
                "category_name".to_string(),
                serde_json::Value::String(category),
            );
            cat_data.insert(
                "use_cases".to_string(),
                serde_json::Value::Array(use_case_data),
            );
            category_data.push(serde_json::Value::Object(cat_data.into_iter().collect()));
        }

        template_data.insert(
            "categories".to_string(),
            serde_json::Value::Array(category_data),
        );

        // Render using template
        let content = self.template_engine.render_overview(&template_data)?;

        fs::write(&overview_path, content)?;
        println!("Generated overview at: {}", overview_path.display());

        Ok(())
    }

    pub fn list_use_cases(&self) -> Result<()> {
        if self.use_cases.is_empty() {
            println!("No use cases found. Create one with 'mucm create'");
            return Ok(());
        }

        println!("\n{}", "📋 Use Cases".bold().blue());
        println!("{}", "━".repeat(50));

        for use_case in &self.use_cases {
            let status_display = format!("{}", use_case.status());
            println!(
                "{} {} [{}] - {}",
                status_display,
                use_case.id.cyan(),
                use_case.category.yellow(),
                use_case.title.bold()
            );

            if !use_case.scenarios.is_empty() {
                for scenario in &use_case.scenarios {
                    println!(
                        "  └─ {} {} - {}",
                        scenario.status,
                        scenario.id.bright_black(),
                        scenario.title
                    );
                }
            }
            println!();
        }

        Ok(())
    }

    pub fn show_status(&self) -> Result<()> {
        let total_use_cases = self.use_cases.len();
        let total_scenarios: usize = self.use_cases.iter().map(|uc| uc.scenarios.len()).sum();

        let mut status_counts: HashMap<Status, usize> = HashMap::new();
        for use_case in &self.use_cases {
            *status_counts.entry(use_case.status()).or_insert(0) += 1;
        }

        println!("\n{}", "📊 Project Status".bold().blue());
        println!("{}", "━".repeat(50));
        println!("Total Use Cases: {}", total_use_cases.to_string().cyan());
        println!("Total Scenarios: {}", total_scenarios.to_string().cyan());
        println!();

        for (status, count) in status_counts {
            println!("{}: {}", status, count.to_string().cyan());
        }

        Ok(())
    }

    fn generate_use_case_id(&self, category: &str) -> Result<String> {
        let category_prefix = category.to_uppercase().chars().take(3).collect::<String>();
        let existing_count = self
            .use_cases
            .iter()
            .filter(|uc| uc.category.to_uppercase() == category.to_uppercase())
            .count();

        Ok(format!("UC-{}-{:03}", category_prefix, existing_count + 1))
    }

    fn load_use_cases(&mut self) -> Result<()> {
        let use_case_dir = Path::new(&self.config.directories.use_case_dir);

        if !use_case_dir.exists() {
            return Ok(()); // No use cases yet
        }

        for entry in walkdir::WalkDir::new(use_case_dir) {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "md")
            {
                let content = fs::read_to_string(entry.path())?;
                if let Some(use_case) = self.parse_use_case_from_markdown(&content)? {
                    self.use_cases.push(use_case);
                }
            }
        }

        Ok(())
    }

    fn save_use_case(&self, use_case: &UseCase) -> Result<()> {
        // Create category-based directory structure
        let category_dir = Path::new(&self.config.directories.use_case_dir)
            .join(to_snake_case(&use_case.category));
        fs::create_dir_all(&category_dir)?;

        // Save only Markdown with embedded metadata
        let md_path = category_dir.join(format!("{}.md", use_case.id));
        let md_content = self.generate_use_case_markdown(use_case)?;
        fs::write(&md_path, md_content)?;

        Ok(())
    }

    fn generate_use_case_markdown(&self, use_case: &UseCase) -> Result<String> {
        let mut data = HashMap::new();
        data.insert("id".to_string(), json!(use_case.id));
        data.insert("title".to_string(), json!(use_case.title));
        data.insert("category".to_string(), json!(use_case.category));
        data.insert("priority".to_string(), json!(use_case.priority.to_string()));
        data.insert(
            "status_name".to_string(),
            json!(use_case.status().display_name()),
        );
        data.insert("description".to_string(), json!(use_case.description));
        data.insert("scenarios".to_string(), json!(use_case.scenarios));
        data.insert("metadata".to_string(), json!(use_case.metadata));

        // Format dates nicely (YYYY-MM-DD)
        data.insert(
            "created_date".to_string(),
            json!(use_case.metadata.created_at.format("%Y-%m-%d").to_string()),
        );
        data.insert(
            "updated_date".to_string(),
            json!(use_case.metadata.updated_at.format("%Y-%m-%d").to_string()),
        );

        // Add metadata configuration
        let metadata_config = &self.config.metadata;
        data.insert(
            "metadata_enabled".to_string(),
            json!(metadata_config.enabled),
        );
        data.insert("include_id".to_string(), json!(metadata_config.include_id));
        data.insert(
            "include_title".to_string(),
            json!(metadata_config.include_title),
        );
        data.insert(
            "include_category".to_string(),
            json!(metadata_config.include_category),
        );
        data.insert(
            "include_status".to_string(),
            json!(metadata_config.include_status),
        );
        data.insert(
            "include_priority".to_string(),
            json!(metadata_config.include_priority),
        );
        data.insert(
            "include_created".to_string(),
            json!(metadata_config.include_created),
        );
        data.insert(
            "include_last_updated".to_string(),
            json!(metadata_config.include_last_updated),
        );
        data.insert(
            "custom_fields".to_string(),
            json!(metadata_config.custom_fields),
        );

        self.template_engine.render_use_case(&data)
    }

    fn generate_test_file(&self, use_case: &UseCase) -> Result<()> {
        // Only generate tests if explicitly enabled and scenarios exist
        if !self.config.generation.auto_generate_tests {
            return Ok(());
        }

        if use_case.scenarios.is_empty() {
            // No scenarios = no tests generated
            return Ok(());
        }

        // Check if we support the configured language
        if !self
            .template_engine
            .has_test_template(&self.config.generation.test_language)
        {
            println!(
                "Warning: Test language '{}' not supported, skipping test generation",
                self.config.generation.test_language
            );
            return Ok(());
        }

        let test_dir =
            Path::new(&self.config.directories.test_dir).join(to_snake_case(&use_case.category));
        fs::create_dir_all(&test_dir)?;

        // Generate file extension based on language
        let language_registry = LanguageRegistry::new();
        let file_extension = language_registry
            .get(&self.config.generation.test_language)
            .map(|lang| lang.file_extension())
            .unwrap_or("txt"); // fallback

        let test_file_name = format!("{}.{}", to_snake_case(&use_case.id), file_extension);
        let test_path = test_dir.join(test_file_name);

        let mut data = HashMap::new();
        data.insert("id".to_string(), json!(use_case.id));
        data.insert("title".to_string(), json!(use_case.title));
        data.insert(
            "title_snake_case".to_string(),
            json!(to_snake_case(&use_case.title)),
        );
        data.insert("description".to_string(), json!(use_case.description));
        data.insert("category".to_string(), json!(use_case.category));
        data.insert(
            "category_snake_case".to_string(),
            json!(to_snake_case(&use_case.category)),
        );
        data.insert(
            "test_module_name".to_string(),
            json!(to_snake_case(&use_case.id)),
        );
        data.insert(
            "generated_at".to_string(),
            json!(Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()),
        );

        // Prepare scenarios data with snake_case IDs
        let scenarios_data: Vec<Value> = use_case
            .scenarios
            .iter()
            .map(|scenario| {
                json!({
                    "id": scenario.id,
                    "snake_case_id": to_snake_case(&scenario.id),
                    "title": scenario.title,
                    "description": scenario.description,
                    "status": scenario.status.to_string(),
                    "preconditions": Vec::<String>::new(), // TODO: Add when scenario model supports it
                    "steps": Vec::<String>::new() // TODO: Add when scenario model supports it
                })
            })
            .collect();

        data.insert("scenarios".to_string(), json!(scenarios_data));

        // Check if file exists and handle smart documentation
        if test_path.exists() {
            let existing_content = fs::read_to_string(&test_path)?;
            let updated_content = self.merge_test_documentation(&existing_content, &data)?;
            fs::write(&test_path, updated_content)?;
            println!(
                "Updated test documentation: {}",
                test_path.display().to_string().cyan()
            );
        } else {
            // Generate new file using language-specific template
            let test_content = self
                .template_engine
                .render_test(&self.config.generation.test_language, &data)?;
            fs::write(&test_path, test_content)?;
            println!(
                "Generated test file: {}",
                test_path.display().to_string().cyan()
            );
        }

        Ok(())
    }

    /// Merge new documentation with existing test file, preserving user content between implementation markers
    fn merge_test_documentation(
        &self,
        existing_content: &str,
        data: &HashMap<String, Value>,
    ) -> Result<String> {
        const START_MARKER: &str = "// START USER IMPLEMENTATION";
        const END_MARKER: &str = "// END USER IMPLEMENTATION";

        // If overwrite_test_documentation is false, preserve the entire file
        if !self.config.generation.overwrite_test_documentation {
            println!("⚠️  overwrite_test_documentation=false, preserving existing test file");
            return Ok(existing_content.to_string());
        }

        // Generate new template content
        let new_template = self
            .template_engine
            .render_test(&self.config.generation.test_language, data)?;

        // Extract user implementations from existing content
        let mut user_implementations = std::collections::HashMap::new();
        let mut current_pos = 0;

        while let Some(start_pos) = existing_content[current_pos..].find(START_MARKER) {
            let absolute_start = current_pos + start_pos;

            // Find the end of the start marker line
            if let Some(start_line_end) = existing_content[absolute_start..].find('\n') {
                let impl_start = absolute_start + start_line_end + 1;

                // Find the corresponding end marker
                if let Some(end_pos) = existing_content[impl_start..].find(END_MARKER) {
                    let impl_end = impl_start + end_pos;

                    // Extract the user implementation
                    let user_impl = existing_content[impl_start..impl_end].trim_end();

                    // Try to identify which test function this belongs to by looking backwards
                    let before_start = &existing_content[..absolute_start];
                    if let Some(fn_match) = before_start.rfind("fn test_") {
                        if let Some(fn_end) = existing_content[fn_match..absolute_start].find('(') {
                            let fn_name = &existing_content[fn_match + 3..fn_match + fn_end]; // +3 to skip "fn "
                            user_implementations.insert(fn_name.to_string(), user_impl.to_string());
                        }
                    }

                    current_pos = impl_end;
                } else {
                    break; // No matching end marker found
                }
            } else {
                break; // No line end found
            }
        }

        // Replace user implementations in the new template
        let mut result = new_template;
        for (fn_name, user_impl) in user_implementations {
            // Create pattern to find the default implementation for this function
            let start_pattern = format!("fn {}(", fn_name);
            if let Some(fn_pos) = result.find(&start_pattern) {
                // Find the start marker for this function
                if let Some(start_marker_pos) = result[fn_pos..].find(START_MARKER) {
                    let absolute_start_marker = fn_pos + start_marker_pos;

                    // Find the start of implementation (after the marker line)
                    if let Some(start_line_end) = result[absolute_start_marker..].find('\n') {
                        let impl_start = absolute_start_marker + start_line_end + 1;

                        // Find the end marker
                        if let Some(end_marker_pos) = result[impl_start..].find(END_MARKER) {
                            let impl_end = impl_start + end_marker_pos;

                            // Replace the default implementation with user implementation
                            let before = &result[..impl_start];
                            let after = &result[impl_end..];
                            result = format!("{}{}\n        {}", before, user_impl, after);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn parse_use_case_from_markdown(&self, content: &str) -> Result<Option<UseCase>> {
        // Check if file has YAML frontmatter
        if content.starts_with("---\n") {
            return self.parse_use_case_with_frontmatter(content);
        }

        // Fallback to old format parsing
        self.parse_use_case_legacy_format(content)
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

    fn parse_use_case_legacy_format(&self, content: &str) -> Result<Option<UseCase>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut use_case = None;

        for (i, line) in lines.iter().enumerate() {
            if let Some(stripped) = line.strip_prefix("# ") {
                let title = stripped.trim().to_string();

                // Look for metadata in the following lines
                let mut id = String::new();
                let mut category = String::new();
                let mut description = String::new();
                let mut scenarios = Vec::new();

                // Parse the structured metadata
                for j in i + 1..lines.len() {
                    let metadata_line = lines[j];
                    if metadata_line.starts_with("**ID:**") {
                        id = metadata_line.replace("**ID:**", "").trim().to_string();
                    } else if metadata_line.starts_with("**Category:**") {
                        category = metadata_line
                            .replace("**Category:**", "")
                            .trim()
                            .to_string();
                    } else if metadata_line.starts_with("## Description") {
                        // Get description from next non-empty line
                        if j + 2 < lines.len() {
                            description = lines[j + 2].trim().to_string();
                        }
                    } else if metadata_line.starts_with("## Scenarios") {
                        // Parse scenarios section
                        scenarios = self.parse_scenarios_from_markdown(&lines[j + 1..])?;
                        break;
                    }
                }

                if !id.is_empty() && !category.is_empty() {
                    let mut uc = UseCase::new(id, title, category, description);
                    for scenario in scenarios {
                        uc.add_scenario(scenario);
                    }
                    use_case = Some(uc);
                    break;
                }
            }
        }

        Ok(use_case)
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
