//! Exercise 14: Password Validation - Security validation
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement security-focused validation
//! - Check multiple conditions
//! - Return detailed validation errors

#[derive(Debug, PartialEq)]
pub enum PasswordError {
    TooShort,
    TooLong,
    NoUppercase,
    NoLowercase,
    NoDigit,
    NoSpecialChar,
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result  {
        todo!("Implement fmt")
    }
}

/// Validate a password against security requirements.
/// Must be 8-128 chars, contain upper, lower, digit, and special char.
pub fn validate_password(password: &str) -> Result<(), Vec<PasswordError>>  {
    todo!("Must be 8-128 chars, contain upper, lower, digit, and special char.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_valid() {
        assert_eq!(validate_password("Password123!"), Ok(()));
        assert_eq!(validate_password("Secure@Pass1"), Ok(()));
        assert_eq!(validate_password("MyP@ssw0rd"), Ok(()));
    }

    #[test]
    fn test_validate_password_too_short() {
        let result = validate_password("Pass1!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&PasswordError::TooShort));
    }

    #[test]
    fn test_validate_password_no_uppercase() {
        let result = validate_password("password123!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&PasswordError::NoUppercase));
    }

    #[test]
    fn test_validate_password_no_lowercase() {
        let result = validate_password("PASSWORD123!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&PasswordError::NoLowercase));
    }

    #[test]
    fn test_validate_password_no_digit() {
        let result = validate_password("Password!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&PasswordError::NoDigit));
    }

    #[test]
    fn test_validate_password_no_special() {
        let result = validate_password("Password123");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&PasswordError::NoSpecialChar));
    }

    #[test]
    fn test_validate_password_multiple_errors() {
        let result = validate_password("pass");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2);
    }

    #[test]
    fn test_validate_password_too_long() {
        let long_password = "A".repeat(129) + "a1!";
        let result = validate_password(&long_password);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&PasswordError::TooLong));
    }
}
