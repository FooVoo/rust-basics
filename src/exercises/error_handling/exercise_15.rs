//! Exercise 15: Error Conversion - Converting between error types
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement From trait for error conversion
//! - Use ? operator with different error types
//! - Create unified error types

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    ParseError(String),
    RangeError(String),
}

impl From<ParseIntError> for ConversionError {
    fn from(err: ParseIntError) -> Self {
        ConversionError::ParseError(err.to_string())
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConversionError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConversionError::RangeError(msg) => write!(f, "Range error: {}", msg),
        }
    }
}

impl std::error::Error for ConversionError {}

/// Parse a string to u8, checking it's in valid range.
pub fn parse_to_u8(s: &str) -> Result<u8, ConversionError> {
    let num: i32 = s.parse()?;
    
    if num < 0 || num > 255 {
        Err(ConversionError::RangeError(format!(
            "{} is out of range for u8 (0-255)",
            num
        )))
    } else {
        Ok(num as u8)
    }
}

/// Parse multiple strings to u8 values.
pub fn parse_many_u8(strings: &[&str]) -> Result<Vec<u8>, ConversionError> {
    strings.iter().map(|s| parse_to_u8(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_u8_valid() {
        assert_eq!(parse_to_u8("0"), Ok(0));
        assert_eq!(parse_to_u8("127"), Ok(127));
        assert_eq!(parse_to_u8("255"), Ok(255));
    }

    #[test]
    fn test_parse_to_u8_parse_error() {
        let result = parse_to_u8("abc");
        assert!(matches!(result, Err(ConversionError::ParseError(_))));
    }

    #[test]
    fn test_parse_to_u8_range_error_negative() {
        let result = parse_to_u8("-1");
        assert!(matches!(result, Err(ConversionError::RangeError(_))));
    }

    #[test]
    fn test_parse_to_u8_range_error_too_large() {
        let result = parse_to_u8("256");
        assert!(matches!(result, Err(ConversionError::RangeError(_))));
        
        let result = parse_to_u8("1000");
        assert!(matches!(result, Err(ConversionError::RangeError(_))));
    }

    #[test]
    fn test_parse_many_u8_all_valid() {
        let input = &["1", "2", "3", "255"];
        assert_eq!(parse_many_u8(input), Ok(vec![1, 2, 3, 255]));
    }

    #[test]
    fn test_parse_many_u8_with_error() {
        let input = &["1", "abc", "3"];
        assert!(parse_many_u8(input).is_err());
        
        let input = &["1", "256", "3"];
        assert!(parse_many_u8(input).is_err());
    }

    #[test]
    fn test_error_display() {
        let err = ConversionError::ParseError("test".to_string());
        assert!(err.to_string().contains("Parse error"));
        
        let err = ConversionError::RangeError("out of bounds".to_string());
        assert!(err.to_string().contains("Range error"));
    }
}
