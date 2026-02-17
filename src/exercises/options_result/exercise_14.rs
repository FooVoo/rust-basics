//! Exercise 14: Result and_then
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use and_then() to chain Result operations
//! - Avoid nested Results
//! - Compose fallible operations

/// Parse a string and validate it's positive.
pub fn parse_positive(s: &str) -> Result<i32, String> {
    todo!("Implement parse_positive")
}

/// Parse and divide in one chain.
pub fn parse_and_safe_divide(a: &str, b: &str) -> Result<i32, String> {
    todo!("Implement parse_and_safe_divide")
}

/// Parse a number and check if it's even.
pub fn parse_even(s: &str) -> Result<i32, String> {
    todo!("Implement parse_even")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert!(parse_positive("0").is_err());
        assert!(parse_positive("-5").is_err());
        assert!(parse_positive("abc").is_err());
    }

    #[test]
    fn test_parse_and_safe_divide() {
        assert_eq!(parse_and_safe_divide("10", "2"), Ok(5));
        assert!(parse_and_safe_divide("10", "0").is_err());
        assert!(parse_and_safe_divide("abc", "2").is_err());
        assert!(parse_and_safe_divide("10", "xyz").is_err());
    }

    #[test]
    fn test_parse_even() {
        assert_eq!(parse_even("42"), Ok(42));
        assert!(parse_even("43").is_err());
        assert!(parse_even("abc").is_err());
    }
}
