//! Exercise 11: Result unwrap_or and unwrap_or_else
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use unwrap_or() with Result
//! - Use unwrap_or_else() for lazy defaults
//! - Recover from errors with defaults

/// Parse a number or return a default value.
pub fn parse_or_default(s: &str, default: i32) -> i32 {
    s.parse::<i32>().unwrap_or(default)
}

/// Parse a number or compute a default from the error.
pub fn parse_or_length(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or_else(|_| s.len() as i32)
}

/// Divide or return zero on error.
pub fn divide_or_zero(a: i32, b: i32) -> i32 {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42", 0), 42);
        assert_eq!(parse_or_default("abc", 0), 0);
        assert_eq!(parse_or_default("", -1), -1);
    }

    #[test]
    fn test_parse_or_length() {
        assert_eq!(parse_or_length("42"), 42);
        assert_eq!(parse_or_length("hello"), 5); // length of "hello"
        assert_eq!(parse_or_length("abc"), 3);
    }

    #[test]
    fn test_divide_or_zero() {
        assert_eq!(divide_or_zero(10, 2), 5);
        assert_eq!(divide_or_zero(10, 0), 0);
        assert_eq!(divide_or_zero(7, 3), 2);
    }
}
