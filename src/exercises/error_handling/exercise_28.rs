//! Exercise 28: Composite Error Types - Combine multiple error types
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create composite error types
//! - Handle errors from multiple subsystems
//! - Implement unified error handling

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum IoError {
    NotFound(String),
    PermissionDenied(String),
    ConnectionFailed(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IoError::NotFound(path) => write!(f, "File not found: {}", path),
            IoError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            IoError::ConnectionFailed(host) => write!(f, "Connection failed: {}", host),
        }
    }
}

impl std::error::Error for IoError {}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort(usize, usize),
    TooLong(usize, usize),
    InvalidFormat(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::TooShort(actual, min) => {
                write!(f, "Too short: {} (min: {})", actual, min)
            }
            ValidationError::TooLong(actual, max) => {
                write!(f, "Too long: {} (max: {})", actual, max)
            }
            ValidationError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, PartialEq)]
pub enum ApplicationError {
    Io(IoError),
    Validation(ValidationError),
    Parse(ParseIntError),
    Other(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::Io(e) => write!(f, "I/O error: {}", e),
            ApplicationError::Validation(e) => write!(f, "Validation error: {}", e),
            ApplicationError::Parse(e) => write!(f, "Parse error: {}", e),
            ApplicationError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::Io(e) => Some(e),
            ApplicationError::Validation(e) => Some(e),
            ApplicationError::Parse(e) => Some(e),
            ApplicationError::Other(_) => None,
        }
    }
}

impl From<IoError> for ApplicationError {
    fn from(err: IoError) -> Self {
        ApplicationError::Io(err)
    }
}

impl From<ValidationError> for ApplicationError {
    fn from(err: ValidationError) -> Self {
        ApplicationError::Validation(err)
    }
}

impl From<ParseIntError> for ApplicationError {
    fn from(err: ParseIntError) -> Self {
        ApplicationError::Parse(err)
    }
}

/// Validate and parse input.
pub fn process_input(input: &str) -> Result<i32, ApplicationError> {
    if input.len() < 3 {
        return Err(ValidationError::TooShort(input.len(), 3).into());
    }
    
    if input.len() > 10 {
        return Err(ValidationError::TooLong(input.len(), 10).into());
    }
    
    let number: i32 = input.parse()?;
    
    if number < 0 {
        return Err(ApplicationError::Other("Number must be positive".to_string()));
    }
    
    Ok(number)
}

/// Load and process data from a "file".
pub fn load_and_process(path: &str, content: Option<&str>) -> Result<i32, ApplicationError> {
    if path.is_empty() {
        return Err(IoError::NotFound("Empty path".to_string()).into());
    }
    
    if !path.starts_with('/') {
        return Err(IoError::PermissionDenied(path.to_string()).into());
    }
    
    let data = content.ok_or_else(|| IoError::NotFound(path.to_string()))?;
    
    process_input(data)
}

/// Handle multiple operations with different error types.
pub fn batch_process(inputs: &[&str]) -> Result<Vec<i32>, Vec<ApplicationError>> {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for input in inputs {
        match process_input(input) {
            Ok(value) => results.push(value),
            Err(e) => errors.push(e),
        }
    }
    
    if errors.is_empty() {
        Ok(results)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_process_input_success() {
        assert_eq!(process_input("123"), Ok(123));
        assert_eq!(process_input("999"), Ok(999));
    }

    #[test]
    fn test_process_input_too_short() {
        let result = process_input("12");
        assert!(matches!(
            result,
            Err(ApplicationError::Validation(ValidationError::TooShort(_, _)))
        ));
    }

    #[test]
    fn test_process_input_too_long() {
        let result = process_input("12345678901");
        assert!(matches!(
            result,
            Err(ApplicationError::Validation(ValidationError::TooLong(_, _)))
        ));
    }

    #[test]
    fn test_process_input_parse_error() {
        let result = process_input("abc");
        assert!(matches!(result, Err(ApplicationError::Parse(_))));
    }

    #[test]
    fn test_process_input_negative() {
        let result = process_input("-123");
        assert!(matches!(result, Err(ApplicationError::Other(_))));
    }

    #[test]
    fn test_load_and_process_success() {
        let result = load_and_process("/path/to/file", Some("123"));
        assert_eq!(result, Ok(123));
    }

    #[test]
    fn test_load_and_process_not_found() {
        let result = load_and_process("", None);
        assert!(matches!(
            result,
            Err(ApplicationError::Io(IoError::NotFound(_)))
        ));
    }

    #[test]
    fn test_load_and_process_permission_denied() {
        let result = load_and_process("relative/path", Some("123"));
        assert!(matches!(
            result,
            Err(ApplicationError::Io(IoError::PermissionDenied(_)))
        ));
    }

    #[test]
    fn test_load_and_process_file_not_found() {
        let result = load_and_process("/valid/path", None);
        assert!(matches!(
            result,
            Err(ApplicationError::Io(IoError::NotFound(_)))
        ));
    }

    #[test]
    fn test_batch_process_all_success() {
        let inputs = &["123", "456", "789"];
        let result = batch_process(inputs);
        assert_eq!(result, Ok(vec![123, 456, 789]));
    }

    #[test]
    fn test_batch_process_some_errors() {
        let inputs = &["123", "ab", "456"];
        let result = batch_process(inputs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_batch_process_all_errors() {
        let inputs = &["ab", "12", "xyz"];
        let result = batch_process(inputs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn test_error_source_chain() {
        let validation_err = ValidationError::TooShort(1, 3);
        let app_err = ApplicationError::Validation(validation_err);
        assert!(app_err.source().is_some());
    }

    #[test]
    fn test_error_display() {
        let err = ApplicationError::Io(IoError::NotFound("test.txt".to_string()));
        assert!(err.to_string().contains("I/O error"));
        assert!(err.to_string().contains("test.txt"));
    }
}
