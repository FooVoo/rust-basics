//! Exercise 02: Simple Validation - Check age validity
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Validate input ranges
//! - Return descriptive error messages
//! - Use conditional error handling

/// Validate an age (must be between 0 and 150).
/// Return Ok(age) if valid, Err with message otherwise.
pub fn validate_age(age: i32) -> Result<i32, String> {
    todo!("Implement validate_age")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_age_valid() {
        assert_eq!(validate_age(25), Ok(25));
        assert_eq!(validate_age(0), Ok(0));
        assert_eq!(validate_age(150), Ok(150));
        assert_eq!(validate_age(1), Ok(1));
    }

    #[test]
    fn test_validate_age_invalid() {
        assert!(validate_age(-1).is_err());
        assert!(validate_age(151).is_err());
        assert!(validate_age(-100).is_err());
        assert!(validate_age(200).is_err());
    }
}
