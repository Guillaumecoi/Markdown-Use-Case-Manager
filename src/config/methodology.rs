// src/config/methodology.rs - Methodology-specific configuration handling
use crate::config::file_manager::ConfigFileManager;
use anyhow::Result;
use std::fs;

pub struct MethodologyManager;

impl MethodologyManager {
    /// Get methodology-specific recommendations as a human-readable string
    pub fn get_recommendations(methodology: &str) -> String {
        match methodology {
            "business" => {
                "Business Methodology Recommendations:
- Focus on business value and stakeholder needs
- Business-oriented language and structure
- Emphasis on ROI and business outcomes
- Best for: Business analysts, product managers, stakeholder documentation".to_string()
            },
            "developer" => {
                "Developer Methodology Recommendations:
- Technical implementation focus
- System behavior and API documentation
- Code-centric perspective
- Best for: Development teams, technical documentation, API design".to_string()
            },
            "feature" => {
                "Feature Methodology Recommendations:
- Feature-oriented documentation
- User story and epic integration
- Agile-friendly structure
- Best for: Product development, agile teams, feature tracking".to_string()
            },
            "testing" => {
                "Testing Methodology Recommendations:
- Test-focused documentation
- Test scenarios and coverage tracking
- Quality assurance emphasis
- Best for: QA teams, test automation, quality metrics".to_string()
            },
            _ => "Unknown methodology. Using developer methodology defaults.".to_string()
        }
    }

    /// Get list of available methodologies (those with config files)
    pub fn list_available() -> Result<Vec<String>> {
        let methodologies_dir = ConfigFileManager::find_config_dir()?
            .join("methodologies");

        if !methodologies_dir.exists() {
            return Ok(Vec::new());
        }

        let mut methodologies = Vec::new();
        for entry in fs::read_dir(&methodologies_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Files are named {methodology}.toml
                    methodologies.push(name.to_string());
                }
            }
        }

        methodologies.sort();
        Ok(methodologies)
    }
}