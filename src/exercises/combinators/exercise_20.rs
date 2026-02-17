//! Exercise 20: Result::unwrap_or_else - Recover from errors
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Recover from Result errors
//! - Use error information in fallback
//! - Handle errors gracefully

/// Parse with fallback based on error.
pub fn parse_with_recovery(s: &str) -> i32 {
    todo!("Implement parse_with_recovery")
}

/// Divide with fallback to zero on error.
pub fn divide_or_zero(a: i32, b: i32) -> i32 {
    todo!("Implement divide_or_zero")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_recovery() {
        assert_eq!(parse_with_recovery("42"), 42);
        assert_eq!(parse_with_recovery("hello"), 5);
        assert_eq!(parse_with_recovery("abc"), 3);
    }

    #[test]
    fn test_divide_or_zero() {
        assert_eq!(divide_or_zero(10, 2), 5);
        assert_eq!(divide_or_zero(10, 0), 0);
        assert_eq!(divide_or_zero(15, 3), 5);
    }
}
