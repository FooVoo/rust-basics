//! Exercise 10: Error Context - Adding context to errors
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Add context to error messages
//! - Chain error information
//! - Provide helpful debugging information

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    input: String,
    reason: String,
}

impl ParseError {
    pub fn new(input: impl Into<String>, reason: impl Into<String>) -> Self {
        todo!("Implement new")
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Implement fmt")
    }
}

impl std::error::Error for ParseError {}

/// Parse a string to i32 with detailed error context.
pub fn parse_with_context(s: &str) -> Result<i32, ParseError> {
    todo!("Implement parse_with_context")
}

/// Parse multiple strings and collect all errors.
pub fn parse_multiple(strings: &[&str]) -> Result<Vec<i32>, Vec<ParseError>> {
    todo!("Implement parse_multiple")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_context_success() {
        assert_eq!(parse_with_context("42"), Ok(42));
        assert_eq!(parse_with_context("-10"), Ok(-10));
    }

    #[test]
    fn test_parse_with_context_failure() {
        let err = parse_with_context("abc").unwrap_err();
        assert_eq!(err.input, "abc");
        assert!(err.reason.contains("invalid"));
    }

    #[test]
    fn test_parse_with_context_display() {
        let err = parse_with_context("xyz").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("xyz"));
        assert!(msg.contains("Failed to parse"));
    }

    #[test]
    fn test_parse_multiple_all_valid() {
        let input = &["1", "2", "3"];
        assert_eq!(parse_multiple(input), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_multiple_some_invalid() {
        let input = &["1", "abc", "3", "xyz"];
        let result = parse_multiple(input);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].input, "abc");
        assert_eq!(errors[1].input, "xyz");
    }

    #[test]
    fn test_parse_multiple_all_invalid() {
        let input = &["abc", "def"];
        let result = parse_multiple(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().len(), 2);
    }
}
