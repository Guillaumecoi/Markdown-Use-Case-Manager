// Language definitions infrastructure

mod definitions;

pub(crate) use definitions::{JAVASCRIPT, PYTHON, RUST};

use std::collections::HashMap;

/// Trait representing a programming language
pub trait Language {
    fn name(&self) -> &'static str;
    fn aliases(&self) -> &'static [&'static str];
    /// Get the file extension for this language (e.g., "rs", "py", "js")
    /// TODO: Use this when generating test files to determine correct extension
    #[allow(dead_code)]
    fn file_extension(&self) -> &'static str;
    fn test_template(&self) -> &'static str;
}

/// Static language definition
#[derive(Debug, Clone)]
pub struct LanguageDefinition {
    name: &'static str,
    aliases: &'static [&'static str],
    test_template: &'static str,
    /// File extension for test files
    /// TODO: Use this field when test generation is re-implemented
    #[allow(dead_code)]
    file_extension: &'static str,
}

impl LanguageDefinition {
    pub const fn new(
        name: &'static str,
        aliases: &'static [&'static str],
        file_extension: &'static str,
        test_template: &'static str,
    ) -> Self {
        Self {
            name,
            aliases,
            file_extension,
            test_template,
        }
    }
}

impl Language for LanguageDefinition {
    fn name(&self) -> &'static str {
        self.name
    }

    fn aliases(&self) -> &'static [&'static str] {
        self.aliases
    }

    fn file_extension(&self) -> &'static str {
        self.file_extension
    }

    fn test_template(&self) -> &'static str {
        self.test_template
    }
}

/// Registry of all available programming languages
pub struct LanguageRegistry {
    languages: HashMap<String, &'static LanguageDefinition>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut languages = HashMap::new();

        // Register all languages
        for lang in &[&RUST, &PYTHON, &JAVASCRIPT] {
            languages.insert(lang.name().to_string(), *lang);
            for alias in lang.aliases() {
                languages.insert(alias.to_string(), *lang);
            }
        }

        Self { languages }
    }

    pub fn get(&self, name: &str) -> Option<&'static dyn Language> {
        self.languages
            .get(&name.to_lowercase())
            .map(|lang| *lang as &'static dyn Language)
    }

    pub fn available_languages(&self) -> Vec<String> {
        vec![
            RUST.name().to_string(),
            PYTHON.name().to_string(),
            JAVASCRIPT.name().to_string(),
        ]
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}
