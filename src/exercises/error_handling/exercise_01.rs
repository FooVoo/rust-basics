//! Exercise 01: Basic Result - Parse a string to integer
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Result<T, E> type
//! - Work with parse() method
//! - Convert errors to strings

/// Parse a string to an i32.
/// Return Ok(value) if successful, Err(message) otherwise.
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_success() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("-10"), Ok(-10));
        assert_eq!(parse_number("0"), Ok(0));
    }

    #[test]
    fn test_parse_number_failure() {
        assert!(parse_number("abc").is_err());
        assert!(parse_number("").is_err());
        assert!(parse_number("12.5").is_err());
    }
}
