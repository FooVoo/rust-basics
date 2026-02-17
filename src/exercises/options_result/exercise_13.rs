//! Exercise 13: The ? operator basics
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use the ? operator for error propagation
//! - Understand early return with ?
//! - Chain multiple ? operations

/// Parse two strings and add them using ?.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError>  {
    todo!("Parse two strings and add them using ?.")
}

/// Parse three strings and multiply them.
pub fn parse_and_multiply(
    a: &str,
    b: &str,
    c: &str,
) -> Result<i32, std::num::ParseIntError>  {
    todo!("Parse three strings and multiply them.")
}

/// Divide two parsed numbers.
pub fn parse_and_divide(a: &str, b: &str) -> Result<i32, String>  {
    todo!("Divide two parsed numbers.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_add() {
        assert_eq!(parse_and_add("5", "10"), Ok(15));
        assert_eq!(parse_and_add("0", "0"), Ok(0));
        assert!(parse_and_add("abc", "10").is_err());
        assert!(parse_and_add("5", "xyz").is_err());
    }

    #[test]
    fn test_parse_and_multiply() {
        assert_eq!(parse_and_multiply("2", "3", "4"), Ok(24));
        assert_eq!(parse_and_multiply("0", "5", "10"), Ok(0));
        assert!(parse_and_multiply("a", "2", "3").is_err());
    }

    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("10", "2"), Ok(5));
        assert!(parse_and_divide("10", "0").is_err());
        assert!(parse_and_divide("abc", "2").is_err());
    }
}
