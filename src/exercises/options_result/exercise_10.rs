//! Exercise 10: Result unwrap and expect
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use unwrap() on Result types
//! - Use expect() with custom error messages
//! - Understand panic behavior with Result

/// Parse a known-good number string using unwrap.
pub fn parse_known_good(s: &str) -> i32 {
    todo!("Implement parse_known_good")
}

/// Parse a number with a descriptive error using expect.
pub fn parse_with_context(s: &str) -> i32 {
    todo!("Implement parse_with_context")
}

/// Safely extract the Ok value if Result is Ok, panic otherwise.
pub fn extract_ok<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    todo!("Implement extract_ok")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_known_good() {
        assert_eq!(parse_known_good("42"), 42);
        assert_eq!(parse_known_good("-10"), -10);
    }

    #[test]
    #[should_panic]
    fn test_parse_known_good_panics() {
        parse_known_good("not a number");
    }

    #[test]
    fn test_parse_with_context() {
        assert_eq!(parse_with_context("42"), 42);
    }

    #[test]
    #[should_panic(expected = "String should contain a valid number")]
    fn test_parse_with_context_panics() {
        parse_with_context("abc");
    }

    #[test]
    fn test_extract_ok() {
        assert_eq!(extract_ok(Ok::<i32, String>(42)), 42);
    }

    #[test]
    #[should_panic]
    fn test_extract_ok_panics() {
        extract_ok(Err::<i32, String>(String::from("error")));
    }
}
