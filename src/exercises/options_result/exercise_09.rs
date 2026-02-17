//! Exercise 09: Basic Result - Ok and Err
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand Result<T, E> type
//! - Create Ok and Err variants
//! - Handle success and error cases

/// Divide two numbers, returning an error message if dividing by zero.
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    todo!("Implement divide")
}

/// Parse a string to i32, converting parse error to custom message.
pub fn parse_number(s: &str) -> Result<i32, String> {
    todo!("Implement parse_number")
}

/// Check if a number is positive, return error otherwise.
pub fn validate_positive(n: i32) -> Result<i32, String> {
    todo!("Implement validate_positive")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(7, 3), Ok(2));
        assert_eq!(divide(10, 0), Err(String::from("Division by zero")));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert!(parse_number("abc").is_err());
        assert!(parse_number("").is_err());
    }

    #[test]
    fn test_validate_positive() {
        assert_eq!(validate_positive(5), Ok(5));
        assert!(validate_positive(0).is_err());
        assert!(validate_positive(-5).is_err());
    }
}
