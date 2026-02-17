//! Exercise 13: Result Mapping - Transform Result values
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use map and map_err to transform Results
//! - Chain transformations
//! - Convert between error types

/// Parse a string to a number and double it.
pub fn parse_and_double(s: &str) -> Result<i32, String> {
    todo!("Implement parse_and_double")
}

/// Parse a string to a number, double it, and convert to string.
pub fn parse_double_stringify(s: &str) -> Result<String, String> {
    todo!("Implement parse_double_stringify")
}

/// Parse and transform with a custom function.
pub fn parse_and_transform<F>(s: &str, f: F) -> Result<i32, String>
where
    F: FnOnce(i32) -> i32,
 {
    todo!("Parse and transform with a custom function.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double_success() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert_eq!(parse_and_double("100"), Ok(200));
        assert_eq!(parse_and_double("-3"), Ok(-6));
        assert_eq!(parse_and_double("0"), Ok(0));
    }

    #[test]
    fn test_parse_and_double_failure() {
        let result = parse_and_double("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Parse error"));
    }

    #[test]
    fn test_parse_double_stringify_success() {
        assert_eq!(parse_double_stringify("5"), Ok("10".to_string()));
        assert_eq!(parse_double_stringify("21"), Ok("42".to_string()));
        assert_eq!(parse_double_stringify("0"), Ok("0".to_string()));
    }

    #[test]
    fn test_parse_double_stringify_failure() {
        let result = parse_double_stringify("xyz");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed"));
    }

    #[test]
    fn test_parse_and_transform_square() {
        assert_eq!(parse_and_transform("5", |n| n * n), Ok(25));
        assert_eq!(parse_and_transform("10", |n| n * n), Ok(100));
    }

    #[test]
    fn test_parse_and_transform_add() {
        assert_eq!(parse_and_transform("5", |n| n + 10), Ok(15));
        assert_eq!(parse_and_transform("0", |n| n + 100), Ok(100));
    }

    #[test]
    fn test_parse_and_transform_identity() {
        assert_eq!(parse_and_transform("42", |n| n), Ok(42));
    }

    #[test]
    fn test_parse_and_transform_failure() {
        assert!(parse_and_transform("bad", |n| n * 2).is_err());
    }
}
