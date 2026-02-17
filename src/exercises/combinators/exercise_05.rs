//! Exercise 05: Result::map - Transform success values
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Result::map combinator
//! - Transform Ok values while preserving Err
//! - Basic Result transformations

/// Parse a string to i32 and double it.
pub fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    todo!("Implement parse_and_double")
}

/// Parse and convert to absolute value.
pub fn parse_and_abs(s: &str) -> Result<i32, std::num::ParseIntError> {
    todo!("Implement parse_and_abs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert_eq!(parse_and_double("-3"), Ok(-6));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_parse_and_abs() {
        assert_eq!(parse_and_abs("5"), Ok(5));
        assert_eq!(parse_and_abs("-10"), Ok(10));
        assert_eq!(parse_and_abs("0"), Ok(0));
        assert!(parse_and_abs("xyz").is_err());
    }
}
