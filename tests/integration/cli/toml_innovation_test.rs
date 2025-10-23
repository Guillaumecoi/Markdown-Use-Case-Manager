// Comprehensive CLI tests showcasing TOML metadata innovation
// DISABLED: These tests rely on hardcoded methodologies (feature, business, developer, tester)
// which have been removed in favor of a 100% configuration-driven system.
// See docs/HARDCODED_REMOVAL_SUMMARY.md for details.

/*
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;
use std::fs;

#[cfg(test)]
mod cli_toml_innovation_tests {
    use super::*;

    fn setup_test_project() -> TempDir {
        let temp_dir = TempDir::new().expect("Should create temp directory");
        
        // Initialize project
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&temp_dir)
            .arg("init")
            .assert()
            .success();
            
        temp_dir
    }

    #[test]
    fn test_innovative_methodology_switching() {
        let temp_dir = setup_test_project();
        
        // Test all 4 methodologies are available
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&temp_dir)
            .arg("methodologies")
            .assert()
            .success()
            .stdout(predicate::str::contains("business"))
            .stdout(predicate::str::contains("feature"))
            .stdout(predicate::str::contains("developer"))
            .stdout(predicate::str::contains("tester"));

        // Create use cases with different methodologies to showcase innovation
        let use_cases = vec![
            ("feature", "User Experience Enhancement", "UX"),
            ("business", "Revenue Optimization", "Business"),
            ("developer", "API Performance Improvement", "Technical"),
            ("tester", "Quality Assurance Framework", "QA"),
        ];

        for (methodology, title, category) in &use_cases {
            let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
            cmd.current_dir(&temp_dir)
                .arg("create")
                .arg("--methodology")
                .arg(methodology)
                .arg("--category")
                .arg(category)
                .arg(title)
                .assert()
                .success()
                .stdout(predicate::str::contains(format!("with {} methodology", methodology)));
        }

        // Verify use cases were created with methodology-specific templates
        let use_case_dirs = ["ux", "business", "technical", "qa"];
        for (i, dir) in use_case_dirs.iter().enumerate() {
            let use_case_path = temp_dir.path()
                .join("docs")
                .join("use-cases")
                .join(dir);
            assert!(use_case_path.exists(), "Use case directory {} should exist", dir);
            
            // Find the generated markdown file
            let files: Vec<_> = fs::read_dir(&use_case_path)
                .expect("Should read directory")
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
                .collect();
            
            assert!(!files.is_empty(), "Should have at least one markdown file in {}", dir);
            
            // Verify the content matches the methodology
            let file_path = files[0].path();
            let content = fs::read_to_string(&file_path)
                .expect("Should read use case file");
            
            match use_cases[i].0 {
                "feature" => {
                    assert!(content.contains("Value"), "Feature methodology should contain Value section");
                    assert!(content.contains("Users"), "Feature methodology should contain Users section");
                    assert!(content.contains("Success"), "Feature methodology should contain Success section");
                },
                "business" => {
                    assert!(content.contains("Business"), "Business methodology should contain business content");
                },
                "developer" => {
                    assert!(content.contains("Technical") || content.contains("Implementation"), 
                        "Developer methodology should contain technical content");
                },
                "tester" => {
                    assert!(content.contains("Test") || content.contains("Quality"), 
                        "Tester methodology should contain testing content");
                },
                _ => {}
            }
        }
    }

    #[test]
    fn test_cli_template_generation_innovation() {
        let temp_dir = setup_test_project();
        
        // Create a use case
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&temp_dir)
            .arg("create")
            .arg("--methodology")
            .arg("feature")
            .arg("--category")
            .arg("Innovation")
            .arg("Template Generation Innovation")
            .assert()
            .success();

        // Verify the use case was created
        let use_case_files: Vec<_> = fs::read_dir(temp_dir.path().join("docs").join("use-cases").join("innovation"))
            .expect("Should read innovation directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
            .collect();
        
        assert_eq!(use_case_files.len(), 1, "Should have exactly one use case file");
        
        let use_case_content = fs::read_to_string(use_case_files[0].path())
            .expect("Should read use case content");
        
        // Verify feature methodology specific content
        assert!(use_case_content.contains("Template Generation Innovation"));
        assert!(use_case_content.contains("UC-INN-001"));
        assert!(use_case_content.contains("## Value"));
        assert!(use_case_content.contains("## Users"));
        assert!(use_case_content.contains("## Success"));
        assert!(use_case_content.contains("**Target Users:**"));
    }

    #[test]
    fn test_cli_project_overview_generation() {
        let temp_dir = setup_test_project();
        
        // Create multiple use cases to test overview generation
        let use_cases = vec![
            ("feature", "User Dashboard", "Frontend"),
            ("business", "Market Analysis", "Strategy"),
            ("developer", "API Gateway", "Backend"),
            ("tester", "Automation Suite", "Testing"),
        ];

        for (methodology, title, category) in &use_cases {
            let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
            cmd.current_dir(&temp_dir)
                .arg("create")
                .arg("--methodology")
                .arg(methodology)
                .arg("--category")
                .arg(category)
                .arg(title)
                .assert()
                .success();
        }

        // Verify overview file was generated/updated
        let overview_path = temp_dir.path().join("docs").join("use-cases").join("README.md");
        
        if overview_path.exists() {
            let overview_content = fs::read_to_string(&overview_path)
                .expect("Should read overview content");
            
            // Verify overview contains structure and at least some use cases
            assert!(overview_content.contains("# Use Cases Overview") || overview_content.contains("Use Cases"));
            
            // Check for at least one of the use cases (they might be organized differently)
            let has_content = overview_content.contains("User Dashboard") ||
                            overview_content.contains("Market Analysis") ||
                            overview_content.contains("API Gateway") ||
                            overview_content.contains("Automation Suite");
            assert!(has_content, "Overview should contain at least one use case");
        } else {
            // If no overview exists, just verify use cases were created
            assert!(temp_dir.path().join("docs").join("use-cases").exists(), "Use cases directory should exist");
        }
    }

    #[test]
    fn test_cli_error_handling_innovation() {
        let temp_dir = setup_test_project();
        
        // Test invalid methodology
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&temp_dir)
            .arg("create")
            .arg("--methodology")
            .arg("invalid_methodology")
            .arg("--category")
            .arg("Test")
            .arg("Invalid Test")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Unknown methodology"));

        // Test missing required arguments
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&temp_dir)
            .arg("create")
            .arg("--methodology")
            .arg("feature")
            .assert()
            .failure()
            .stderr(predicate::str::contains("required"));

        // Test uninitialized project
        let uninit_temp_dir = TempDir::new().expect("Should create temp directory");
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&uninit_temp_dir)
            .arg("create")
            .arg("--methodology")
            .arg("feature")
            .arg("--category")
            .arg("Test")
            .arg("Should Fail")
            .assert()
            .failure()
            .stderr(predicate::str::contains("No markdown use case manager project found"));
    }

    #[test]
    fn test_cli_methodology_specific_output() {
        let temp_dir = setup_test_project();
        
        // Test each methodology produces different output
        let methodologies = ["feature", "business", "developer", "tester"];
        
        for methodology in &methodologies {
            let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
            let output = cmd.current_dir(&temp_dir)
                .arg("create")
                .arg("--methodology")
                .arg(methodology)
                .arg("--category")
                .arg("Test")
                .arg(format!("{} Test Case", methodology.to_uppercase()))
                .output()
                .expect("Should execute command");
            
            assert!(output.status.success(), "Command should succeed for methodology: {}", methodology);
            
            let stdout = String::from_utf8(output.stdout).expect("Should convert stdout");
            assert!(stdout.contains(&format!("with {} methodology", methodology)));
            
            // Verify methodology-specific use case ID prefixes
            let expected_prefix = match *methodology {
                "feature" => "FEA",
                "business" => "BUS", 
                "developer" => "TEC",
                "tester" => "QA",
                _ => "TES" // fallback
            };
            
            assert!(stdout.contains(&format!("UC-{}-", expected_prefix)) || 
                   stdout.contains("UC-TES-"), // Some might use TES prefix
                   "Should contain expected prefix for methodology: {}", methodology);
        }
    }

    #[test]
    fn test_cli_project_structure_innovation() {
        let temp_dir = setup_test_project();
        
        // Create use cases in different categories
        let test_cases = vec![
            ("feature", "Mobile App", "Frontend"),
            ("business", "Sales Strategy", "Business"),
            ("developer", "Microservices", "Architecture"),
            ("tester", "Performance Tests", "Quality"),
        ];

        for (methodology, title, category) in &test_cases {
            let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
            cmd.current_dir(&temp_dir)
                .arg("create")
                .arg("--methodology")
                .arg(methodology)
                .arg("--category")
                .arg(category)
                .arg(title)
                .assert()
                .success();
        }

        // Verify innovative directory structure
        let base_path = temp_dir.path().join("docs").join("use-cases");
        
        // Check category-based organization
        assert!(base_path.join("frontend").exists());
        assert!(base_path.join("business").exists());
        assert!(base_path.join("architecture").exists());
        assert!(base_path.join("quality").exists());
        
        // Verify each category has the expected use case
        for (_, title, category) in &test_cases {
            let category_path = base_path.join(category.to_lowercase());
            let files: Vec<_> = fs::read_dir(&category_path)
                .expect("Should read category directory")
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
                .collect();
            
            assert!(!files.is_empty(), "Category {} should have use case files", category);
            
            // Verify the content contains the title
            let content = fs::read_to_string(files[0].path())
                .expect("Should read file content");
            assert!(content.contains(title), "File should contain title: {}", title);
        }
    }

    #[test]
    fn test_cli_help_and_documentation() {
        // Test main help
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Markdown Use Case Manager"))
            .stdout(predicate::str::contains("create"))
            .stdout(predicate::str::contains("init"))
            .stdout(predicate::str::contains("methodologies"));

        // Test create command help
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.arg("create")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("--methodology"))
            .stdout(predicate::str::contains("--category"))
            .stdout(predicate::str::contains("developer"))
            .stdout(predicate::str::contains("business"))
            .stdout(predicate::str::contains("tester"));

        // Test methodologies command help  
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.arg("methodologies")
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("List available methodologies"));
    }

    #[test]
    fn test_cli_configuration_handling() {
        let temp_dir = setup_test_project();
        
        // Verify configuration file was created
        let config_path = temp_dir.path().join(".config").join(".mucm").join("mucm.toml");
        assert!(config_path.exists(), "Configuration file should exist");
        
        let config_content = fs::read_to_string(&config_path)
            .expect("Should read config file");
        
        // Verify configuration contains expected sections
        assert!(config_content.contains("use_case_dir"));
        assert!(config_content.contains("docs/use-cases"));
        
        // Create a use case and verify it respects configuration
        let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
        cmd.current_dir(&temp_dir)
            .arg("create")
            .arg("--methodology")
            .arg("feature")
            .arg("--category")
            .arg("Config")
            .arg("Configuration Test")
            .assert()
            .success();
        
        // Verify use case was created in configured directory
        let use_case_path = temp_dir.path().join("docs").join("use-cases").join("config");
        assert!(use_case_path.exists(), "Use case should be created in configured directory");
    }

    #[test]
    fn test_cli_concurrent_operations() {
        let temp_dir = setup_test_project();
        
        // Simulate multiple rapid use case creations
        let use_cases = vec![
            ("feature", "Rapid Test 1", "Concurrent"),
            ("business", "Rapid Test 2", "Concurrent"),
            ("developer", "Rapid Test 3", "Concurrent"),
            ("tester", "Rapid Test 4", "Concurrent"),
        ];

        // Create all use cases rapidly
        for (methodology, title, category) in &use_cases {
            let mut cmd = Command::cargo_bin("mucm").expect("Should find binary");
            cmd.current_dir(&temp_dir)
                .arg("create")
                .arg("--methodology")
                .arg(methodology)
                .arg("--category")
                .arg(category)
                .arg(title)
                .assert()
                .success();
        }

        // Verify all use cases were created correctly
        let concurrent_path = temp_dir.path().join("docs").join("use-cases").join("concurrent");
        assert!(concurrent_path.exists());
        
        let files: Vec<_> = fs::read_dir(&concurrent_path)
            .expect("Should read concurrent directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
            .collect();
        
        assert_eq!(files.len(), 4, "Should have created 4 use case files");
        
        // Verify unique IDs were generated
        let mut ids = Vec::new();
        for file in &files {
            let content = fs::read_to_string(file.path())
                .expect("Should read file");
            
            // Extract ID from content
            if let Some(id_line) = content.lines().find(|line| line.contains("**ID:**")) {
                if let Some(id) = id_line.split("**ID:**").nth(1) {
                    let clean_id = id.split('|').next().unwrap().trim();
                    ids.push(clean_id.to_string());
                }
            }
        }
        
        assert_eq!(ids.len(), 4, "Should extract 4 IDs");
        
        // Verify all IDs are unique
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 4, "All IDs should be unique");
    }
}
*/
