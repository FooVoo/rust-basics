//! Exercise 24: Custom error handling patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement custom error handling patterns
//! - Create reusable error handling utilities
//! - Design error recovery strategies

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ValidationError {
    field: String,
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        todo!("Implement fmt")
    }
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self  {
        todo!("Implement new")
    }
}

/// Validate a username (non-empty, alphanumeric).
pub fn validate_username(username: &str) -> Result<String, ValidationError>  {
    todo!("Validate a username (non-empty, alphanumeric).")
}

/// Validate an age (positive, reasonable range).
pub fn validate_age(age: i32) -> Result<i32, ValidationError>  {
    todo!("Validate an age (positive, reasonable range).")
}

/// Combine multiple validations.
pub fn validate_user(
    username: &str,
    age: i32,
) -> Result<(String, i32), ValidationError>  {
    todo!("Combine multiple validations.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert_eq!(validate_username("alice123"), Ok(String::from("alice123")));
        assert!(validate_username("").is_err());
        assert!(validate_username("alice@123").is_err());
    }

    #[test]
    fn test_validate_age() {
        assert_eq!(validate_age(25), Ok(25));
        assert!(validate_age(0).is_err());
        assert!(validate_age(-5).is_err());
        assert!(validate_age(200).is_err());
    }

    #[test]
    fn test_validate_user() {
        assert_eq!(
            validate_user("alice", 25),
            Ok((String::from("alice"), 25))
        );
        assert!(validate_user("", 25).is_err());
        assert!(validate_user("alice", 0).is_err());
    }
}
