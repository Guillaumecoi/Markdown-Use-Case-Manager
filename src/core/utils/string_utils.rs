// String utility functions

/// Converts a string to snake_case format.
///
/// This function takes any string and converts it to snake_case by:
/// 1. Converting to lowercase
/// 2. Replacing non-alphanumeric characters with underscores
/// 3. Removing consecutive underscores
/// 4. Removing leading/trailing underscores
///
/// # Examples
///
/// - `"Hello World"` → `"hello_world"`
/// - `"UC-TEST-001"` → `"uc_test_001"`
/// - `"some__value"` → `"some_value"`
pub fn to_snake_case(s: &str) -> String {
    // First convert to lowercase and replace special characters with underscores
    let cleaned = s
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

    // Remove multiple consecutive underscores and clean up
    cleaned
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_simple() {
        assert_eq!(to_snake_case("Hello World"), "hello_world");
        assert_eq!(to_snake_case("HelloWorld"), "helloworld");
    }

    #[test]
    fn test_to_snake_case_with_hyphens() {
        assert_eq!(to_snake_case("UC-TEST-001"), "uc_test_001");
        assert_eq!(to_snake_case("my-value"), "my_value");
    }

    #[test]
    fn test_to_snake_case_multiple_underscores() {
        assert_eq!(to_snake_case("some__value"), "some_value");
        assert_eq!(to_snake_case("test___case"), "test_case");
    }

    #[test]
    fn test_to_snake_case_special_characters() {
        assert_eq!(to_snake_case("hello@world!"), "hello_world");
        assert_eq!(to_snake_case("test#case$value"), "test_case_value");
    }

    #[test]
    fn test_to_snake_case_already_snake_case() {
        assert_eq!(to_snake_case("already_snake_case"), "already_snake_case");
    }

    #[test]
    fn test_to_snake_case_empty() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn test_to_snake_case_numbers() {
        assert_eq!(to_snake_case("test123"), "test123");
        assert_eq!(to_snake_case("123test"), "123test");
    }
}
