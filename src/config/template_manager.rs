// src/config/template_manager.rs - Template file handling and processing
use crate::config::types::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TemplateManager;

impl TemplateManager {
    /// Create config file from template
    pub fn create_config_from_template(config: &Config) -> Result<()> {
        // Serialize the config to TOML instead of copying the template
        // This ensures the user's chosen language and methodology are saved
        let config_content =
            toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;

        // Write the config
        let config_path = Config::config_path();
        fs::write(&config_path, config_content).context("Failed to write config file")?;

        Ok(())
    }

    /// Find the source templates directory
    pub fn find_source_templates_dir() -> Result<PathBuf> {
        // Try current directory first
        let local_templates = Path::new("source-templates");
        if local_templates.exists() {
            return Ok(local_templates.to_path_buf());
        }

        // Try CARGO_MANIFEST_DIR (set during cargo test and build)
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let cargo_templates = Path::new(&manifest_dir).join("source-templates");
            if cargo_templates.exists() {
                return Ok(cargo_templates);
            }
        }

        // If still not found, try to find templates relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Check ../../source-templates (when running from target/release/)
                let dev_templates = exe_dir
                    .parent()
                    .and_then(|p| p.parent())
                    .map(|p| p.join("source-templates"));
                if let Some(dev_templates) = dev_templates {
                    if dev_templates.exists() {
                        return Ok(dev_templates);
                    }
                }
            }
        }

        anyhow::bail!("Source templates directory not found. Run from project root or ensure source-templates/ exists.")
    }

    /// Copy templates to .config/.mucm/handlebars/ with methodologies and languages
    pub fn copy_templates_to_config(base_dir: &str) -> Result<()> {
        let base_path = Path::new(base_dir);
        let config_templates_dir = base_path
            .join(Config::CONFIG_DIR)
            .join(Config::TEMPLATES_DIR);
        let config_methodologies_dir = base_path.join(Config::CONFIG_DIR).join("methodologies");

        // Create directories
        fs::create_dir_all(&config_templates_dir)
            .context("Failed to create config templates directory")?;
        fs::create_dir_all(&config_methodologies_dir)
            .context("Failed to create config methodologies directory")?;

        // Load the config from base_dir to see which methodologies to import
        let config_path = base_path.join(Config::CONFIG_DIR).join("mucm.toml");
        if !config_path.exists() {
            anyhow::bail!(
                "Config file not found at {:?} - run 'mucm init' first",
                config_path
            );
        }
        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        let source_templates_dir = Self::find_source_templates_dir()?;

        // Copy root template files
        Self::copy_root_templates(&source_templates_dir, &config_templates_dir)?;

        // Copy methodologies
        Self::copy_methodologies(
            &source_templates_dir,
            &config,
            &config_templates_dir,
            &config_methodologies_dir,
        )?;

        // Copy language templates
        Self::copy_language_templates(&source_templates_dir, &config, &config_templates_dir)?;

        Ok(())
    }

    /// Copy root template files (overview.hbs, etc.)
    fn copy_root_templates(source_templates_dir: &Path, config_templates_dir: &Path) -> Result<()> {
        // Copy overview.hbs
        let overview_src = source_templates_dir.join("overview.hbs");
        if overview_src.exists() {
            let overview_dst = config_templates_dir.join("overview.hbs");
            fs::copy(&overview_src, &overview_dst)?;
            println!("✓ Copied overview template");
        }

        Ok(())
    }

    /// Copy methodology templates and configs
    fn copy_methodologies(
        source_templates_dir: &Path,
        config: &Config,
        config_templates_dir: &Path,
        config_methodologies_dir: &Path,
    ) -> Result<()> {
        let source_methodologies = source_templates_dir.join("methodologies");
        if !source_methodologies.exists() {
            anyhow::bail!(
                "Source methodologies directory not found at {:?}",
                source_methodologies
            );
        }

        for methodology in &config.templates.methodologies {
            let source_method_dir = source_methodologies.join(methodology);
            if !source_method_dir.exists() {
                anyhow::bail!(
                    "Methodology '{}' not found in source-templates/methodologies/. \
                     Available methodologies should be in source-templates/methodologies/{{name}}/ directories.",
                    methodology
                );
            }

            // Copy methodology templates to handlebars/{methodology}/ (skip config.toml files)
            let target_method_templates = config_templates_dir.join(methodology);
            Self::copy_dir_recursive_skip_config(&source_method_dir, &target_method_templates)?;

            // Copy methodology config.toml to methodologies/{methodology}.toml
            let source_config = source_method_dir.join("config.toml");
            if source_config.exists() {
                let target_config = config_methodologies_dir.join(format!("{}.toml", methodology));
                fs::copy(&source_config, &target_config)?;
                println!("✓ Copied methodology: {}", methodology);
            } else {
                anyhow::bail!(
                    "Methodology '{}' is missing config.toml file at {:?}",
                    methodology,
                    source_config
                );
            }
        }

        Ok(())
    }

    /// Copy language templates
    fn copy_language_templates(
        source_templates_dir: &Path,
        config: &Config,
        config_templates_dir: &Path,
    ) -> Result<()> {
        let source_languages = source_templates_dir.join("languages");
        if !source_languages.exists() {
            return Ok(()); // Languages are optional
        }

        let source_lang_dir = source_languages.join(&config.templates.test_language);
        if source_lang_dir.exists() {
            let target_languages = config_templates_dir.join("languages");
            let target_lang_dir = target_languages.join(&config.templates.test_language);
            Self::copy_dir_recursive(&source_lang_dir, &target_lang_dir)?;
            println!(
                "✓ Copied language templates: {}",
                config.templates.test_language
            );
        } else {
            println!(
                "⚠ Language '{}' not found in source-templates/languages/, skipping",
                config.templates.test_language
            );
        }

        Ok(())
    }

    /// Recursively copy a directory and all its contents
    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    /// Recursively copy a directory but skip config.toml files (used for methodology template copying)
    fn copy_dir_recursive_skip_config(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let file_name = entry.file_name();
            let dst_path = dst.join(&file_name);

            // Skip config.toml files - these are handled separately
            if file_name == "config.toml" {
                continue;
            }

            if src_path.is_dir() {
                Self::copy_dir_recursive_skip_config(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}
