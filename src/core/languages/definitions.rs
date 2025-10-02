use crate::core::languages::LanguageDefinition;

/// Rust language definition
pub const RUST: LanguageDefinition = LanguageDefinition::new(
    "rust",
    &[],
    "rs",
    include_str!("../../../templates/languages/rust/test.hbs"),
    true, // uses legacy directory
);

/// Python language definition
pub const PYTHON: LanguageDefinition = LanguageDefinition::new(
    "python",
    &["py"],
    "py",
    include_str!("../../../templates/languages/python/test.hbs"),
    true, // uses legacy directory
);

/// JavaScript language definition
pub const JAVASCRIPT: LanguageDefinition = LanguageDefinition::new(
    "javascript",
    &["js"],
    "js",
    include_str!("../../../templates/languages/javascript/test.hbs"),
    false, // uses new directory format
);