// src/core/utils/mod.rs
mod fuzzy_match;
mod string_utils;

pub use fuzzy_match::suggest_alternatives;
pub use string_utils::{slugify_for_id, to_snake_case};
