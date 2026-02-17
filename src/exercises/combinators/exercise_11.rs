//! Exercise 11: Result::or_else - Error recovery
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use or_else for error recovery
//! - Provide fallback computations
//! - Handle multiple strategies

/// Try primary parser, fallback to default value.
pub fn parse_or_default(s: &str, default: i32) -> Result<i32, String> {
    todo!("Implement parse_or_default")
}

/// Try parsing as i32, fallback to parsing length.
pub fn parse_or_length(s: &str) -> Result<i32, String> {
    todo!("Implement parse_or_length")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42", 0), Ok(42));
        assert_eq!(parse_or_default("abc", 10), Ok(10));
        assert_eq!(parse_or_default("", 5), Ok(5));
    }

    #[test]
    fn test_parse_or_length() {
        assert_eq!(parse_or_length("42"), Ok(42));
        assert_eq!(parse_or_length("abc"), Ok(3));
        assert_eq!(parse_or_length("hello"), Ok(5));
        assert_eq!(parse_or_length(""), Ok(0));
    }
}
