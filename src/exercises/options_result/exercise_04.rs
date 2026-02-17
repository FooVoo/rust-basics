//! Exercise 04: Option unwrap_or
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use unwrap_or() to provide default values
//! - Understand the difference between unwrap and unwrap_or
//! - Practice safe Option handling

/// Get a value from Option or return a default.
pub fn get_or_default(opt: Option<i32>, default: i32) -> i32 {
    todo!("Implement get_or_default")
}

/// Get the first element of a slice, or 0 if empty.
pub fn first_or_zero(numbers: &[i32]) -> i32 {
    todo!("Implement first_or_zero")
}

/// Parse a string to i32, returning 0 on failure.
pub fn parse_or_zero(s: &str) -> i32 {
    todo!("Implement parse_or_zero")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_default() {
        assert_eq!(get_or_default(Some(42), 0), 42);
        assert_eq!(get_or_default(None, 0), 0);
        assert_eq!(get_or_default(None, -1), -1);
    }

    #[test]
    fn test_first_or_zero() {
        assert_eq!(first_or_zero(&[1, 2, 3]), 1);
        assert_eq!(first_or_zero(&[]), 0);
        assert_eq!(first_or_zero(&[-5]), -5);
    }

    #[test]
    fn test_parse_or_zero() {
        assert_eq!(parse_or_zero("42"), 42);
        assert_eq!(parse_or_zero("abc"), 0);
        assert_eq!(parse_or_zero(""), 0);
        assert_eq!(parse_or_zero("-10"), -10);
    }
}
