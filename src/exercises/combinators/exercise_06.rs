//! Exercise 06: Result::map_err - Transform error values
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Result::map_err combinator
//! - Transform Err values while preserving Ok
//! - Convert error types

/// Parse a string and convert error to custom message.
pub fn parse_with_custom_error(s: &str) -> Result<i32, String> {
    todo!("Implement parse_with_custom_error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        assert_eq!(parse_with_custom_error("42"), Ok(42));
        assert_eq!(parse_with_custom_error("-10"), Ok(-10));
    }

    #[test]
    fn test_parse_custom_error() {
        assert_eq!(
            parse_with_custom_error("abc"),
            Err("Failed to parse 'abc' as integer".to_string())
        );
        assert_eq!(
            parse_with_custom_error("12.5"),
            Err("Failed to parse '12.5' as integer".to_string())
        );
    }
}
