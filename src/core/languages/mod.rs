//! Modular language support system
//!
//! This module provides a unified approach to programming language support.
//! All languages use the same `LanguageDefinition` struct with different configuration.

use std::collections::HashMap;
use std::rc::Rc;

/// Trait defining the interface for programming language support
pub trait Language {
    /// Language identifier (e.g., "rust", "python", "javascript")
    fn name(&self) -> &'static str;

    /// Alternative names/aliases for this language (e.g., "js" for "javascript")
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }

    /// File extension for test files (e.g., "rs", "py", "js")
    fn file_extension(&self) -> &'static str;

    /// Template content for test files
    fn test_template(&self) -> &'static str;

    /// Get all names this language responds to (name + aliases)
    fn all_names(&self) -> Vec<&'static str> {
        let mut names = vec![self.name()];
        names.extend_from_slice(self.aliases());
        names
    }
}

/// Unified language definition struct used by all programming languages
pub struct LanguageDefinition {
    name: &'static str,
    aliases: &'static [&'static str],
    file_extension: &'static str,
    test_template: &'static str,
}

impl LanguageDefinition {
    /// Create a new language definition
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

/// Registry for managing available languages
pub struct LanguageRegistry {
    languages: HashMap<String, Rc<dyn Language>>,
}

impl std::fmt::Debug for LanguageRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageRegistry")
            .field("languages", &self.languages.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl LanguageRegistry {
    /// Create a new language registry with all built-in languages
    pub fn new() -> Self {
        let mut registry = Self {
            languages: HashMap::new(),
        };

        // Register built-in languages using definitions from separate file
        registry.register(Box::new(definitions::RUST));
        registry.register(Box::new(definitions::PYTHON));
        registry.register(Box::new(definitions::JAVASCRIPT));

        registry
    }

    /// Register a new language
    pub fn register(&mut self, language: Box<dyn Language>) {
        let language_rc: Rc<dyn Language> = Rc::from(language);

        for name in language_rc.all_names() {
            self.languages.insert(name.to_string(), language_rc.clone());
        }
    }

    /// Get language by name or alias
    pub fn get(&self, name: &str) -> Option<&dyn Language> {
        self.languages.get(name).map(|rc| rc.as_ref())
    }

    /// Get all available language names (primary names only)
    pub fn available_languages(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .languages
            .values()
            .map(|lang| lang.name().to_string())
            .collect();
        names.sort();
        names.dedup();
        names
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Language definitions
pub mod definitions;
