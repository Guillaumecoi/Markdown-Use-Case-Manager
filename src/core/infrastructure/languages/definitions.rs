use crate::core::infrastructure::languages::LanguageDefinition;

/// Rust language definition
pub const RUST: LanguageDefinition = LanguageDefinition::new(
    "rust",
    &["rs"],
    "rs",
    include_str!("../../../../source-templates/languages/rust/test.hbs"),
);

/// Python language definition
pub const PYTHON: LanguageDefinition = LanguageDefinition::new(
    "python",
    &["py"],
    "py",
    include_str!("../../../../source-templates/languages/python/test.hbs"),
);

/// JavaScript language definition
pub const JAVASCRIPT: LanguageDefinition = LanguageDefinition::new(
    "javascript",
    &["js"],
    "js",
    include_str!("../../../../source-templates/languages/javascript/test.hbs"),
);
